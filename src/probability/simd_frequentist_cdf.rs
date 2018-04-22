use core;
use core::simd::FromBits;
use super::interface::{Prob, BaseCDF, Speed, CDF16, BLEND_FIXED_POINT_PRECISION, SymStartFreq, LOG2_SCALE};
use super::numeric;
use core::simd;
use core::simd::{i16x16, i64x4, i16x8, i8x32, i8x16, u32x8, u8x16, i64x2, i32x8};
//use stdsimd::vendor::__m256i;

#[derive(Clone,Copy)]
pub struct SIMDFrequentistCDF16 {
    pub cdf: i16x16,
    pub inv_max: (i64, u8),
}
    fn epvec(data: i16x16) {
       let mut slice = [0i16; 16];
       data.store_unaligned(&mut slice);
       eprint!("{:?}\n", slice);
    }

impl SIMDFrequentistCDF16 {
    #[inline(always)]
    fn new(input: i16x16) -> Self {
        let mut ret = SIMDFrequentistCDF16 {
            cdf: input,
            inv_max: (0, 0),
        };
        ret.inv_max = numeric::lookup_divisor(ret.max());
        ret
    }
    fn eprin(&self) {
       let mut slice = [0i16; 16];
       self.cdf.store_unaligned(&mut slice);
       eprint!("{:?}\n", slice);
    }
}

impl Default for SIMDFrequentistCDF16 {
    #[inline(always)]
    fn default() -> Self {
        SIMDFrequentistCDF16::new(i16x16::new(4, 8, 12, 16, 20, 24, 28, 32, 36, 40, 44, 48, 52, 56, 60, 64))
    }
}

extern "platform-intrinsic" {
    pub fn simd_shuffle4<T, U>(x: T, y: T, idx: [u32; 4]) -> U;
    pub fn simd_shuffle16<T, U>(x: T, y: T, idx: [u32; 16]) -> U;
}

impl BaseCDF for SIMDFrequentistCDF16 {
    #[inline(always)]
    fn num_symbols() -> u8 { 16 }
    #[inline(always)]
    fn used(&self) -> bool { self.entropy() != Self::default().entropy() }
    #[inline(always)]
    fn max(&self) -> Prob { self.cdf.extract(15) }
    #[inline(always)]
    fn div_by_max(&self, val:i32) -> i32 { numeric::fast_divide_30bit_by_16bit(val, self.inv_max) }
    #[inline(always)]
    fn log_max(&self) -> Option<i8> { None }
    #[inline(always)]
    fn cdf(&self, symbol: u8) -> Prob {
        // bypass the internal assert by hinting to the compiler that symbol is 4-bit.
        self.cdf.extract(symbol as usize & 0xf)
    }
    fn valid(&self) -> bool {
        let mut slice = [0i16; 16];
        self.cdf.store_unaligned(&mut slice);
        for it in slice[0..15].iter().zip(slice[1..16].iter()) {
            let (prev, next) = it;
            if (*next <= *prev) {
                return false;
            }
        }
        self.inv_max == numeric::lookup_divisor(self.max())
    }
    /* //slower
    fn sym_to_start_and_freq(&self,
                             sym: u8) -> SymStartFreq {
        let prev_cur = i64x2::new(if sym != 0 {self.cdf(sym - 1) as u64 as i64} else {0},
                                  self.cdf(sym) as u64 as i64);
        let scaled_prev_cur = prev_cur << LOG2_SCALE;
        let prev_cur_over_max = numeric::fast_divide_30bit_i64x2_by_16bit(scaled_prev_cur, self.inv_max);
        let cdf_prev = prev_cur_over_max.extract(0);
        let freq = prev_cur_over_max.extract(1) - cdf_prev;
        SymStartFreq {
            start: cdf_prev as Prob + 1, // major hax
            freq:  freq as Prob - 1, // don't want rounding errors to work out unfavorably
            sym: sym,
        }
}*/
    #[cfg(feature="avx2")]
    #[inline(always)]
    fn cdf_offset_to_sym_start_and_freq(&self,
                                        cdf_offset_p: Prob) -> SymStartFreq {
        let rescaled_cdf_offset = ((i32::from(cdf_offset_p) * i32::from(self.max())) >> LOG2_SCALE) as i16;
        let symbol_less = i16x16::splat(rescaled_cdf_offset).gt(self.cdf - i16x16::splat(1));
        let bitmask = unsafe { core::arch::x86_64::_mm256_movemask_epi8(i8x32::from(symbol_less)) };
        let symbol_id = ((32 - (bitmask as u32).leading_zeros()) >> 1) as u8;
        self.sym_to_start_and_freq(symbol_id)
    }
    #[cfg(not(feature="avx2"))]
    #[inline(always)]
    fn cdf_offset_to_sym_start_and_freq(&self,
                                        cdf_offset_p: Prob) -> SymStartFreq {
        let rescaled_cdf_offset = ((i32::from(cdf_offset_p) * i32::from(self.max())) >> LOG2_SCALE) as i16;
        let symbol_less = i16x16::splat(rescaled_cdf_offset).gt(self.cdf - i16x16::splat(1));
        let tmp: i8x16 = unsafe { simd_shuffle16(i8x32::from_bits(symbol_less), i8x32::splat(0),
                                                 [0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30]) };
        let bitmask = unsafe { core::arch::x86_64::_mm_movemask_epi8(core::arch::x86_64::__m128i::from_bits(tmp)) };
        let symbol_id = (32 - (bitmask as u32).leading_zeros()) as u8;
        self.sym_to_start_and_freq(symbol_id)
    }
}

#[inline(always)]
fn i16x16_to_i64x4_tuple(input: i16x16) -> (i64x4, i64x4, i64x4, i64x4) {
    let zero = i16x16::splat(0);
    unsafe {
        let widened_q0: i16x16 = simd_shuffle16(
            input, zero, [0, 16, 16, 16, 1, 16, 16, 16, 2, 16, 16, 16, 3, 16, 16, 16]);
        let widened_q1: i16x16 = simd_shuffle16(
            input, zero, [4, 16, 16, 16, 5, 16, 16, 16, 6, 16, 16, 16, 7, 16, 16, 16]);
        let widened_q2: i16x16 = simd_shuffle16(
            input, zero, [8, 16, 16, 16, 9, 16, 16, 16, 10, 16, 16, 16, 11, 16, 16, 16]);
        let widened_q3: i16x16 = simd_shuffle16(
            input, zero, [12, 16, 16, 16, 13, 16, 16, 16, 14, 16, 16, 16, 15, 16, 16, 16]);
        (i64x4::from_bits(widened_q0), i64x4::from_bits(widened_q1), i64x4::from_bits(widened_q2), i64x4::from_bits(widened_q3))
    }
}

#[inline(always)]
fn i64x4_tuple_to_i16x16(input0: i64x4, input1: i64x4, input2: i64x4, input3: i64x4) -> i16x16 {
    unsafe {
        let input01: i16x16 = simd_shuffle16(i16x16::from_bits(input0), i16x16::from_bits(input1),
                                             [0, 4, 8, 12, 16, 20, 24, 28, 0, 0, 0, 0, 0, 0, 0, 0]);
        let input23: i16x16 = simd_shuffle16(i16x16::from_bits(input2), i16x16::from_bits(input3),
                                             [0, 4, 8, 12, 16, 20, 24, 28, 0, 0, 0, 0, 0, 0, 0, 0]);
        let output: i64x4 = simd_shuffle4(i64x4::from_bits(input01), i64x4::from_bits(input23), [0, 1, 4, 5]);
        i16x16::from_bits(output)
    }
}

#[inline(always)]
fn i16x16_to_i32x8_tuple(input: i16x16) -> (i32x8, i32x8) {
    let zero = i16x16::splat(0);
    unsafe {
        let widened_lo: i16x16 = simd_shuffle16(
            input, zero, [0, 16, 1, 16, 2, 16, 3, 16, 4, 16, 5, 16, 6, 16, 7, 16]);
        let widened_hi: i16x16 = simd_shuffle16(
            input, zero, [8, 16, 9, 16, 10, 16, 11, 16, 12, 16, 13, 16, 14, 16, 15, 16]);
        (i32x8::from_bits(widened_lo), i32x8::from_bits(widened_hi))
    }
}

#[inline(always)]
fn i32x8_tuple_to_i16x16(input0: i32x8, input1: i32x8) -> i16x16 {
    unsafe {
        simd_shuffle16(i16x16::from_bits(input0), i16x16::from_bits(input1),
                       [0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30])
    }
}


impl CDF16 for SIMDFrequentistCDF16 {
    #[inline(always)]
    fn average(&self, other:&Self, mix_rate:i32) -> Self {

        let ourmax = i32::from(self.max());
        let othermax = i32::from(other.max());
        let ourmax_times_othermax = ourmax * othermax;
        let leading_zeros_combo = core::cmp::min(ourmax_times_othermax.leading_zeros(), 17);
        let desired_shift = 17 - leading_zeros_combo;

        let inv_mix_rate = (1 << BLEND_FIXED_POINT_PRECISION) - mix_rate;
        let mix_rate_v = i32x8::splat(mix_rate);
        let inv_mix_rate_v = i32x8::splat(inv_mix_rate);
        let our_max_v = i32x8::splat(ourmax);
        let other_max_v = i32x8::splat(othermax);
        let one = i32x8::splat(1);
        let (self0, self1) = i16x16_to_i32x8_tuple(self.cdf);
        let (other0, other1) = i16x16_to_i32x8_tuple(other.cdf);
        let rescaled_self0 = (self0 * other_max_v) >> desired_shift; // now we know we have at least 15 bits remaining in our space
        let rescaled_self1 = (self1 * other_max_v) >> desired_shift;
        let rescaled_other0 = (other0 * our_max_v) >> desired_shift;
        let rescaled_other1 = (other1 * our_max_v) >> desired_shift;

        let ret0 = (rescaled_self0 * mix_rate_v + rescaled_other0 * inv_mix_rate_v + one) >> (BLEND_FIXED_POINT_PRECISION as i8);
        let ret1 = (rescaled_self1 * mix_rate_v + rescaled_other1 * inv_mix_rate_v + one) >> (BLEND_FIXED_POINT_PRECISION as i8);
        SIMDFrequentistCDF16::new(i32x8_tuple_to_i16x16(ret0, ret1))
    }
    #[inline(always)]
    fn blend(&mut self, symbol: u8, speed: Speed) {
        let increment_v = i16x16::splat(speed.inc());
        let one_to_16 = i16x16::new(1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16);
        let mask_v = unsafe {
            core::arch::x86_64::_mm256_cmpgt_epi16(core::arch::x86_64::__m256i::from_bits(one_to_16),
                                                   core::arch::x86_64::__m256i::from_bits(i16x16::splat(i16::from(symbol))))
        };
        self.eprin();
        epvec(i16x16::from_bits(mask_v));
        epvec(increment_v);
        epvec(increment_v & i16x16::from_bits(mask_v));
        self.cdf = self.cdf + (increment_v & i16x16::from_bits(mask_v));
        eprint!("Ok added the mask\n");
        self.eprin();
        let mut cdf_max = self.max();
        if cdf_max >= speed.lim() {
            let cdf_bias = one_to_16;
            self.cdf = self.cdf + cdf_bias - ((self.cdf + cdf_bias) >> 2);
            cdf_max = self.max();
        }
        self.inv_max = numeric::lookup_divisor(cdf_max);
    }
}

//__mmask16 _mm256_cmpge_epi16_mask (__m256i a, __m256i b)

#[cfg(test)]
mod test {
    use super::{i16x16, i32x8, i64x4};
    use super::{i16x16_to_i32x8_tuple, i16x16_to_i64x4_tuple,
                i32x8_tuple_to_i16x16, i64x4_tuple_to_i16x16};
    use super::SIMDFrequentistCDF16;


    #[test]
    fn test_cdf_simd_eq_opt() {
        use super::super::{common_tests, OptFrequentistCDF16};
        common_tests::operation_test_helper(&mut SIMDFrequentistCDF16::default(),
                                            &mut SIMDFrequentistCDF16::default(),
                                            &mut OptFrequentistCDF16::default(),
                                            &mut OptFrequentistCDF16::default());
    }
}
