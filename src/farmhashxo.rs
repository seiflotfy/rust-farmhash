use platform::*;
use farmhashna::*;
use farmhashuo::*;
use farmhashna_shared::*;

fn xo_h32(s: &[u8], len: usize, mul: u64, seed0: u64, seed1: u64) -> u64 {
    let mut a = fetch64(s).wrapping_mul(K1);
    let mut b = fetch64(&s[8..]);
    let c = fetch64(&s[len - 8..]).wrapping_mul(mul);
    let d = fetch64(&s[len - 16 ..]).wrapping_mul(K2);
    let u = rotate64(a.wrapping_add(b), 43) .wrapping_add(rotate64(c, 30)) .wrapping_add(d) .wrapping_add(seed0);
    let v = a.wrapping_add(rotate64(b.wrapping_add(K2), 18).wrapping_add(c).wrapping_add(seed1));
    a = shift_mix((u ^ v).wrapping_mul(mul));
    b = shift_mix((v ^ a).wrapping_mul(mul));

    b
}

// Return an 8-byte hash for 33 to 64 bytes.
fn xo_hash_len_33_to_64(s: &[u8], len: usize) -> u64 {
    let mul0 = K2.wrapping_sub(30);
    let mul1 = K2.wrapping_sub(30).wrapping_add(2 * len as u64);
    let h0 = xo_h32(s, 32, mul0, 0, 0);
    let h1 = xo_h32(&s[s.len() - 32..], 32, mul1, 0, 0);

    ((h1.wrapping_mul(mul1)).wrapping_add(h0)).wrapping_mul(mul1)
}

// Return an 8-byte hash for 65 to 96 bytes.
fn xo_hash_len_65_to_96(s: &[u8], len: usize) -> u64 {
    let mul0 = K2.wrapping_sub(114);
    let mul1 = K2.wrapping_sub(114).wrapping_add(2 * len as u64);
    let h0 = xo_h32(s, 32, mul0, 0, 0);
    let h1 = xo_h32(&s[32..], 32, mul1, 0, 0);
    let h2 = xo_h32(&s[s.len() - 32..], 32, mul1, h0, h1);

    (h2 .wrapping_mul(9)
        .wrapping_add(h0 >> 17)
        .wrapping_add(h1 >> 21)).wrapping_mul(mul1)
}

pub fn xo_hash64(s: &[u8]) -> u64 {
    match s.len() {
        0 ...16  => hash_len_0_to_16(s),
        17...32  => hash_len_17_to_32(s),
        33...64  => xo_hash_len_33_to_64(s, s.len()),
        65...96  => xo_hash_len_65_to_96(s, s.len()),
        97...256 => na_hash64(s),
        _ => uo_hash64(s),
    }
}

pub fn xo_hash64_with_seeds(s: &[u8], seed0: u64, seed1: u64) -> u64 {
    uo_hash64_with_seeds(s, seed0, seed1)
}

pub fn xo_hash64_with_seed(s: &[u8], seed: u64) -> u64 {
    uo_hash64_with_seed(s, seed)
}
