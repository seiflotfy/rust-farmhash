/*
mod tests {
    #![feature(test)]
    extern crate test;
    extern crate time;

    use super::*;
    use self::test::Bencher;
    use std::fs::File;
    use std::io::prelude::*;
    use std::path::Path;
    use self::time::precise_time_s;
    use std::error::Error;

    #[bench]
    fn bench_web2_dict(b: &mut Bencher) {
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

        {
            let split: Vec<&str> = web2.split("\n").collect();
            let split_len = split.len();
            let mut v: Vec<u64> = vec![];
            let mut needed_time: f64 = 0.0;
     
            for s in split {
                let starttime = precise_time_s();
                let res = farmhash::hash64(&s.as_bytes());
                needed_time += precise_time_s() - starttime;
                v.push(res);
            }
            v.sort();
            v.dedup();
            let dedup_percent = split_len - v.len();
            println!("farmhash required {} s with {}/{} collisions", needed_time, dedup_percent, split_len);
        }

    }
}
*/