// Based on the original C++ farmhashmk.cc

// Note: These functions clashed with the versions in farmhashcc
// Some only differ by taking a seed but others are quite different
// To avoid clashes I've added mk to the start of these names

use platform::*;

pub fn mk_hask32_len_13_to_24 (s: &[u8], seed: u32) -> u32 {
    let len = s.len() as usize;
    let mut a = fetch32(&s[(len>>1)-4..]);
    let b = fetch32(&s[4..]);
    let c = fetch32(&s[len-8..]);
    let d = fetch32(&s[len>>1..]);
    let e = fetch32(&s);
    let f = fetch32(&s[len-4..]);
    let mut h = d.wrapping_mul(C1) + (len as u32) + seed;
    a = rotate32(a, 12).wrapping_add(f);
    h = mur(c, h).wrapping_add(a);
    a = rotate32(a, 3).wrapping_add(c);
    h = mur(e, h).wrapping_add(a);
    a = rotate32(a.wrapping_add(f), 12).wrapping_add(d);
    h = mur(b^seed, h).wrapping_add(a);
    return fmix(h);
}

pub fn mk_hash32_len_0_to_4 (s: &[u8], seed: u32) -> u32 {
    let len = s.len() as usize;
    let mut b = seed;
    let mut c: u32 = 9;
    for i in 0..len{
        let v = s[i] as u8;
        b = b.wrapping_mul(C1).wrapping_add(v as u32);
        c ^= b;
    }
    return fmix(mur(b, mur((len as u32), c)))
}

pub fn mk_hash32_len_5_to_12 (s: &[u8], seed: u32) -> u32 {
    let len = s.len() as usize;
    let mut a = len as u32;
    let mut b =(len as u32) * 5;
    let mut c: u32 = 9;
    let d: u32 = b + seed;
    a += fetch32(&s);
    b += fetch32(&s[len-4..]);
    c += fetch32(&s[(len>>1)&4..]);
    return fmix(seed ^ mur(c, mur(b, mur(a, d))))
}