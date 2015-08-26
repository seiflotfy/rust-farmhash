use platform::*;

pub fn shift_mix(val: u64) -> u64 {
    val ^ (val >> 47)
}
// Note: the C++ original was overloaded hashLen16()
pub fn hash_len_16_mul(u: u64, v: u64, mul: u64) -> u64 {
    let mut a = (u ^ v).wrapping_mul(mul);
    a ^= a >> 47;
    let mut b = (v ^ a).wrapping_mul(mul);
    b ^= b >> 47;
    b = b.wrapping_mul(mul);
    return b;
}

pub fn hash_len_16(u: u64, v: u64) -> u64{
    hash128to64(Uint128{first: u, second: v})
}

pub fn hash_len_0_to_16(s: &[u8]) -> u64 {
    let len = s.len() as usize;
    if len >= 8 {
        let mul = K2.wrapping_add(len as u64 *2);
        let a = fetch64(s).wrapping_add(K2);
        let b = fetch64(&s[len - 8 ..]);
        let c = rotate64(b, 37).wrapping_mul(mul).wrapping_add(a);
        let d = rotate64(a, 25).wrapping_add(b).wrapping_mul(mul);
        return hash_len_16_mul(c, d, mul);
    }
    if len >= 4 {
        let mul = K2 + len as u64 * 2;
        let a = fetch32(s);
        return hash_len_16_mul(len as u64 + ((a as u64) << 3), fetch32(&s[len-4..]) as u64, mul);
    }
    if len > 0 {
        let a = s[0];
        let b = if len >= 2 { s[1] } else { a }; // s[len / 2]
        let c = s[len - 1];
        let y = a as u32 + ((b as u32) << 8);
        let z = len as u64 + ((c as u64) << 2);
        return shift_mix((y as u64).wrapping_mul(K2) ^ z.wrapping_mul(K0)).wrapping_mul(K2);
    }
    return K2;
}

// This probably works well for 16-byte strings as well, but it may be overkill
// in that case.
pub fn hash_len_17_to_32(s: &[u8]) -> u64 {
    let len = s.len() as usize;
    let mul = K2.wrapping_add((len as u64)*2);
    let a = fetch64(&s).wrapping_mul(K1);
    let b = fetch64(&s[8..]);
    let c = fetch64(&s[len-8..]).wrapping_mul(mul);
    let d = fetch64(&s[len-16..]).wrapping_mul(K2);
    return hash_len_16_mul(rotate64(a.wrapping_add(b), 43).wrapping_add(rotate64(c, 30)).wrapping_add(d),
        (a.wrapping_add(rotate64(b.wrapping_add(K2), 18))).wrapping_add(c), mul);
}

// Return an 8-byte hash for 33 to 64 bytes.
pub fn hash_len_33_to_64(s: &[u8]) -> u64 {
    let len = s.len() as usize;
    let mul = K2.wrapping_add((len as u64) * 2);
    let a = fetch64(&s).wrapping_mul(K2);
    let b = fetch64(&s[8..]);
    let c = fetch64(&s[len-8..]).wrapping_mul(mul);
    let d = fetch64(&s[len-16..]).wrapping_mul(K2);
    let y = rotate64(a.wrapping_add(b), 43).wrapping_add(rotate64(c, 30)).wrapping_add(d);
    let z = hash_len_16_mul(y, a.wrapping_add(rotate64(b.wrapping_add(K2), 18)).wrapping_add(c), mul);
    let e = fetch64(&s[16..]).wrapping_mul(mul);
    let f = fetch64(&s[24..]);
    let g = (y.wrapping_add(fetch64(&s[len-32..]))).wrapping_mul(mul);
    let h = (z.wrapping_add(fetch64(&s[len-24..]))).wrapping_mul(mul);
    return hash_len_16_mul(rotate64(e.wrapping_add(f), 43).wrapping_add(rotate64(g, 30).wrapping_add(h)),
        e.wrapping_add(rotate64(f.wrapping_add(a), 18).wrapping_add(g)), mul);
}
