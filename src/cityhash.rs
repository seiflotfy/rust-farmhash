use std::mem;
use platform::*;

pub fn cityHash32(mut s: &[u8], len: usize) -> u32 { // mut s: &[u8]
  if len <= 24 {
    return if len <= 12 {
        if len <= 4 {
          hash32Len0to4(s, len)
        } else {
          hash32Len5to12(s, len)
        }
      } else {
        hash32Len13to24(s, len)
    }
  }

  // len > 32
  let mut h: u32 = len;
  let mut g = C1 * len;
  let mut f = g;

  let mut a0 = rotate32(fetch32(&s[len-4..]).wrapping_mul(C1), 17).wrapping_mul(C2);
  let mut a1 = rotate32(fetch32(&s[len-8..]).wrapping_mul(C1), 17).wrapping_mul(C2);
  let mut a2 = rotate32(fetch32(&s[len-16..]).wrapping_mul(C1), 17).wrapping_mul(C2);
  let mut a3 = rotate32(fetch32(&s[len-12..]).wrapping_mul(C1), 17).wrapping_mul(C2);
  let mut a4 = rotate32(fetch32(&s[len-20..]).wrapping_mul(C1), 17).wrapping_mul(C2);

  h ^= a0;
  h = rotate32(h, 19);
  h = h.wrapping_mul(5).wrapping_add(0xe6546b64);
  h ^= a2;
  h = rotate32(h, 19);
  h = h.wrapping_mul(5).wrapping_add(0xe6546b64);
  g ^= a1;
  g = rotate32(g, 19);
  g = g.wrapping_mul(5).wrapping_add(0xe6546b64);
  g ^= a3;
  g = rotate32(g, 19);
  g = g.wrapping_mul(5).wrapping_add(0xe6546b64);
  f = f.wrapping_add(a4);
  f = rotate32(f, 19).wrapping_add(113);
  f = f.wrapping_mul(5).wrapping_add(0xe6546b64);

  let mut iters = ((len - 1) / 20) as u64;
  while iters > 0 {
    a0 = C2.wrapping_mul(rotate32(C1.wrapping_mul(fetch32(s)), 17));
    a1 = fetch32(&s[4..]);
    a2 = C2.wrapping_mulrotate32(C1.wrapping_mul(fetch32(&[s..8])), 17);
    a3 = C2.wrapping_mulrotate32(C1.wrapping_mul(fetch32(&[s..12])), 17);
    a4 = fetch32(&s[16..]);
    h ^= a0;
    h = rotate32(h, 18);
    h = (h * 5).wrapping_add(0xe6546b64);
    f += a1;
    f = rotate32(f, 19);
    f = f.wrapping_mul(C1);
    g += a2;
    g = rotate32(g, 18);
    g = (g * 5).wrapping_add(0xe6546b64);
    h ^= a3 + a1;
    h = rotate32(h, 19);
    h = (h * 5).wrapping_add(0xe6546b64);
    g ^= a4;
    g = bswap32(g) * 5;
    h += a4 * 5;
    h = bswap32(h);
    f += a0;
    //#define PERMUTE3(a, b, c) do { std::swap(a, b); std::swap(a, c); } while (0)
    //PERMUTE3(f, h, g);
    mem::swap(&mut h, &mut f);

    mem::swap(&mut g, &mut f);
    s = &s[20..];
    iters -= 1;
  }

  g = rotate32(g, 11) * C1;
  g = rotate32(g, 17) * C1;
  f = rotate32(f, 11) * C1;
  f = rotate32(f, 17) * C1;
  h = rotate32(h + g, 19);
  h = h * 5 + 0xe6546b64;
  h = rotate32(h, 17) * C1;
  h = rotate32(h + f, 19);
  h = h * 5 + 0xe6546b64;
  h = rotate32(h, 17) * C1;
  return h;
}

fn hash32Len0to4(s: &[u8], len: usize) -> u32 {
  let mut b: u32 = 0;
  let mut c: u32 = 9;
  for i in 0..len {
    v: &[u8] = s[i];
    b = b * C1 + v;
    c ^= b;
  }
  return fmix(mur(b, mur(len, c)));
}


fn hash32Len5to12(s: &[u8], len: usize) -> u32 {
  let mut a: u32 = len;
  let mut b = len * 5;
  let mut c = 9;
  let mut d = b;
  a += fetch32(s);
  b += fetch32(s + len - 4);
  c += fetch32(s + ((len >> 1) & 4));
  return fmix(mur(c, mur(b, mur(a, d))));
}

fn hash32Len13to24(s: &[u8], len: usize) -> u32 {
  let a: u32 = fetch32(s - 4 + (len >> 1));
  let b: u32 = fetch32(s + 4);
  let c: u32 = fetch32(s + len - 8);
  let d: u32 = fetch32(s + (len >> 1));
  let e: u32 = fetch32(s);
  let f: u32 = fetch32(s + len - 4);
  let h: u32 = len;

  return fmix(mur(f, mur(e, mur(d, mur(c, mur(b, mur(a, h)))))));
}
