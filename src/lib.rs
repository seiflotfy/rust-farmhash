mod platform;
mod farmhashna;
mod farmhashmk;
mod farmhashuo;
mod farmhashxo;
mod farmhashna_shared;
mod farmhashcc_shared;
mod farmhashmk_shared;

use farmhashmk::mk_hash32;
use farmhashmk::mk_hash32_with_seed;
use farmhashxo::xo_hash64;
use farmhashxo::xo_hash64_with_seed;
use farmhashxo::xo_hash64_with_seeds;
use std::hash::Hasher;

/// Create a new farmhash based u32 for a gives array of bytes.
///
/// # Examples
///
/// ```
/// let value: &str = "hello world";
/// let res32 = farmhash::hash32(&value.as_bytes());
/// //res32 ==> 430397466
/// ```
pub fn hash32(s: &[u8]) -> u32 {
    return mk_hash32(s);
}
/// Create a new farmhash based u32 for a gives array of bytes with a given seed.
pub fn hash32_with_seed(s: &[u8], seed: u32) -> u32 {
    return mk_hash32_with_seed(s, seed);
}

/// Create a new farmhash based u64 for a gives array of bytes.
///
/// # Examples
///
/// ```
/// let value: &str = "hello world";
/// let res64 = farmhash::hash64(&value.as_bytes());
/// //res64 ==> 6381520714923946011
/// ```
pub fn hash64(s: &[u8]) -> u64 {
    return xo_hash64(s);
}

/// Create a new farmhash based u64 for a gives array of bytes with a given seed.
pub fn hash64_with_seed(s: &[u8], seed: u64) -> u64 {
    return xo_hash64_with_seed(s, seed);
}

/// Create a new farmhash based u64 for a gives array of bytes with 2 given seeds.
pub fn hash64_with_seeds(s: &[u8], seed0: u64, seed1: u64) -> u64 {
    return xo_hash64_with_seeds(s, seed0, seed1);
}


pub struct FarmHasher {
    bytes: Vec<u8>
}

impl FarmHasher {
    /// Creates a new `FarmHasher`.
    #[inline]
    pub fn new() -> FarmHasher {
        let mut h = FarmHasher{bytes: vec![]};
        h.reset();
        return h;
    }

    #[inline]
    fn reset(&mut self) {
        self.bytes.clear();
    }

    #[inline]
    fn write(&mut self, msg: &[u8]) {
        for b in msg {
            self.bytes.push(*b);
        }
    }

}

impl Hasher for FarmHasher {
    #[inline]
    fn write(&mut self, msg: &[u8]) {
        self.write(msg)
    }

    #[inline]
    fn finish(&self) -> u64 {
        hash64(&self.bytes[..])
    }
}

#[cfg(test)]
struct StrToHash32<'a> {
    value:    &'a str,
    expected: u32
}

#[cfg(test)]
struct StrToHash64<'a> {
    value:    &'a str,
    expected: u64
}

#[test]
fn test_hash32_len_0_to_4() {
    let str_to_hash32 = vec![
        // Hash32
        StrToHash32{expected: 0xdc56d17a, value: ""},
        StrToHash32{expected: 0x3c973d4d, value: "a"},
        StrToHash32{expected: 0x417330fd, value: "ab"},
        StrToHash32{expected: 0x2f635ec7, value: "abc"},
        StrToHash32{expected: 0x98b51e95, value: "abcd"},
        StrToHash32{expected: 0xf2311502, value: "hi"},
    ];
    for s in str_to_hash32 {
        let hash = hash32((&s.value).as_bytes());
        if hash != s.expected {
            println!("{}", s.value);
        }
        assert_eq!(hash, s.expected);
    }
}


#[test]
fn test_hash32_len_5_to_12() {
    let str_to_hash32 = vec![
        // Hash32
        StrToHash32{expected: 0xa3f366ac, value: "abcde"},
        StrToHash32{expected: 0x0f813aa4, value: "abcdef"},
        StrToHash32{expected: 0x21deb6d7, value: "abcdefg"},
        StrToHash32{expected: 0xfd7ec8b9, value: "abcdefgh"},
        StrToHash32{expected: 0x6f98dc86, value: "abcdefghi"},
        StrToHash32{expected: 0xf2669361, value: "abcdefghij"},
        StrToHash32{expected: 0x19a7581a, value: "hello world"},
    ];
    for s in str_to_hash32 {
        let hash = hash32((&s.value).as_bytes());
        if hash != s.expected {
            println!("{}", s.value);
        }
        assert_eq!(hash, s.expected);
    }
}

#[test]
fn test_hash32_len_13_to_24() {
    let str_to_hash32 = vec![
        // Hash32
        StrToHash32{expected: 0x7acdc357, value: "fred@example.com"},
        StrToHash32{expected: 0xaf0a30fe, value: "lee@lmmrtech.com"},
        StrToHash32{expected: 0x5d8cdbf4, value: "docklandsman@gmail.com"},
        StrToHash32{expected: 0xc6246b8d, value: "size:  a.out:  bad magic"},
    ];
    for s in str_to_hash32 {
        let hash = hash32((&s.value).as_bytes());
        if hash != s.expected {
            println!("{}", s.value);
        }
        assert_eq!(hash, s.expected);
    }
}


#[test]
fn test_hash32_else() {
    let str_to_hash32 = vec![
        // Hash32
        StrToHash32{expected: 0x9c8f96f3, value: "Go is a tool for managing Go source code.Usage:	go command [arguments]The commands are:    build       compile packages and dependencies    clean       remove object files    env         print Go environment information    fix         run go tool fix on packages    fmt         run gofmt on package sources    generate    generate Go files by processing source    get         download and install packages and dependencies    install     compile and install packages and dependencies    list        list packages    run         compile and run Go program    test        test packages    tool        run specified go tool    version     print Go version    vet         run go tool vet on packagesUse go help [command] for more information about a command.Additional help topics:    c           calling between Go and C    filetype    file types    gopath      GOPATH environment variable    importpath  import path syntax    packages    description of package lists    testflag    description of testing flags    testfunc    description of testing functionsUse go help [topic] for more information about that topic."},
        StrToHash32{expected: 0xe273108f, value: "Discard medicine more than two years old."},
        StrToHash32{expected: 0xf585dfc4, value: "He who has a shady past knows that nice guys finish last."},
        StrToHash32{expected: 0x363394d1, value: "I wouldn't marry him with a ten foot pole."},
        StrToHash32{expected: 0x7613810f, value: "Free! Free!/A trip/to Mars/for 900/empty jars/Burma Shave"},
        StrToHash32{expected: 0x2cc30bb7, value: "The days of the digital watch are numbered.  -Tom Stoppard"},
        StrToHash32{expected: 0x322984d9, value: "Nepal premier won't resign."},
        StrToHash32{expected: 0xa5812ac8, value: "For every action there is an equal and opposite government program."},
        StrToHash32{expected: 0x1090d244, value: "His money is twice tainted: 'taint yours and 'taint mine."},
        StrToHash32{expected: 0xff16c9e6, value: "There is no reason for any individual to have a computer in their home. -Ken Olsen, 1977"},
        StrToHash32{expected: 0xcc3d0ff2, value: "It's a tiny change to the code and not completely disgusting. - Bob Manchek"},
        StrToHash32{expected: 0xd225e92e, value: "The major problem is with sendmail.  -Mark Horton"},
        StrToHash32{expected: 0x1b8db5d0, value: "Give me a rock, paper and scissors and I will move the world.  CCFestoon"},
        StrToHash32{expected: 0x4fda5f07, value: "If the enemy is within range, then so are you."},
        StrToHash32{expected: 0x2e18e880, value: "It's well we cannot hear the screams/That we create in others' dreams."},
        StrToHash32{expected: 0xd07de88f, value: "You remind me of a TV show, but that's all right: I watch it anyway."},
        StrToHash32{expected: 0x221694e4, value: "C is as portable as Stonehedge!!"},
        StrToHash32{expected: 0xe2053c2c, value: "Even if I could be Shakespeare, I think I should still choose to be Faraday. - A. Huxley"},
        StrToHash32{expected: 0x11c493bb, value: "The fugacity of a constituent in a mixture of gases at a given temperature is proportional to its mole fraction.  Lewis-Randall Rule"},
        StrToHash32{expected: 0x0819a4e8, value: "How can you write a big system without C++?  -Paul Glick"}
    ];
    for s in str_to_hash32 {
        let hash = hash32((&s.value).as_bytes());
        if hash != s.expected {
            println!("{}", s.value);
        }
        assert_eq!(hash, s.expected);
    }
}
#[test]
fn test_hash64_0_to_16() {
    let str_to_hash64 = vec![
        // Hash32
        StrToHash64{expected: 0x9ae16a3b2f90404f, value: ""},
        StrToHash64{expected: 0xb3454265b6df75e3, value: "a"},
        StrToHash64{expected: 0xaa8d6e5242ada51e, value: "ab"},
        StrToHash64{expected: 0x24a5b3a074e7f369, value: "abc"},
        StrToHash64{expected: 0x1a5502de4a1f8101, value: "abcd"},
        StrToHash64{expected: 0xc22f4663e54e04d4, value: "abcde"},
        StrToHash64{expected: 0xc329379e6a03c2cd, value: "abcdef"},
        StrToHash64{expected: 0x3c40c92b1ccb7355, value: "abcdefg"},
        StrToHash64{expected: 0xfee9d22990c82909, value: "abcdefgh"},
        StrToHash64{expected: 0x332c8ed4dae5ba42, value: "abcdefghi"},
        StrToHash64{expected: 0x8a3abb6a5f3fb7fb, value: "abcdefghij"},
        StrToHash64{expected: 0x6a5d2fba44f012f8, value: "hi"},
        StrToHash64{expected: 0x588fb7478bd6b01b, value: "hello world"},
        StrToHash64{expected: 0x61bec68db00fa2ff, value: "lee@lmmrtech.com"},
        StrToHash64{expected: 0x7fbbcd6191d8dce0, value: "fred@example.com"},
        StrToHash64{expected: 0x80d73b843ba57db8, value: "size:  a.out:  bad magic"},
    ];
    let mut hasher = FarmHasher::new();
    for s in str_to_hash64 {
        let hash = hash64((&s.value).as_bytes());
        if hash != s.expected {
            println!("{}", s.value);
        }
        assert_eq!(hash, s.expected);
        hasher.write((&s.value).as_bytes());
        let hash = hasher.finish();
        hasher.reset();
        assert_eq!(hash, s.expected);
    }
}

#[test]
fn test_hash64_17_to_32() {
    let str_to_hash64 = vec![
        // Hash32
        StrToHash64{expected: 0xb678cf3842309f40, value: "docklandsman@gmail.com"},
        StrToHash64{expected: 0x8eb3808d1ccfc779, value: "Nepal premier won't resign."},
        StrToHash64{expected: 0xb944f8a16261e414, value: "C is as portable as Stonehedge!!"}
    ];
    let mut hasher = FarmHasher::new();
    for s in str_to_hash64 {
        let hash = hash64((&s.value).as_bytes());
        if hash != s.expected {
            println!("{}", s.value);
        }
        assert_eq!(hash, s.expected);
        hasher.write((&s.value).as_bytes());
        let hash = hasher.finish();
        hasher.reset();
        assert_eq!(hash, s.expected);
    }
}

#[test]
fn test_hash64_33_to_64() {
    let str_to_hash64 = vec![
        // Hash32
        StrToHash64{expected: 0x2d072041b535155d, value: "Discard medicine more than two years old."}, //41
        StrToHash64{expected: 0x9f9e3cdeb570f926, value: "He who has a shady past knows that nice guys finish last."},
        StrToHash64{expected: 0x361b79df08615cd6, value: "I wouldn't marry him with a ten foot pole."},
        StrToHash64{expected: 0xdcfb73d4de1111c6, value: "Free! Free!/A trip/to Mars/for 900/empty jars/Burma Shave"},
        StrToHash64{expected: 0xd71bdfedb6182a5d, value: "The days of the digital watch are numbered.  -Tom Stoppard"},
        StrToHash64{expected: 0x3df4b8e109629602, value: "His money is twice tainted: 'taint yours and 'taint mine."},
        StrToHash64{expected: 0x1da6c1dfec23a597, value: "The major problem is with sendmail.  -Mark Horton"},
        StrToHash64{expected: 0x1f232f3375914f0a, value: "If the enemy is within range, then so are you."},
        StrToHash64{expected: 0xa29944470950e8e4, value: "How can you write a big system without C++?  -Paul Glick"}
    ];
    let mut hasher = FarmHasher::new();
    for s in str_to_hash64 {
        let hash = hash64((&s.value).as_bytes());
        if hash != s.expected {
            println!("{}", s.value);
        }
        assert_eq!(hash, s.expected);
        hasher.write((&s.value).as_bytes());
        let hash = hasher.finish();
        hasher.reset();
        assert_eq!(hash, s.expected);
    }
}

#[test]
fn test_hash64_else() {
    let str_to_hash64 = vec![
        // Hash32
        StrToHash64{expected: 0x8452fbb0c8f98c4f, value: "For every action there is an equal and opposite government program."},
        StrToHash64{expected: 0x7fee06e367562d44, value: "There is no reason for any individual to have a computer in their home. -Ken Olsen, 1977"},
        StrToHash64{expected: 0x889b024bab17bf54, value: "It's a tiny change to the code and not completely disgusting. - Bob Manchek"},
        StrToHash64{expected: 0xb8e2918a4398348d, value: "Give me a rock, paper and scissors and I will move the world.  CCFestoon"},
        StrToHash64{expected: 0x796229f1faacec7e, value: "It's well we cannot hear the screams/That we create in others' dreams."},
        StrToHash64{expected: 0x98d2fbd5131a5860, value: "You remind me of a TV show, but that's all right: I watch it anyway."},
        StrToHash64{expected: 0x4c349a4ff7ac0c89, value: "Even if I could be Shakespeare, I think I should still choose to be Faraday. - A. Huxley"},
        StrToHash64{expected: 0x98eff6958c5e91a,  value: "The fugacity of a constituent in a mixture of gases at a given temperature is proportional to its mole fraction.  Lewis-Randall Rule"},
        StrToHash64{expected: 0x21609f6764c635ed, value: "Go is a tool for managing Go source code.Usage: go command [arguments]The commands are:    build       compile packages and dependencies    clean       remove object files    env         print Go environment information    fix         run go tool fix on packages    fmt         run gofmt on package sources    generate    generate Go files by processing source    get         download and install packages and dependencies    install     compile and install packages and dependencies    list        list packages    run         compile and run Go program    test        test packages    tool        run specified go tool    version     print Go version    vet         run go tool vet on packagesUse go help [command] for more information about a command.Additional help topics:    c           calling between Go and C    filetype    file types    gopath      GOPATH environment variable    importpath  import path syntax    packages    description of package lists    testflag    description of testing flags    testfunc    description of testing functionsUse go help [topic] for more information about that topic."},
    ];
    let mut hasher = FarmHasher::new();
    for s in str_to_hash64 {
        let hash = hash64((&s.value).as_bytes());
        if hash != s.expected {
            println!("{}", s.value);
        }
        assert_eq!(hash, s.expected);
        hasher.write((&s.value).as_bytes());
        let hash = hasher.finish();
        hasher.reset();
        assert_eq!(hash, s.expected);
    }
}
