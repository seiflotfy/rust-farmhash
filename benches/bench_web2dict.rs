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


#[bench]
fn bench_sip_web2_dict(b: &mut Bencher) {
    let path = Path::new("/usr/share/dict/web2");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   Error::description(&why)),
        Ok(file) => file,
    };


    let mut web2 = String::new();
    match file.read_to_string(&mut web2) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   Error::description(&why)),
        Ok(_) => {},
    }

    b.iter(|| {
        let split: Vec<&str> = web2.split("\n").collect();
        for s in split {
            let mut hasher = SipHasher::default();
            s.hash(&mut hasher);
            hasher.finish();
        }
    });
}

#[bench]
fn bench_fnv_web2_dict(b: &mut Bencher) {
    let path = Path::new("/usr/share/dict/web2");
    let display = path.display();
 
    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   Error::description(&why)),
        Ok(file) => file,
    };


    let mut web2 = String::new();
    match file.read_to_string(&mut web2) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   Error::description(&why)),
        Ok(_) => {},
    }

    b.iter(|| {
        let split: Vec<&str> = web2.split("\n").collect();
        for s in split {
            let mut hasher = fnv::FnvHasher::default();
            s.hash(&mut hasher);
            hasher.finish();
        }
    });
}

#[bench]
fn bench_farm_web2_dict(b: &mut Bencher) {
    let path = Path::new("/usr/share/dict/web2");
    let display = path.display();
 
    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   Error::description(&why)),
        Ok(file) => file,
    };

    let mut web2 = String::new();
    match file.read_to_string(&mut web2) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   Error::description(&why)),
        Ok(_) => {},
    }

    b.iter(|| {
        let split: Vec<&str> = web2.split("\n").collect();
        for s in split {
            let mut hasher = farmhash::FarmHasher::default();
            s.hash(&mut hasher);
            hasher.finish();
        }
    });
}

#[bench]
fn bench_farm_direct_web2_dict(b: &mut Bencher) {
    let path = Path::new("/usr/share/dict/web2");
    let display = path.display();
 
    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   Error::description(&why)),
        Ok(file) => file,
    };

    let mut web2 = String::new();
    match file.read_to_string(&mut web2) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   Error::description(&why)),
        Ok(_) => {},
    }

    b.iter(|| {
        let split: Vec<&str> = web2.split("\n").collect();
        for s in split {
            farmhash::hash64(&s.as_bytes());
        }
    });
}