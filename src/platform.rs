// Based on the original C++ platform.cc

// A small optimisation I switched from binary.LittleEndian.Uint32/64 to
// hand coded. The benchmark, on my system, went from
// around 64ns/op to around 51ns/op

// Note: I used to call binary.LittleEndian.Uint32 and Uint64 inline but it
// made comparing the code to the original much trickier

use std::mem;
use std::ptr;

#[inline]
pub fn fetch32(p: &[u8]) -> u32 {
    assert!(p.len() >= 4);
    let mut result = 0u32;
    unsafe {
        ptr::copy_nonoverlapping(p.as_ptr(), &mut result as *mut _ as *mut u8, mem::size_of::<u32>());
    }
    result.to_le()
}

#[inline]
pub fn fetch64(p: &[u8]) -> u64 {
    assert!(p.len() >= 8);
    let mut result = 0u64;
    unsafe {
        ptr::copy_nonoverlapping(p.as_ptr(), &mut result as *mut _ as *mut u8, mem::size_of::<u64>());
    }
    result.to_le()
}

// rotate32 is a bitwise rotate
pub fn rotate32(val: u32, shift: u32) -> u32 {
    if shift == 0 {
        return val;
    }
    return val >> shift | val << (32 - shift)
}

// rotate64 is a bitwise rotate
pub fn rotate64(val: u64, shift: u64) -> u64 {
    if shift == 0 {
        return val;
    }
    return val >> shift | val << (64 - shift)
}


// Some primes between 2^63 and 2^64 for various uses.
pub const K0: u64 = 0xc3a5c85c97cb3127;
pub const K1: u64 = 0xb492b66fbe98f273;
pub const K2: u64 = 0x9ae16a3b2f90404f;

// Magic numbers for 32-bit hashing.  Copied from Murmur3.
pub const C1: u32 = 0xcc9e2d51;
pub const C2: u32 = 0x1b873593;

// fmix is a 32-bit to 32-bit integer hash copied from Murmur3.
pub fn fmix(mut h: u32) -> u32 {
    h ^= h >> 16;
    h = h.wrapping_mul(0x85ebca6b);
    h ^= h >> 13;
    h = h.wrapping_mul(0xc2b2ae35);
    h ^= h >> 16;
    return h;
}

// Mur is a helper from Murmur3 for combining two 32-bit values.
pub fn mur(mut a: u32, mut h: u32) -> u32 {
    a = a.wrapping_mul(C1);
    a = rotate32(a, 17);
    a = a.wrapping_mul(C2);
    h ^= a;
    h = rotate32(h, 19);
    return h.wrapping_mul(5).wrapping_add(0xe6546b64);
}

pub fn bswap32(x: u32) -> u32 {
    return ((x >> 24) & 0xFF) | ((x >> 8) & 0xFF00) |
        ((x << 8) & 0xFF0000) | ((x << 24) & 0xFF000000)
}

pub struct Uint128 {
    pub first: u64,
    pub second: u64,
}

// hash128to64 is from farmhash.h it is intended to be a reasonably good hash function.
pub fn hash128to64(x: Uint128) -> u64 {
    // Murmur-inspired hashing.
    const KMUL: u64 = 0x9ddfea08eb382d69;
    let mut a = (x.first ^ x.second).wrapping_mul(KMUL);
    a ^= a >> 47;
    let mut b = (x.second ^ a).wrapping_mul(KMUL);
    b ^= b >> 47;
    b = b.wrapping_mul(KMUL);
    return b;
}


#[cfg(test)]
struct NumIn2Out {
    in1: u32,
    in2: u32,
    out: u32
}


#[cfg(test)]
struct NumInOut {
    in1: u32,
    out: u32
}

#[test]
fn test_mur() {
    let i = mur(12, 1);
    assert_eq!(i, 4253658843);
}

#[test]
fn test_bswap32() {
    let i = bswap32(12);
    assert_eq!(i, 201326592);
}
#[test]
fn test_hash128to64() {
    let j = hash128to64(Uint128{first:12, second:1});
    assert_eq!(j, 463069014307918310);
}

#[test]
fn test_rotate64() {
    let rots = vec![
        NumIn2Out{in1: 0, in2: 0, out: 0},
        // Rotate32
        NumIn2Out{in1: 100, in2: 0, out: 100},
        NumIn2Out{in1: 100, in2: 1, out: 50},
        NumIn2Out{in1: 100, in2: 2, out: 25},
        NumIn2Out{in1: 100, in2: 3, out: 2147483660},
        NumIn2Out{in1: 100, in2: 4, out: 1073741830},
        NumIn2Out{in1: 100, in2: 5, out: 536870915},
        NumIn2Out{in1: 100, in2: 6, out: 2415919105},
        NumIn2Out{in1: 100, in2: 7, out: 3355443200},
        NumIn2Out{in1: 100, in2: 8, out: 1677721600},
        NumIn2Out{in1: 100, in2: 9, out: 838860800},
        NumIn2Out{in1: 100, in2: 10, out: 419430400},
        NumIn2Out{in1: 100, in2: 11, out: 209715200},
        NumIn2Out{in1: 100, in2: 12, out: 104857600},
        NumIn2Out{in1: 100, in2: 13, out: 52428800},
        NumIn2Out{in1: 100, in2: 14, out: 26214400},
        NumIn2Out{in1: 100, in2: 15, out: 13107200},
        NumIn2Out{in1: 100, in2: 16, out: 6553600},
        NumIn2Out{in1: 100, in2: 17, out: 3276800},
        NumIn2Out{in1: 100, in2: 18, out: 1638400},
        NumIn2Out{in1: 100, in2: 19, out: 819200},
        NumIn2Out{in1: 100, in2: 20, out: 409600},
        NumIn2Out{in1: 100, in2: 21, out: 204800},
        NumIn2Out{in1: 100, in2: 22, out: 102400},
        NumIn2Out{in1: 100, in2: 23, out: 51200},
        NumIn2Out{in1: 100, in2: 24, out: 25600},
        NumIn2Out{in1: 100, in2: 25, out: 12800},
        NumIn2Out{in1: 100, in2: 26, out: 6400},
        NumIn2Out{in1: 100, in2: 27, out: 3200},
        NumIn2Out{in1: 100, in2: 28, out: 1600},
        NumIn2Out{in1: 100, in2: 29, out: 800},
        NumIn2Out{in1: 100, in2: 30, out: 400},
        NumIn2Out{in1: 100, in2: 31, out: 200},
        NumIn2Out{in1: 1000, in2: 0, out: 1000},
        NumIn2Out{in1: 1000, in2: 1, out: 500},
        NumIn2Out{in1: 1000, in2: 2, out: 250},
        NumIn2Out{in1: 1000, in2: 3, out: 125},
        NumIn2Out{in1: 1000, in2: 4, out: 2147483710},
        NumIn2Out{in1: 1000, in2: 5, out: 1073741855},
        NumIn2Out{in1: 1000, in2: 6, out: 2684354575},
        NumIn2Out{in1: 1000, in2: 7, out: 3489660935},
        NumIn2Out{in1: 1000, in2: 8, out: 3892314115},
        NumIn2Out{in1: 1000, in2: 9, out: 4093640705},
        NumIn2Out{in1: 1000, in2: 10, out: 4194304000},
        NumIn2Out{in1: 1000, in2: 11, out: 2097152000},
        NumIn2Out{in1: 1000, in2: 12, out: 1048576000},
        NumIn2Out{in1: 1000, in2: 13, out: 524288000},
        NumIn2Out{in1: 1000, in2: 14, out: 262144000},
        NumIn2Out{in1: 1000, in2: 15, out: 131072000},
        NumIn2Out{in1: 1000, in2: 16, out: 65536000},
        NumIn2Out{in1: 1000, in2: 17, out: 32768000},
        NumIn2Out{in1: 1000, in2: 18, out: 16384000},
        NumIn2Out{in1: 1000, in2: 19, out: 8192000},
        NumIn2Out{in1: 1000, in2: 20, out: 4096000},
        NumIn2Out{in1: 1000, in2: 21, out: 2048000},
        NumIn2Out{in1: 1000, in2: 22, out: 1024000},
        NumIn2Out{in1: 1000, in2: 23, out: 512000},
        NumIn2Out{in1: 1000, in2: 24, out: 256000},
        NumIn2Out{in1: 1000, in2: 25, out: 128000},
        NumIn2Out{in1: 1000, in2: 26, out: 64000},
        NumIn2Out{in1: 1000, in2: 27, out: 32000},
        NumIn2Out{in1: 1000, in2: 28, out: 16000},
        NumIn2Out{in1: 1000, in2: 29, out: 8000},
        NumIn2Out{in1: 1000, in2: 30, out: 4000},
        NumIn2Out{in1: 1000, in2: 31, out: 2000},
        NumIn2Out{in1: 100000, in2: 0, out: 100000},
        NumIn2Out{in1: 100000, in2: 1, out: 50000},
        NumIn2Out{in1: 100000, in2: 2, out: 25000},
        NumIn2Out{in1: 100000, in2: 3, out: 12500},
        NumIn2Out{in1: 100000, in2: 4, out: 6250},
        NumIn2Out{in1: 100000, in2: 5, out: 3125},
        NumIn2Out{in1: 100000, in2: 6, out: 2147485210},
        NumIn2Out{in1: 100000, in2: 7, out: 1073742605},
        NumIn2Out{in1: 100000, in2: 8, out: 2684354950},
        NumIn2Out{in1: 100000, in2: 9, out: 1342177475},
        NumIn2Out{in1: 100000, in2: 10, out: 2818572385},
        NumIn2Out{in1: 100000, in2: 11, out: 3556769840},
        NumIn2Out{in1: 100000, in2: 12, out: 1778384920},
        NumIn2Out{in1: 100000, in2: 13, out: 889192460},
        NumIn2Out{in1: 100000, in2: 14, out: 444596230},
        NumIn2Out{in1: 100000, in2: 15, out: 222298115},
        NumIn2Out{in1: 100000, in2: 16, out: 2258632705},
        NumIn2Out{in1: 100000, in2: 17, out: 3276800000},
        NumIn2Out{in1: 100000, in2: 18, out: 1638400000},
        NumIn2Out{in1: 100000, in2: 19, out: 819200000},
        NumIn2Out{in1: 100000, in2: 20, out: 409600000},
        NumIn2Out{in1: 100000, in2: 21, out: 204800000},
        NumIn2Out{in1: 100000, in2: 22, out: 102400000},
        NumIn2Out{in1: 100000, in2: 23, out: 51200000},
        NumIn2Out{in1: 100000, in2: 24, out: 25600000},
        NumIn2Out{in1: 100000, in2: 25, out: 12800000},
        NumIn2Out{in1: 100000, in2: 26, out: 6400000},
        NumIn2Out{in1: 100000, in2: 27, out: 3200000},
        NumIn2Out{in1: 100000, in2: 28, out: 1600000},
        NumIn2Out{in1: 100000, in2: 29, out: 800000},
        NumIn2Out{in1: 100000, in2: 30, out: 400000},
        NumIn2Out{in1: 100000, in2: 31, out: 200000},
        NumIn2Out{in1: 1000000, in2: 0, out: 1000000},
        NumIn2Out{in1: 1000000, in2: 1, out: 500000},
        NumIn2Out{in1: 1000000, in2: 2, out: 250000},
        NumIn2Out{in1: 1000000, in2: 3, out: 125000},
        NumIn2Out{in1: 1000000, in2: 4, out: 62500},
        NumIn2Out{in1: 1000000, in2: 5, out: 31250},
        NumIn2Out{in1: 1000000, in2: 6, out: 15625},
        NumIn2Out{in1: 1000000, in2: 7, out: 2147491460},
        NumIn2Out{in1: 1000000, in2: 8, out: 1073745730},
        NumIn2Out{in1: 1000000, in2: 9, out: 536872865},
        NumIn2Out{in1: 1000000, in2: 10, out: 2415920080},
        NumIn2Out{in1: 1000000, in2: 11, out: 1207960040},
        NumIn2Out{in1: 1000000, in2: 12, out: 603980020},
        NumIn2Out{in1: 1000000, in2: 13, out: 301990010},
        NumIn2Out{in1: 1000000, in2: 14, out: 150995005},
        NumIn2Out{in1: 1000000, in2: 15, out: 2222981150},
        NumIn2Out{in1: 1000000, in2: 16, out: 1111490575},
        NumIn2Out{in1: 1000000, in2: 17, out: 2703228935},
        NumIn2Out{in1: 1000000, in2: 18, out: 3499098115},
        NumIn2Out{in1: 1000000, in2: 19, out: 3897032705},
        NumIn2Out{in1: 1000000, in2: 20, out: 4096000000},
        NumIn2Out{in1: 1000000, in2: 21, out: 2048000000},
        NumIn2Out{in1: 1000000, in2: 22, out: 1024000000},
        NumIn2Out{in1: 1000000, in2: 23, out: 512000000},
        NumIn2Out{in1: 1000000, in2: 24, out: 256000000},
        NumIn2Out{in1: 1000000, in2: 25, out: 128000000},
        NumIn2Out{in1: 1000000, in2: 26, out: 64000000},
        NumIn2Out{in1: 1000000, in2: 27, out: 32000000},
        NumIn2Out{in1: 1000000, in2: 28, out: 16000000},
        NumIn2Out{in1: 1000000, in2: 29, out: 8000000},
        NumIn2Out{in1: 1000000, in2: 30, out: 4000000},
        NumIn2Out{in1: 1000000, in2: 31, out: 2000000},
        NumIn2Out{in1: 10000000, in2: 0, out: 10000000},
        NumIn2Out{in1: 10000000, in2: 1, out: 5000000},
        NumIn2Out{in1: 10000000, in2: 2, out: 2500000},
        NumIn2Out{in1: 10000000, in2: 3, out: 1250000},
        NumIn2Out{in1: 10000000, in2: 4, out: 625000},
        NumIn2Out{in1: 10000000, in2: 5, out: 312500},
        NumIn2Out{in1: 10000000, in2: 6, out: 156250},
        NumIn2Out{in1: 10000000, in2: 7, out: 78125},
        NumIn2Out{in1: 10000000, in2: 8, out: 2147522710},
        NumIn2Out{in1: 10000000, in2: 9, out: 1073761355},
        NumIn2Out{in1: 10000000, in2: 10, out: 2684364325},
        NumIn2Out{in1: 10000000, in2: 11, out: 3489665810},
        NumIn2Out{in1: 10000000, in2: 12, out: 1744832905},
        NumIn2Out{in1: 10000000, in2: 13, out: 3019900100},
        NumIn2Out{in1: 10000000, in2: 14, out: 1509950050},
        NumIn2Out{in1: 10000000, in2: 15, out: 754975025},
        NumIn2Out{in1: 10000000, in2: 16, out: 2524971160},
        NumIn2Out{in1: 10000000, in2: 17, out: 1262485580},
        NumIn2Out{in1: 10000000, in2: 18, out: 631242790},
        NumIn2Out{in1: 10000000, in2: 19, out: 315621395},
        NumIn2Out{in1: 10000000, in2: 20, out: 2305294345},
        NumIn2Out{in1: 10000000, in2: 21, out: 3300130820},
        NumIn2Out{in1: 10000000, in2: 22, out: 1650065410},
        NumIn2Out{in1: 10000000, in2: 23, out: 825032705},
        NumIn2Out{in1: 10000000, in2: 24, out: 2560000000},
        NumIn2Out{in1: 10000000, in2: 25, out: 1280000000},
        NumIn2Out{in1: 10000000, in2: 26, out: 640000000},
        NumIn2Out{in1: 10000000, in2: 27, out: 320000000},
        NumIn2Out{in1: 10000000, in2: 28, out: 160000000},
        NumIn2Out{in1: 10000000, in2: 29, out: 80000000},
        NumIn2Out{in1: 10000000, in2: 30, out: 40000000},
        NumIn2Out{in1: 10000000, in2: 31, out: 20000000},
        NumIn2Out{in1: 100000000, in2: 0, out: 100000000},
        NumIn2Out{in1: 100000000, in2: 1, out: 50000000},
        NumIn2Out{in1: 100000000, in2: 2, out: 25000000},
        NumIn2Out{in1: 100000000, in2: 3, out: 12500000},
        NumIn2Out{in1: 100000000, in2: 4, out: 6250000},
        NumIn2Out{in1: 100000000, in2: 5, out: 3125000},
        NumIn2Out{in1: 100000000, in2: 6, out: 1562500},
        NumIn2Out{in1: 100000000, in2: 7, out: 781250},
        NumIn2Out{in1: 100000000, in2: 8, out: 390625},
        NumIn2Out{in1: 100000000, in2: 9, out: 2147678960},
        NumIn2Out{in1: 100000000, in2: 10, out: 1073839480},
        NumIn2Out{in1: 100000000, in2: 11, out: 536919740},
        NumIn2Out{in1: 100000000, in2: 12, out: 268459870},
        NumIn2Out{in1: 100000000, in2: 13, out: 134229935},
        NumIn2Out{in1: 100000000, in2: 14, out: 2214598615},
        NumIn2Out{in1: 100000000, in2: 15, out: 3254782955},
        NumIn2Out{in1: 100000000, in2: 16, out: 3774875125},
        NumIn2Out{in1: 100000000, in2: 17, out: 4034921210},
        NumIn2Out{in1: 100000000, in2: 18, out: 2017460605},
        NumIn2Out{in1: 100000000, in2: 19, out: 3156213950},
        NumIn2Out{in1: 100000000, in2: 20, out: 1578106975},
        NumIn2Out{in1: 100000000, in2: 21, out: 2936537135},
        NumIn2Out{in1: 100000000, in2: 22, out: 3615752215},
        NumIn2Out{in1: 100000000, in2: 23, out: 3955359755},
        NumIn2Out{in1: 100000000, in2: 24, out: 4125163525},
        NumIn2Out{in1: 100000000, in2: 25, out: 4210065410},
        NumIn2Out{in1: 100000000, in2: 26, out: 2105032705},
        NumIn2Out{in1: 100000000, in2: 27, out: 3200000000},
        NumIn2Out{in1: 100000000, in2: 28, out: 1600000000},
        NumIn2Out{in1: 100000000, in2: 29, out: 800000000},
        NumIn2Out{in1: 100000000, in2: 30, out: 400000000},
        NumIn2Out{in1: 100000000, in2: 31, out: 200000000},
        NumIn2Out{in1: 1000000000, in2: 0, out: 1000000000},
        NumIn2Out{in1: 1000000000, in2: 1, out: 500000000},
        NumIn2Out{in1: 1000000000, in2: 2, out: 250000000},
        NumIn2Out{in1: 1000000000, in2: 3, out: 125000000},
        NumIn2Out{in1: 1000000000, in2: 4, out: 62500000},
        NumIn2Out{in1: 1000000000, in2: 5, out: 31250000},
        NumIn2Out{in1: 1000000000, in2: 6, out: 15625000},
        NumIn2Out{in1: 1000000000, in2: 7, out: 7812500},
        NumIn2Out{in1: 1000000000, in2: 8, out: 3906250},
        NumIn2Out{in1: 1000000000, in2: 9, out: 1953125},
        NumIn2Out{in1: 1000000000, in2: 10, out: 2148460210},
        NumIn2Out{in1: 1000000000, in2: 11, out: 1074230105},
        NumIn2Out{in1: 1000000000, in2: 12, out: 2684598700},
        NumIn2Out{in1: 1000000000, in2: 13, out: 1342299350},
        NumIn2Out{in1: 1000000000, in2: 14, out: 671149675},
        NumIn2Out{in1: 1000000000, in2: 15, out: 2483058485},
        NumIn2Out{in1: 1000000000, in2: 16, out: 3389012890},
        NumIn2Out{in1: 1000000000, in2: 17, out: 1694506445},
        NumIn2Out{in1: 1000000000, in2: 18, out: 2994736870},
        NumIn2Out{in1: 1000000000, in2: 19, out: 1497368435},
        NumIn2Out{in1: 1000000000, in2: 20, out: 2896167865},
        NumIn2Out{in1: 1000000000, in2: 21, out: 3595567580},
        NumIn2Out{in1: 1000000000, in2: 22, out: 1797783790},
        NumIn2Out{in1: 1000000000, in2: 23, out: 898891895},
        NumIn2Out{in1: 1000000000, in2: 24, out: 2596929595},
        NumIn2Out{in1: 1000000000, in2: 25, out: 3445948445},
        NumIn2Out{in1: 1000000000, in2: 26, out: 3870457870},
        NumIn2Out{in1: 1000000000, in2: 27, out: 1935228935},
        NumIn2Out{in1: 1000000000, in2: 28, out: 3115098115},
        NumIn2Out{in1: 1000000000, in2: 29, out: 3705032705},
        NumIn2Out{in1: 1000000000, in2: 30, out: 4000000000},
        NumIn2Out{in1: 1000000000, in2: 31, out: 2000000000}
    ];
    for f in rots {
        let u = rotate32(f.in1 as u32, f.in2 as u32);
        assert_eq!(u, f.out);
    }
}

#[test]
fn test_mux() {
    let murs = vec![
        // Mur
        NumIn2Out{in1: 100, in2: 100, out: 296331858},
        NumIn2Out{in1: 100, in2: 1000, out: 322382418},
        NumIn2Out{in1: 100, in2: 100000, out: 3309578834},
        NumIn2Out{in1: 100, in2: 1000000, out: 2332043863},
        NumIn2Out{in1: 100, in2: 10000000, out: 1532504573},
        NumIn2Out{in1: 100, in2: 100000000, out: 109586476},
        NumIn2Out{in1: 100, in2: 1000000000, out: 4129826525},
        NumIn2Out{in1: 1000, in2: 100, out: 904209336},
        NumIn2Out{in1: 1000, in2: 1000, out: 909616056},
        NumIn2Out{in1: 1000, in2: 100000, out: 2222629816},
        NumIn2Out{in1: 1000, in2: 1000000, out: 4010714045},
        NumIn2Out{in1: 1000, in2: 10000000, out: 3999704087},
        NumIn2Out{in1: 1000, in2: 100000000, out: 3377899334},
        NumIn2Out{in1: 1000, in2: 1000000000, out: 2218174583},
        NumIn2Out{in1: 100000, in2: 100, out: 193992030},
        NumIn2Out{in1: 100000, in2: 1000, out: 157783390},
        NumIn2Out{in1: 100000, in2: 100000, out: 4239037790},
        NumIn2Out{in1: 100000, in2: 1000000, out: 2456196451},
        NumIn2Out{in1: 100000, in2: 10000000, out: 1388221705},
        NumIn2Out{in1: 100000, in2: 100000000, out: 2402195232},
        NumIn2Out{in1: 100000, in2: 1000000000, out: 3180234409},
        NumIn2Out{in1: 1000000, in2: 100, out: 3909538526},
        NumIn2Out{in1: 1000000, in2: 1000, out: 3946730206},
        NumIn2Out{in1: 1000000, in2: 100000, out: 3622916830},
        NumIn2Out{in1: 1000000, in2: 1000000, out: 1918718691},
        NumIn2Out{in1: 1000000, in2: 10000000, out: 1107645065},
        NumIn2Out{in1: 1000000, in2: 100000000, out: 2509590432},
        NumIn2Out{in1: 1000000, in2: 1000000000, out: 2805288489},
        NumIn2Out{in1: 10000000, in2: 100, out: 1497840043},
        NumIn2Out{in1: 10000000, in2: 1000, out: 1471134123},
        NumIn2Out{in1: 10000000, in2: 100000, out: 1794947499},
        NumIn2Out{in1: 10000000, in2: 1000000, out: 3491281318},
        NumIn2Out{in1: 10000000, in2: 10000000, out: 353417548},
        NumIn2Out{in1: 10000000, in2: 100000000, out: 2900408157},
        NumIn2Out{in1: 10000000, in2: 1000000000, out: 2604713900},
        NumIn2Out{in1: 100000000, in2: 100, out: 3858265169},
        NumIn2Out{in1: 100000000, in2: 1000, out: 3843028049},
        NumIn2Out{in1: 100000000, in2: 100000, out: 2534601809},
        NumIn2Out{in1: 100000000, in2: 1000000, out: 2896622668},
        NumIn2Out{in1: 100000000, in2: 10000000, out: 754906278},
        NumIn2Out{in1: 100000000, in2: 100000000, out: 716108471},
        NumIn2Out{in1: 100000000, in2: 1000000000, out: 567217414},
        NumIn2Out{in1: 1000000000, in2: 100, out: 4215452687},
        NumIn2Out{in1: 1000000000, in2: 1000, out: 4179244047},
        NumIn2Out{in1: 1000000000, in2: 100000, out: 1192047631},
        NumIn2Out{in1: 1000000000, in2: 1000000, out: 32584714},
        NumIn2Out{in1: 1000000000, in2: 10000000, out: 2969121872},
        NumIn2Out{in1: 1000000000, in2: 100000000, out: 107557113},
        NumIn2Out{in1: 1000000000, in2: 1000000000, out: 2529766768}
    ];
    for m in murs {
        let u = mur(m.in1 as u32, m.in2 as u32);
        assert_eq!(u, m.out);
    }
}

#[test]
fn test_fmix() {
    let fmix_in_out = vec![
        NumInOut{in1: 0, out: 0},
        NumInOut{in1: 100, out: 4258159850},
        NumInOut{in1: 1000, out: 1718167128},
        NumInOut{in1: 100000, out: 1391155934},
        NumInOut{in1: 1000000, out: 37787785},
        NumInOut{in1: 10000000, out: 3568206535},
        NumInOut{in1: 100000000, out: 701900797},
        NumInOut{in1: 1000000000, out: 4234498180}
    ];

    for f in fmix_in_out {
        let u = fmix(f.in1 as u32);
        assert_eq!(u, f.out);
    }
}
