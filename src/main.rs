extern crate stdsimd;
use stdsimd::simd::{i16x16, i64x4, i16x8, u8x16, i8x16};

use stdsimd::vendor::__m256i;

fn main() {
    let cdf = i16x16::new(100,101,102,103,104,105,106,107,108,109,110,111,112,113,114,115);
    
    //let shuf = i64x4::new(0,1,2,3);
    //let _upper = unsafe{stdsimd::vendor::_mm256_permutevar8x32_epi32(shuf, shuf)};
    //let upper = unsafe{stdsimd::vendor::_mm256_permute4x64_epi64(shuf, 0xee)};
    let upper_addresses = u8x16::new(8, 9, 10, 11, 12, 13, 14, 15, 8, 9, 10, 11, 12, 13, 14, 15);
    // FIXME this is missing let upper_quad = stdsimd::vendor::_mm256_extracti128_si256(__m256i::from(cdf), 1);
    let upper_quad_replicated = unsafe{stdsimd::vendor::_mm256_permute4x64_epi64(i64x4::from(cdf),
                                                                                 0xee)};
    let upper_quad = unsafe{stdsimd::vendor::_mm256_castsi256_si128(__m256i::from(upper_quad_replicated))};
    //let self0 = unsafe{stdsimd::vendor::_mm256_cvtepi16_epi64(i16x8::from(stdsimd::vendor::_mm256_castsi256_si128(__m256i::from(cdf))))};
    //let self1 = unsafe{stdsimd::vendor::_mm256_cvtepi16_epi64(i16x8::from(stdsimd::vendor::_mm_shuffle_epi8(stdsimd::vendor::_mm256_castsi256_si128(__m256i::from(cdf)).as_u8x16(), upper_addresses)))};
//    let self2 = unsafe{stdsimd::vendor::_mm256_cvtepi16_epi64(i16x8::from(upper_quad))};
    //    let self3 = unsafe{stdsimd::vendor::_mm256_cvtepi16_epi64(i16x8::from(stdsimd::vendor::_mm_shuffle_epi8(upper_quad.as_u8x16(), upper_addresses)))};
    //println!("{:?} => {:?} {:?} {:?} {:?}", cdf, self0, self1, self2, self3);
    //let x = unsafe{stdsimd::vendor::_mm_shuffle_epi8(u8x16::new(0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15),
    //u8x16::new(5,0,5,4,9,13,7,4,13,6,6,11,5,2,9,1))};
    //let x = unsafe{stdsimd::vendor::_mm_alignr_epi8(i8x16::new(0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15),
    //i8x16::new(0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15),
    //8)};
    //println!("{:?} => {:?}",cdf, x);
    let upper_addresses = u8x16::new(8, 9, 10, 11, 12, 13, 14, 15, 8, 9, 10, 11, 12, 13, 14, 15);
    // FIXME this is missing let upper_quad = stdsimd::vendor::_mm256_extracti128_si256(__m256i::from(self.cdf), 1);
    let upper_quad_replicated = unsafe{stdsimd::vendor::_mm256_permute4x64_epi64(i64x4::from(cdf),
                                                                                 0xee)};
    let upper_quad = unsafe{stdsimd::vendor::_mm256_castsi256_si128(__m256i::from(upper_quad_replicated))};
    let self0 = unsafe{stdsimd::vendor::_mm256_cvtepi16_epi64(i16x8::from(stdsimd::vendor::_mm256_castsi256_si128(__m256i::from(cdf))))};
    let self1 = unsafe{stdsimd::vendor::_mm256_cvtepi16_epi64(i16x8::from(stdsimd::vendor::_mm_alignr_epi8(stdsimd::vendor::_mm256_castsi256_si128(__m256i::from(cdf)),stdsimd::vendor::_mm256_castsi256_si128(__m256i::from(cdf)), 8)))};
        let self2 = unsafe{stdsimd::vendor::_mm256_cvtepi16_epi64(i16x8::from(upper_quad))};
    let self3 = unsafe{stdsimd::vendor::_mm256_cvtepi16_epi64(i16x8::from(stdsimd::vendor::_mm_alignr_epi8(upper_quad, upper_quad, 8)))};
    println!("{:?} => {:?} {:?} {:?} {:?}", cdf, self0, self1, self2, self3);
}
