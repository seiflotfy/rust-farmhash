#![feature(test)]
extern crate test;
extern crate farmhash;
extern crate fnv;

use self::farmhash::*;
use self::test::Bencher;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::error::Error;
use std::hash::{Hash, SipHasher, Hasher};
use test::black_box;

// Macros to insert into other macros to test with Hash trait or with a direct
// function
macro_rules! hash_hashing {
    ($data:ident, $hasher:expr) => {{
        let mut hasher = $hasher;
        $data.hash(&mut hasher);
        black_box(hasher.finish());
    }}
}

macro_rules! direct_hashing_str {
    ($data:ident, $hasher:expr) => {{
        black_box($hasher(&$data.as_bytes()));
    }}
}

macro_rules! direct_hashing_u8 {
    ($data:ident, $hasher:expr) => {{
        black_box($hasher($data));
    }}
}

// Dictonary benchmark
macro_rules! dict_bench {
    ($fun:ident, $hashing:ident, $hasher:expr) => {
        #[bench]
        fn $fun(b: &mut Bencher) {
            let path = Path::new("benches/sample-dict");
            let display = path.display();

            // Open file in read-only mode
            let mut file = match File::open(&path) {
                Err(e) => panic!("Couldn't open '{}': {}", display, e),
                Ok(file) => file,
            };

            // Read all contents to string
            let mut dict = String::new();
            if let Err(e) = file.read_to_string(&mut dict) {
                panic!("Couldn't read '{}': {}", display, e);
            }

            b.iter(|| {
                for s in dict.split('\n') {
                    $hashing!(s, $hasher)
                }
            });
        }
    }
}

dict_bench!(bench_dict_sip, hash_hashing, SipHasher::default());
dict_bench!(bench_dict_fnv, hash_hashing, fnv::FnvHasher::default());
dict_bench!(bench_dict_farm, hash_hashing, farmhash::FarmHasher::default());
dict_bench!(bench_dict_farm_direct, direct_hashing_str, farmhash::hash64);


// Lorem Ipsum benchmark
macro_rules! lorem_bench {
    ($fun:ident, $hashing:ident, $hasher:expr) => {
        #[bench]
        fn $fun(b: &mut Bencher) {
            let data = ["Lorem", "ipsum", "dolor", "sit", "amet,", "consetetur",
                        "sadipscing", "elitr,", "sed", "diam", "nonumy",
                        "eirmod", "tempor", "invidunt", "ut", "labore", "et",
                        "dolore", "magna", "aliquyam", "erat,", "sed", "diam",
                        "voluptua."];

            b.iter(|| {
                for s in &data {
                    $hashing!(s, $hasher)
                }
            });
        }
    }
}

lorem_bench!(bench_lorem_sip, hash_hashing, SipHasher::default());
lorem_bench!(bench_lorem_fnv, hash_hashing, fnv::FnvHasher::default());
lorem_bench!(bench_lorem_farm, hash_hashing, farmhash::FarmHasher::default());
lorem_bench!(bench_lorem_farm_direct, direct_hashing_str, farmhash::hash64);


// Pseudo random data benchmark
macro_rules! lorem_bench {
    ($fun:ident, $chunk:expr, $hashing:ident, $hasher:expr) => {
        #[bench]
        fn $fun(b: &mut Bencher) {
            let path = Path::new("benches/pseudo-random-data.bin");
            let display = path.display();

            // Open file in read-only mode
            let mut file = match File::open(&path) {
                Err(e) => panic!("Couldn't open '{}': {}", display, e),
                Ok(file) => file,
            };

            // Read all contents to string
            let mut data = Vec::new();
            if let Err(e) = file.read_to_end(&mut data) {
                panic!("Couldn't read '{}': {}", display, e);
            }

            b.iter(|| {
                for s in data.chunks($chunk) {
                    $hashing!(s, $hasher)
                }
            });
        }
    }
}

lorem_bench!(bench_pseurand_big_sip, 512, hash_hashing, SipHasher::default());
lorem_bench!(bench_pseurand_big_fnv, 512, hash_hashing, fnv::FnvHasher::default());
lorem_bench!(bench_pseurand_big_farm, 512, hash_hashing, farmhash::FarmHasher::default());
lorem_bench!(bench_pseurand_big_farm_direct, 512, direct_hashing_u8, farmhash::hash64);

lorem_bench!(bench_pseurand_small_sip, 4, hash_hashing, SipHasher::default());
lorem_bench!(bench_pseurand_small_fnv, 4, hash_hashing, fnv::FnvHasher::default());
lorem_bench!(bench_pseurand_small_farm, 4, hash_hashing, farmhash::FarmHasher::default());
lorem_bench!(bench_pseurand_small_farm_direct, 4, direct_hashing_u8, farmhash::hash64);
