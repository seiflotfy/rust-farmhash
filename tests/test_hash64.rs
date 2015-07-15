extern crate farmhash;

use self::farmhash::*;
use std::hash::Hasher;

#[cfg(test)]
struct StrToHash64<'a> {
    value:    &'a str,
    expected: u64
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
    for s in str_to_hash64 {
        let hash = hash64((&s.value).as_bytes());
        if hash != s.expected {
            println!("{}", s.value);
        }
        assert_eq!(hash, s.expected);

        let mut hasher = FarmHasher::default();
        hasher.write((&s.value).as_bytes());
        let hash = hasher.finish();
        if hash != s.expected {
            println!("{}", s.value);
        }
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
    for s in str_to_hash64 {
        let hash = hash64((&s.value).as_bytes());
        if hash != s.expected {
            println!("{}", s.value);
        }
        assert_eq!(hash, s.expected);

        let mut hasher = FarmHasher::default();
        hasher.write((&s.value).as_bytes());
        let hash = hasher.finish();
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
    for s in str_to_hash64 {
        let hash = hash64((&s.value).as_bytes());
        if hash != s.expected {
            println!("{}", s.value);
        }
        assert_eq!(hash, s.expected);

        let mut hasher = FarmHasher::default();
        hasher.write((&s.value).as_bytes());
        let hash = hasher.finish();
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

    for s in str_to_hash64 {
        let hash = hash64((&s.value).as_bytes());
        if hash != s.expected {
            println!("{}", s.value);
        }
        assert_eq!(hash, s.expected);

        let mut hasher = FarmHasher::default();
        hasher.write((&s.value).as_bytes());
        let hash = hasher.finish();
        assert_eq!(hash, s.expected);
    }
}
