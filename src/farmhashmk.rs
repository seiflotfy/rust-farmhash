// Based on the original C++ farmhashmk.cc

// Note: These functions clashed with the versions in farmhashcc
// Some only differ by taking a seed but others are quite different
// To avoid clashes I've added mk to the start of these names

use platform::*;
use farmhashcc_shared;
use farmhashmk_shared::*;

pub fn mk_hash32_with_seed(s: &[u8], seed: u32) -> u32 {
    let len = s.len() as usize;
    if len <= 24 {
        if len >= 13 {
            return mk_hask32_len_13_to_24(s, seed.wrapping_mul(C1))
        } else if len >= 5 {
            return mk_hash32_len_5_to_12(s, seed)
        } else {
            return mk_hash32_len_0_to_4(s, seed)
        }
    }
    let h = mk_hask32_len_13_to_24(&s[0 .. 24], seed^(len as u32));
    return mur(farmhashcc_shared::hash32(&s[24..])+seed, h)
}

pub fn mk_hash32(mut s: &[u8]) -> u32{
    let len = s.len() as usize;
    if len <= 24 {
        if len <= 12 {
            if len <= 4 {
                return mk_hash32_len_0_to_4(s, 0)
            }
            return mk_hash32_len_5_to_12(s, 0)
        }
        return mk_hask32_len_13_to_24(s, 0)
    }

    // len > 24
    //var h, g, f uint32
    let mut h = len as u32;
    let mut g  = (len as u32).wrapping_mul(C1);
    let mut f: u32 = g;
    let a0 = rotate32(fetch32(&s[len-4..]).wrapping_mul(C1), 17).wrapping_mul(C2);
    let a1 = rotate32(fetch32(&s[len-8..]).wrapping_mul(C1), 17).wrapping_mul(C2);
    let a2 = rotate32(fetch32(&s[len-16..]).wrapping_mul(C1), 17).wrapping_mul(C2);
    let a3 = rotate32(fetch32(&s[len-12..]).wrapping_mul(C1), 17).wrapping_mul(C2);
    let a4 = rotate32(fetch32(&s[len-20..]).wrapping_mul(C1), 17).wrapping_mul(C2);
    h ^= a0;
    h = rotate32(h, 19);
    h = h.wrapping_mul(5).wrapping_add(0xe6546b64);
    h ^= a2;
    h = rotate32(h, 19);
    h = h.wrapping_mul(5).wrapping_add(0xe6546b64);
    g ^= a1;
    g = rotate32(g, 19);
    g = g.wrapping_mul(5).wrapping_add(0xe6546b64);
    g ^= a3;
    g = rotate32(g, 19);
    g = g.wrapping_mul(5).wrapping_add(0xe6546b64);
    f = f.wrapping_add(a4);
    f = rotate32(f, 19).wrapping_add(113);
    let mut iters = ((len - 1) / 20) as u64;
    while iters > 0 {
        let a = fetch32(&s);
        let b = fetch32(&s[4..]);
        let c = fetch32(&s[8..]);
        let d = fetch32(&s[12..]);
        let e = fetch32(&s[16..]);
        h = h.wrapping_add(a);
        g = g.wrapping_add(b);
        f = f.wrapping_add(c);
        h = mur(d, h).wrapping_add(e);
        g = mur(c, g).wrapping_add(a);
        f = mur(b.wrapping_add(e.wrapping_mul(C1)), f).wrapping_add(d);
        f = f.wrapping_add(g);
        g = g.wrapping_add(f);
        s = &s[20..];
        iters-=1;
    }
    g = rotate32(g, 11).wrapping_mul(C1);
    g = rotate32(g, 17).wrapping_mul(C1);
    f = rotate32(f, 11).wrapping_mul(C1);
    f = rotate32(f, 17).wrapping_mul(C1);
    h = rotate32(h.wrapping_add(g), 19);
    h = h.wrapping_mul(5).wrapping_add(0xe6546b64);
    h = rotate32(h, 17).wrapping_mul(C1);
    h = rotate32(h.wrapping_add(f), 19);
    h =  h.wrapping_mul(5).wrapping_add(0xe6546b64);
    h = rotate32(h, 17).wrapping_mul(C1);
    return h
}
