# rust-farmhash

Port of Google's Farmhash version 1.1 to pure Rust

For more on Farmhash see https://code.google.com/p/farmhash/

Farmhash is a successor to Cityhash (also from Google). Farmhash, like Cityhash
before it, use ideas from Austin Appleby's MurmurHash.


## Example usage

```rust
extern crate farmhash;

...

let value: &str = "hello world";
let res32 = farmhash::hash32(&value.as_bytes());
// res32 ==> 430397466
let res64 = farmhash::hash64(&value.as_bytes());
// res64 ==> 6381520714923946011
```

## Benchmarks

Tested on /usr/share/dict/web2 on Mac OSX

```
farmhash: required 0.06485s with 0/235887 collisions
fnv:      required 0.12042s with 1/235887 collisions
siphash:  required 0.23546s with 0/235887 collisions
```

### Note

Since FarmHash is not a streaming hash. It is recommended to use the function hash64 or hash32 directly.
Using the hasher interface will give you a different result if you write the same bytearray in chunks before calling finish.

