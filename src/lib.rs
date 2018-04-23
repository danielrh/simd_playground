#![feature(stdsimd)]
#![feature(test)]

extern crate test;

mod tests {
#[allow(unused_imports)]
use test::{Bencher, black_box};
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


#[test]
fn simple() {
use std::simd::*;
   let symbol = 2i16;
   let inc = 1i16;
   let data = i16x16::new(4, 8, 12, 16, 20, 24, 28, 32, 36, 40, 44, 48, 52, 56, 60, 64);
   let one_to_16 = i16x16::new(1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16);
   let increment_v = i16x16::splat(inc);
   let mask_v = one_to_16.gt(i16x16::splat(i16::from(symbol)));
    let output = data + (increment_v & i16x16::from_bits(mask_v));
    let mut xfinal = [0i16; 16];
    output.store_unaligned(&mut xfinal);
    assert_eq!(xfinal, [4, 8, 13, 17, 21, 25, 29, 33, 37, 41, 45, 49, 53, 57, 61, 65]);
}

#[test]
fn sub_baseline() {
use std::simd::*;
   let symbol = 2i16;
   let inc = 1i16;
   let data = i16x16::new(4, 8, 12, 16, 20, 24, 28, 32, 36, 40, 44, 48, 52, 56, 60, 64);
   let one_to_16 = i16x16::new(1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16);
   let increment_v = i16x16::splat(inc);
   let wide_symbol = i16x16::splat(i16::from(symbol));
   let sign_bit = (wide_symbol - one_to_16) & i16x16::splat(-32768);
   let mask_v = sign_bit >> 15;
   
    let output = data + (increment_v & i16x16::from_bits(mask_v));
    let mut xfinal = [0i16; 16];
    output.store_unaligned(&mut xfinal);
    assert_eq!(xfinal, [4, 8, 13, 17, 21, 25, 29, 33, 37, 41, 45, 49, 53, 57, 61, 65]);
}

#[bench]
fn baseline_bench(b: &mut Bencher) {
use std::simd::*;
   let symbol = black_box(3i16);
   let inc = black_box(1i16);
   let data = black_box(i16x16::new(4, 8, 12, 16, 20, 24, 28, 32, 36, 40, 44, 48, 52, 56, 60, 64));
   b.iter(|| {
   for _i in {1..100} {
   let one_to_16 = i16x16::new(1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16);
   let increment_v = i16x16::splat(inc);
   let mask_v = unsafe {
            ::std::arch::x86_64::_mm256_cmpgt_epi16(::std::arch::x86_64::__m256i::from_bits(one_to_16),
                                                   ::std::arch::x86_64::__m256i::from_bits(i16x16::splat(i16::from(symbol))))
    };
    let output = data + (increment_v & i16x16::from_bits(mask_v));
    let mut xfinal = [0i16; 16];
    output.store_unaligned(&mut xfinal);
    assert_eq!(xfinal, [4, 8, 12, 17, 21, 25, 29, 33, 37, 41, 45, 49, 53, 57, 61, 65]);
    }
    });
    }
#[bench]
fn simple_bench(b: &mut Bencher) {
use std::simd::*;
   let symbol = black_box(3i16);
   let inc = black_box(1i16);
   let data = black_box(i16x16::new(4, 8, 12, 16, 20, 24, 28, 32, 36, 40, 44, 48, 52, 56, 60, 64));
   b.iter(|| {
   for _i in {1..100} {
   let one_to_16 = i16x16::new(1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16);
   let increment_v = i16x16::splat(inc);
   let mask_v = one_to_16.gt(i16x16::splat(i16::from(symbol)));
    let output = data + (increment_v & i16x16::from_bits(mask_v));
    let mut xfinal = [0i16; 16];
    output.store_unaligned(&mut xfinal);
    assert_eq!(xfinal, [4, 8, 12, 17, 21, 25, 29, 33, 37, 41, 45, 49, 53, 57, 61, 65]);
    }
    });
    }
#[bench]
fn sub_baseline_bench(b: &mut Bencher) {
use std::simd::*;
   let symbol = black_box(3i16);
   let inc = black_box(1i16);
   let data = black_box(i16x16::new(4, 8, 12, 16, 20, 24, 28, 32, 36, 40, 44, 48, 52, 56, 60, 64));
   b.iter(|| {
      for _i in {1..100} {
   let one_to_16 = i16x16::new(1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16);
   let increment_v = i16x16::splat(inc);
   let wide_symbol = i16x16::splat(i16::from(symbol));
   let sign_bit = (wide_symbol - one_to_16) & i16x16::splat(-128);
   let mask_v = sign_bit >> 7;
   
    let output = data + (increment_v & i16x16::from_bits(mask_v));
    let mut xfinal = [0i16; 16];
    output.store_unaligned(&mut xfinal);
    assert_eq!(xfinal, [4, 8, 12, 17, 21, 25, 29, 33, 37, 41, 45, 49, 53, 57, 61, 65]);
    }
    });
}
use std::simd::i16x16;

#[allow(unused)]
#[inline(always)]
fn cmp_gt_i16x16(lhs: i16x16, rhs: i16x16) -> i16x16 {
    let lz = rhs - lhs;
    let sign_bit = lz & i16x16::splat(-32768);
    sign_bit >> 15
}
#[bench]
fn cmp_less_baseline_bench(b: &mut Bencher) {
use std::simd::*;
   let symbol = black_box(3i16);
   let inc = black_box(1i16);
   let data = black_box(i16x16::new(4, 8, 12, 16, 20, 24, 28, 32, 36, 40, 44, 48, 52, 56, 60, 64));
   b.iter(|| {
      for _i in {1..100} {
   let one_to_16 = i16x16::new(1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16);
   let increment_v = i16x16::splat(inc);
   let mask_v = cmp_gt_i16x16(one_to_16, i16x16::splat(i16::from(symbol)));
   
    let output = data + (increment_v & mask_v);
    let mut xfinal = [0i16; 16];
    output.store_unaligned(&mut xfinal);
    assert_eq!(xfinal, [4, 8, 12, 17, 21, 25, 29, 33, 37, 41, 45, 49, 53, 57, 61, 65]);
    }
    });
}
#[test]
fn cmp_less_baseline_test() {
use std::simd::*;
   let symbol = black_box(3i16);
   let inc = black_box(1i16);
   let data = black_box(i16x16::new(4, 8, 12, 16, 20, 24, 28, 32, 36, 40, 44, 48, 52, 56, 60, 64));
   let one_to_16 = i16x16::new(1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16);
   let increment_v = i16x16::splat(inc);
   let mask_v = cmp_gt_i16x16(one_to_16, i16x16::splat(i16::from(symbol)));
   
    let output = data + (increment_v & mask_v);
    let mut xfinal = [0i16; 16];
    output.store_unaligned(&mut xfinal);
    assert_eq!(xfinal, [4, 8, 12, 17, 21, 25, 29, 33, 37, 41, 45, 49, 53, 57, 61, 65]);
}
}
