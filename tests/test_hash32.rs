extern crate farmhash;

use self::farmhash::*;

#[cfg(test)]
struct StrToHash32<'a> {
    value:    &'a str,
    expected: u32
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