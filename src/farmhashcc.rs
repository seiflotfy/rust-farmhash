// Based on the original C++ farmhashcc.cc

// This file provides a 32-bit hash equivalent to CityHash32 (v1.1.1)
// and a 128-bit hash equivalent to CityHash128 (v1.1.1).  It also provides
// a seeded 32-bit hash function similar to CityHash32.

use platform::*;
use farmhashmk_shared;
use farmhashcc_shared;


fn hash32_with_seed(s: &[u8], seed: u32) -> u32 {
    let len = s.len() as u64;
    if len <= 24 {
        if len >= 13 {
            return farmhashmk_shared::mk_hask32_len_13_to_24(s, seed.wrapping_mul(C1))
        } else if len >= 5 {
            return farmhashmk_shared::mk_hash32_len_5_to_12(s, seed)
        } else {
            return farmhashmk_shared::mk_hash32_len_0_to_4(s, seed)
        }
    }
    let h = farmhashmk_shared::mk_hask32_len_13_to_24(&s[0..24], seed^(len as u32));
    return mur(farmhashcc_shared::hash32(&s[24..])+seed, h);
}

pub fn hash32(mut s: &[u8]) ->u32 {
    farmhashcc_shared::hash32(s)
}
