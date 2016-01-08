mod platform;
mod farmhashna;
mod farmhashmk;
mod farmhashuo;
mod farmhashxo;
mod farmhashna_shared;
mod farmhashcc_shared;
mod farmhashmk_shared;

use farmhashna::na_hash64;
use farmhashmk::mk_hash32;
use farmhashmk::mk_hash32_with_seed;
use farmhashxo::xo_hash64;
use farmhashxo::xo_hash64_with_seed;
use farmhashxo::xo_hash64_with_seeds;
use std::hash::Hasher;

/// Create a new farmhash based u32 for an array of bytes.  Hash value may vary
/// with library version and platform.
///
/// # Examples
///
/// ```
/// let value: &str = "hello world";
/// let res32 = farmhash::hash32(&value.as_bytes());
/// //res32 ==> 430397466
/// ```
pub fn hash32(s: &[u8]) -> u32 {
    mk_hash32(s)
}

/// Create a new farmhash based u32 for an array of bytes with a given seed.
/// Hash value may vary with library version and platform.
pub fn hash32_with_seed(s: &[u8], seed: u32) -> u32 {
    mk_hash32_with_seed(s, seed)
}

/// Create a new farmhash based u64 for an array of bytes.  Hash value may
/// vary with library version and platform.
///
/// # Examples
///
/// ```
/// let value: &str = "hello world";
/// let res64 = farmhash::hash64(&value.as_bytes());
/// //res64 ==> 6381520714923946011
/// ```
pub fn hash64(s: &[u8]) -> u64 {
    xo_hash64(s)
}

/// Create a new farmhash based u64 for an array of bytes with a given seed.
/// Hash value may vary with library version and platform.
pub fn hash64_with_seed(s: &[u8], seed: u64) -> u64 {
    xo_hash64_with_seed(s, seed)
}

/// Create a new farmhash based u64 for an array of bytes with 2 given seeds.
/// Hash value may vary with library version and platform.
pub fn hash64_with_seeds(s: &[u8], seed0: u64, seed1: u64) -> u64 {
    xo_hash64_with_seeds(s, seed0, seed1)
}

/// Create a new farmhash based u32 for an array of bytes.  Fingerprint value
/// should be portable and stable across library versions and platforms.
///
/// # Examples
///
/// ```
/// let value: &str = "hello world";
/// let res32 = farmhash::fingerprint32(&value.as_bytes());
/// //res32 ==> 430397466
/// ```
pub fn fingerprint32(s: &[u8]) -> u32 {
    mk_hash32(s)
}

/// Create a stable farmhash based u64 for an array of bytes.  Fingerprint value
/// should be portable and stable across library versions and platforms.
///
/// # Examples
///
/// ```
/// let value: &str = "hello world";
/// let res64 = farmhash::fingerprint64(&value.as_bytes());
/// //res64 ==> 6381520714923946011
/// ```
pub fn fingerprint64(s: &[u8]) -> u64 {
    na_hash64(s)
}

pub struct FarmHasher {
    bytes: Vec<u8>
}

impl Default for FarmHasher {
    #[inline]
    fn default() -> FarmHasher { FarmHasher{bytes: Vec::with_capacity(20)} }
}

impl Hasher for FarmHasher {
    #[inline]
    fn finish(&self) -> u64 {
        hash64(&self.bytes[..])
    }
    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        self.bytes.extend(bytes.iter().cloned());
    }
}
