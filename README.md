# urbit-q
[![](https://img.shields.io/crates/v/urbit-q.svg)](https://crates.io/crates/urbit-q)
[![](https://docs.rs/urbit-q/badge.svg)](https://docs.rs/urbit-q)
[![](https://travis-ci.org/k2l8m11n2/urbit-q.svg?branch=master)](https://travis-ci.org/github/k2l8m11n2/urbit-q)


Based on [urbit-ob](https://github.com/urbit/urbit-ob), supports only the `@q`
format.

## usage

Note that when encoding more than one byte, `encode` pads from the beginning to
an even number of bytes (as per the original implementation) and `decode`
ignores any dashes, tildes or spaces within the string.
```rust
urbit_q::encode(&[1]); // nec
let string = urbit_q::encode(&[1, 2, 3]); // doznec-binwes
urbit_q::decode(&string).unwrap(); // [0, 1, 2, 3]
urbit_q::decode("doz nec bin wes"); // Some([0, 1, 2, 3])
urbit_q::decode("do-z ne cb~inwes"); // Some([0, 1, 2, 3])
urbit_q::decode("nec-binwes"); // Some([1, 2, 3])
urbit_q::decode("hello world"); // None
```
