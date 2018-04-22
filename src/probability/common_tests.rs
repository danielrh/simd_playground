use super::{BLEND_FIXED_POINT_PRECISION, CDF16, LOG2_SCALE, Prob, ProbRange, Speed};


pub fn assert_cdf_eq<CDF16A: CDF16, CDF16B: CDF16>(cdf0: &CDF16A, cdf1: &CDF16B) {
    eprint!("Checking {} vs {}\n", cdf0.max(), cdf1.max());
    assert_eq!(cdf0.max(), cdf1.max());
    for sym in 0..16 {
        eprint!("{}) Checking {} vs {}\n", sym, cdf0.cdf(sym as u8), cdf1.cdf(sym as u8));
        assert_eq!(cdf0.cdf(sym as u8), cdf1.cdf(sym as u8));
    }
    assert!(cdf0.valid());
    assert!(cdf1.valid());
}

pub fn assert_cdf_similar<CDF16A: CDF16, CDF16B: CDF16>(cdf0: &CDF16A, cdf1: &CDF16B) {
    let max0 = cdf0.max() as i64;
    let max1 = cdf1.max() as i64;
    for sym in 0..16 {
        let sym0cdf = i64::from(cdf0.cdf(sym as u8));
        let sym1cdf = i64::from(cdf1.cdf(sym as u8));
        let cmp0 = sym0cdf * max1;
        let cmp1 = sym1cdf * max0;
        let delta = if cmp0 < cmp1 { cmp1.wrapping_sub(cmp0) } else { cmp0.wrapping_sub(cmp1) };
        assert!(delta < max1 * max0 / 160);
    }
    assert!(cdf0.valid());
    assert!(cdf1.valid());
}

pub fn operation_test_helper<CDFA: CDF16, CDFB: CDF16> (cdf0a: &mut CDFA, cdf1a: &mut CDFA, cdf0b: &mut CDFB, cdf1b: &mut CDFB) {
    assert_cdf_eq(cdf0a, cdf0b);
    assert_cdf_eq(cdf1a, cdf1b);
    let symbol_buffer0 = [0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8,
                          6u8, 7u8, 8u8, 8u8, 9u8, 9u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8,
                          10u8, 10u8, 10u8, 11u8, 12u8, 12u8, 12u8, 13u8, 13u8, 13u8, 14u8, 15u8,
                          15u8, 15u8, 15u8, 15u8, 15u8, 15u8];
    let symbol_buffer1 = [0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 5u8, 5u8, 5u8, 5u8, 5u8];
    for sym in symbol_buffer0.iter() {
        cdf0a.blend(*sym, Speed::MED);
        cdf0b.blend(*sym, Speed::MED);
        assert_cdf_eq(cdf0a, cdf0b);
    }
    assert_cdf_similar(&cdf0a.average(cdf1a, (1<<BLEND_FIXED_POINT_PRECISION)>>2), &cdf0b.average(cdf1b, (1<<BLEND_FIXED_POINT_PRECISION)>>2));
    for sym in symbol_buffer1.iter() {
        cdf0a.blend(*sym, Speed::MED);
        cdf0b.blend(*sym, Speed::MED);
        assert_cdf_eq(cdf0a, cdf0b);
    }
    let all = (1<<BLEND_FIXED_POINT_PRECISION);
    let half = (1<<BLEND_FIXED_POINT_PRECISION)>>1;
    let quarter = (1<<BLEND_FIXED_POINT_PRECISION)>>2;
    let threequarters = half + quarter;;

    assert_cdf_eq(&cdf0a.average(cdf1a, quarter), &cdf0b.average(cdf1b, quarter));
    assert_cdf_eq(&cdf0a.average(cdf1a, half), &cdf0b.average(cdf1b, half));
    assert_cdf_eq(&cdf0a.average(cdf1a, threequarters), &cdf0b.average(cdf1b, threequarters));
    assert_cdf_eq(&cdf0a.average(cdf1a, 0), &cdf0b.average(cdf1b, 0));
    assert_cdf_eq(&cdf0a.average(cdf1a, all), &cdf0b.average(cdf1b, all));
    assert_cdf_similar(&cdf0a.average(cdf1a, 0), cdf1a);
    assert_cdf_similar(&cdf0a.average(cdf1a, all), cdf0a);
    assert_cdf_similar(&cdf0b.average(cdf1b, 0), cdf1b);
    assert_cdf_similar(&cdf0b.average(cdf1b, all), cdf0b);
}

mod test {
use core;
use std::simd::*;
#[test]
fn baseline() {
   let symbol = 2i16;
   let inc = 1i16;
   let data = i16x16::new(4, 8, 12, 16, 20, 24, 28, 32, 36, 40, 44, 48, 52, 56, 60, 64);
   let one_to_16 = i16x16::new(1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16);
   let increment_v = i16x16::splat(inc);
   let mask_v = unsafe {
            core::arch::x86_64::_mm256_cmpgt_epi16(core::arch::x86_64::__m256i::from_bits(one_to_16),
                                                   core::arch::x86_64::__m256i::from_bits(i16x16::splat(i16::from(symbol))))
    };
    let output = data + (increment_v & i16x16::from_bits(mask_v));
    let mut xfinal = [0i16; 16];
    output.store_unaligned(&mut xfinal);
    assert_eq!(xfinal, [4, 8, 13, 17, 21, 25, 29, 33, 37, 41, 45, 49, 53, 57, 61, 65]);
}
}