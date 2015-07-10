mod platform;
mod farmhashna;
mod farmhashmk;
mod farmhashna_shared;
mod farmhashcc_shared;
mod farmhashmk_shared;

use farmhashmk::mk_hash32;
use farmhashmk::mk_hash32_with_seed;
use farmhashna::na_hash64;
use farmhashna::na_hash64_with_seed;
use farmhashna::na_hash64_with_seeds;

pub fn hash32(s: &[u8]) -> u32 {
    return mk_hash32(s);
}

pub fn hash32_with_seed(s: &[u8], seed: u32) -> u32 {
    return mk_hash32_with_seed(s, seed);
}

pub fn hash64(s: &[u8]) -> u64 {
    return na_hash64(s);
}

pub fn hash64_with_seed(s: &[u8], seed: u64) -> u64 {
    return na_hash64_with_seed(s, seed);
}

pub fn hash64_with_seeds(s: &[u8], seed0: u64, seed1: u64) -> u64 {
    return na_hash64_with_seeds(s, seed0, seed1);
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
        StrToHash32{value:"hi", expected: 4063302914},
        StrToHash32{value:"hello world", expected: 4181326496},
    ];
    for s in str_to_hash32 {
        if s.value.len() > 4 {
            continue;
        }
        let hash = hash32((&s.value).as_bytes());
        assert_eq!(hash, s.expected);
    }
}

#[test]
fn test_hash_32() {
    let str_to_hash32 = vec![
        // Hash32
        StrToHash32{value: "", expected: 0xdc56d17a},
        StrToHash32{value: "a", expected: 0x3c973d4d},
        StrToHash32{value: "hi", expected: 0xf2311502},
        StrToHash32{value: "hello world", expected: 0x19a7581a},
        StrToHash32{value: "lee@lmmrtech.com", expected: 0xaf0a30fe},
        StrToHash32{value: "docklandsman@gmail.com", expected: 0x5d8cdbf4},
        StrToHash32{value: "fred@example.com", expected: 0x7acdc357},
        StrToHash32{value: "Go is a tool for managing Go source code.Usage:	go command [arguments]The commands are:    build       compile packages and dependencies    clean       remove object files    env         print Go environment information    fix         run go tool fix on packages    fmt         run gofmt on package sources    generate    generate Go files by processing source    get         download and install packages and dependencies    install     compile and install packages and dependencies    list        list packages    run         compile and run Go program    test        test packages    tool        run specified go tool    version     print Go version    vet         run go tool vet on packagesUse go help [command] for more information about a command.Additional help topics:    c           calling between Go and C    filetype    file types    gopath      GOPATH environment variable    importpath  import path syntax    packages    description of package lists    testflag    description of testing flags    testfunc    description of testing functionsUse go help [topic] for more information about that topic.",  expected: 0x9c8f96f3},
    ];
    for s in str_to_hash32 {
        let hash = hash32((&s.value).as_bytes());
        assert_eq!(hash, s.expected);
    }
}

#[test]
fn test_hash_64() {
    let str_to_hash64 = vec![
        // Hash32
        StrToHash64{value: "", expected: 0x9ae16a3b2f90404f},
        StrToHash64{value: "a", expected: 0xb3454265b6df75e3},
        StrToHash64{value: "hi", expected: 0x6a5d2fba44f012f8},
        StrToHash64{value: "hello world", expected: 0x588fb7478bd6b01b},
        StrToHash64{value: "lee@lmmrtech.com", expected: 0x61bec68db00fa2ff},
        StrToHash64{value: "docklandsman@gmail.com", expected: 0xb678cf3842309f40},
        StrToHash64{value: "fred@example.com", expected: 0x7fbbcd6191d8dce0},
        StrToHash64{value: "Go is a tool for managing Go source code.Usage:	go command [arguments]The commands are:    build       compile packages and dependencies    clean       remove object files    env         print Go environment information    fix         run go tool fix on packages    fmt         run gofmt on package sources    generate    generate Go files by processing source    get         download and install packages and dependencies    install     compile and install packages and dependencies    list        list packages    run         compile and run Go program    test        test packages    tool        run specified go tool    version     print Go version    vet         run go tool vet on packagesUse go help [command] for more information about a command.Additional help topics:    c           calling between Go and C    filetype    file types    gopath      GOPATH environment variable    importpath  import path syntax    packages    description of package lists    testflag    description of testing flags    testfunc    description of testing functionsUse go help [topic] for more information about that topic.",  expected: 0xafe256550e4567c9},
    ];
    for s in str_to_hash64 {
        println!("{}", s.value);
        let hash = hash64((&s.value).as_bytes());
        assert_eq!(hash, s.expected);
    }
}
