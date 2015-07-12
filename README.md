# rust-farmhash

Port of Google's Farmhash version 1.1 to pure Rust

For more on Farmhash see https://code.google.com/p/farmhash/

Farmhash is a successor to Cityhash (also from Google). Farmhash, like Cityhash
before it, use ideas from Austin Appleby's MurmurHash.


## Example usage

'''
extern crate farmhash;

...

let value: &str = "hello world";
let res32 = farmhash::hash32(&value.as_bytes());
// res32 ==> 430397466
let res64 = farmhash::hash64(&value.as_bytes());
// res64 ==> 6381520714923946011
'''


#### Note:
This has been translated from [gofarmhash](https://github.com/leemcloughlin/gofarmhash) with minimal changes
