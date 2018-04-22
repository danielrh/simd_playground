#![feature(stdsimd)]
mod test {
#[test]
fn baseline() {
use std::simd::*;
   let symbol = 2i16;
   let inc = 1i16;
   let data = i16x16::new(4, 8, 12, 16, 20, 24, 28, 32, 36, 40, 44, 48, 52, 56, 60, 64);
   let one_to_16 = i16x16::new(1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16);
   let increment_v = i16x16::splat(inc);
   let mask_v = unsafe {
            ::std::arch::x86_64::_mm256_cmpgt_epi16(::std::arch::x86_64::__m256i::from_bits(one_to_16),
                                                   ::std::arch::x86_64::__m256i::from_bits(i16x16::splat(i16::from(symbol))))
    };
    let output = data + (increment_v & i16x16::from_bits(mask_v));
    let mut xfinal = [0i16; 16];
    output.store_unaligned(&mut xfinal);
    assert_eq!(xfinal, [4, 8, 13, 17, 21, 25, 29, 33, 37, 41, 45, 49, 53, 57, 61, 65]);
}
}