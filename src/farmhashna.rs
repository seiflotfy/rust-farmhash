// Based on the original C++ farmhashna.cc
use platform::*;
use farmhashna_shared::*;
use farmhashcc_shared;
use std::mem;

// renamed from hash64 to make it clearer elsewhere which is being called
pub fn na_hash64(mut s: &[u8]) -> u64 {
    let len = s.len() as usize;
    const SEED: u64 = 81;
    if len <= 32 {
        if len <= 16 {
            return hash_len_0_to_16(s);
        } else {
            return hash_len_17_to_32(s);
        }
    } else if len <= 64{
        return hash_len_33_to_64(s);
    }
    // For strings over 64 bytes we loop.  Internal state consists of
    // 56 bytes: v, w, x, y, and z.
    let mut x = SEED;
    let mut y: u64 = 2480279821605975764; // y := (seed*k1) + 113
    let mut z = shift_mix(y.wrapping_mul(K2).wrapping_add(113)).wrapping_mul(K2);
    let mut v = Uint128 {first:0, second:0};
    let mut w = Uint128 {first:0, second:0};

    x = x.wrapping_mul(K2).wrapping_add(fetch64(s));

    let last64 = &s[len - 64..];
    while {
        x = rotate64(x.wrapping_add(y).wrapping_add(v.first).wrapping_add(fetch64(&s[8..])), 37).wrapping_mul(K1);
        y = rotate64(y.wrapping_add(v.second).wrapping_add(fetch64(&s[48..])), 42).wrapping_mul(K1);
        x ^= w.second;
        y = y.wrapping_add(v.first).wrapping_add(fetch64(&s[40..]));
        z = rotate64(z.wrapping_add(w.first), 33).wrapping_mul(K1);
        v = farmhashcc_shared::weak_hash_len_32_with_seeds_bytes(&s,v.second.wrapping_mul(K1), x.wrapping_add(w.first));
        w = farmhashcc_shared::weak_hash_len_32_with_seeds_bytes(&s[32..],z.wrapping_add(w.second), y.wrapping_add(fetch64(&s[16..])));

        mem::swap(&mut z, &mut x);
        s = &s[64..];
        s.len() >= 64
    } {}

    let mul = K1 + ((z & 0xff) << 1);

    // Make s point to the last 64 bytes of input.
    s = last64;
    w.first = w.first.wrapping_add(((len as u64 - 1) & 63));
    v.first = v.first.wrapping_add(w.first);
    w.first = w.first.wrapping_add(v.first);
    x = rotate64(x.wrapping_add(y)
        .wrapping_add(v.first.wrapping_add(fetch64(&s[8..]))), 37).wrapping_mul(mul);
    y = rotate64(y.wrapping_add(v.second).wrapping_add(fetch64(&s[48..])), 42).wrapping_mul(mul);
    x ^= w.second.wrapping_mul(9);
    y = y.wrapping_add(v.first.wrapping_mul(9).wrapping_add(fetch64(&s[40..])));
    z = rotate64(z.wrapping_add(w.first), 33).wrapping_mul(mul);
    v = farmhashcc_shared::weak_hash_len_32_with_seeds_bytes(&s, v.second.wrapping_mul(mul),
        x.wrapping_add(w.first));
    w = farmhashcc_shared::weak_hash_len_32_with_seeds_bytes(&s[32..], z.wrapping_add(w.second),
        y.wrapping_add(fetch64(&s[16..])));
    mem::swap(&mut z, &mut x);
    return hash_len_16_mul(hash_len_16_mul(v.first, w.first, mul).wrapping_add(shift_mix(y).wrapping_mul(K0).wrapping_add(z)),
        hash_len_16_mul(v.second, w.second, mul).wrapping_add(x),
        mul)
}

pub fn na_hash64_with_seed(s: &[u8], seed: u64) -> u64 {
    return na_hash64_with_seeds(s, K2, seed)
}

pub fn na_hash64_with_seeds(s: &[u8], seed0: u64, seed1: u64) -> u64 {
    return hash_len_16(na_hash64(s).wrapping_sub(seed0), seed1)
}
