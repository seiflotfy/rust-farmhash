use platform::*;
use farmhashna::*;
use farmhashna_shared::*;
use farmhashcc_shared::weak_hash_len_32_with_seeds_bytes;
use std::mem;

fn uo_h(x: u64, y: u64, mul: u64, r: u64) -> u64 {
    let mut a = (x ^ y).wrapping_mul(mul);
    a ^= a >> 47;
    let b = (y ^ a).wrapping_mul(mul);
    return rotate64(b, r).wrapping_mul(mul);
}

pub fn uo_hash64_with_seeds(mut s: &[u8], seed0: u64, seed1: u64) -> u64 {
    let len = s.len() as usize;
    if len <= 64 {
        return na_hash64_with_seeds(s, seed0, seed1)
    }

    // For strings over 64 bytes we loop.  Internal state consists of
    // 64 bytes: u, v, w, x, y, and z.
    let mut x = seed0;
    let mut y = seed1.wrapping_mul(K1).wrapping_add(113);
    let mut z =  shift_mix(y.wrapping_mul(K2)).wrapping_mul(K2);
    let mut v = Uint128{first: seed0, second: seed1};
    let mut w = Uint128{first: 0, second: 0};
    let mut u = x.wrapping_sub(z);
    x = x.wrapping_mul(K2);
    let mul = K2.wrapping_add(u & 0x82);

    // Process the input in blocks of 64 bytes
    let last64 = &s[len - 64..];

    while s.len() >= 64 {
        // The `s.len() >= 64` should allow the boundary checks to be elided.
        let a0 = fetch64(&s[0..8]);
        let a1 = fetch64(&s[8..16]);
        let a2 = fetch64(&s[16..24]);
        let a3 = fetch64(&s[24..32]);
        let a4 = fetch64(&s[32..40]);
        let a5 = fetch64(&s[40..48]);
        let a6 = fetch64(&s[48..56]);
        let a7 = fetch64(&s[56..64]);
        x = x.wrapping_add(a0).wrapping_add(a1);
        y = y.wrapping_add(a2);
        z = z.wrapping_add(a3);
        v.first = v.first.wrapping_add(a4);
        v.second = v.second.wrapping_add(a5).wrapping_add(a1);
        w.first = w.first.wrapping_add(a6);
        w.second = w.second.wrapping_add(a7);

        x = rotate64(x, 26);
        x = x.wrapping_mul(9);
        y = rotate64(y, 29);
        z = z.wrapping_mul(mul);

        v.first = rotate64(v.first, 33);
        v.second = rotate64(v.second, 30);
        w.first ^= x;
        w.first = w.first.wrapping_mul(9);
        z = rotate64(z, 32);
        z = z.wrapping_add(w.second);
        w.second = w.second.wrapping_add(z);
        z = z.wrapping_mul(9);

        mem::swap(&mut u, &mut y);

        z = z.wrapping_add(a0).wrapping_add(a6);
        v.first = v.first.wrapping_add(a2);
        v.second = v.second.wrapping_add(a3);
        w.first = w.first.wrapping_add(a4);
        w.second = w.second.wrapping_add(a5).wrapping_add(a6);
        x = x.wrapping_add(a1);
        y = y.wrapping_add(a7);

        y = y.wrapping_add(v.first);
        v.first = v.first.wrapping_add(x).wrapping_sub(y);
        v.second = v.second.wrapping_add(w.first);
        w.first = w.first.wrapping_add(v.second);
        w.second = w.second.wrapping_add(x).wrapping_sub(y);
        x = x.wrapping_add(w.second);
        w.second = rotate64(w.second, 34);

        mem::swap(&mut u, &mut z);
        s = &s[64..];
    }
    // Make s point to the last 64 bytes of input.
    s = last64;
    u = u.wrapping_mul(9);
    v.second = rotate64(v.second, 28);
    v.first = rotate64(v.first, 20);
    w.first = w.first.wrapping_add((len-1) as u64 & 63);
    u = u.wrapping_add(y);
    y = y.wrapping_add(u);
    x = rotate64(y.wrapping_sub(x).wrapping_add(v.first).wrapping_add(fetch64(&s[8..])), 37).wrapping_mul(mul);
    y = rotate64(y^v.second^fetch64(&s[48..]), 42).wrapping_mul(mul);

    x ^= w.second.wrapping_mul(9);
    y = y.wrapping_add(v.first).wrapping_add(fetch64(&s[40..]));
    z = rotate64(z.wrapping_add(w.first), 33).wrapping_mul(mul);

    v = weak_hash_len_32_with_seeds_bytes(s, v.second.wrapping_mul(mul), x.wrapping_add(w.first));
    w = weak_hash_len_32_with_seeds_bytes(&s[32..], z.wrapping_add(w.second), y.wrapping_add(fetch64(&s[16..])));
    return uo_h(hash_len_16_mul(v.first.wrapping_add(x), w.first^y, mul).wrapping_add(z).wrapping_sub(u),
        uo_h(v.second.wrapping_add(y), w.second.wrapping_add(z), K2, 30)^x,
        K2,
        31);
}

pub fn uo_hash64_with_seed(s: &[u8], seed: u64) -> u64 {
    if s.len() <= 64 {
        return na_hash64_with_seed(s, seed)
    }
    return uo_hash64_with_seeds(s, 0, seed)
}

pub fn uo_hash64(s: &[u8]) -> u64 {
    if s.len() <= 64 {
        return na_hash64(s)
    }
    return uo_hash64_with_seeds(s, 81, 0)
}
