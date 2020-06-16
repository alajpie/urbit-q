//! # urbit-q
//!
//! Based on [urbit-ob](https://github.com/urbit/urbit-ob), supports only the `@q` format.
//!
//! ## usage
//!
//! Note that `encode` pads the beginning to an even number of bytes (as per the
//! original implementation) and `decode` ignores any dashes or spaces within the
//! string.
//! ```rust
//! let bytes: [u8; 3] = [1, 2, 3];
//! let string = urbit_q::encode(&bytes); // doznec-binwes
//! urbit_q::decode(&string).unwrap(); // [0, 1, 2, 3]
//! urbit_q::decode("doz nec bin wes"); // Some([0, 1, 2, 3])
//! urbit_q::decode("do-z ne cb inwes"); // Some([0, 1, 2, 3])
//! urbit_q::decode("hello world"); // None
//! ```

mod consts;

/// Encodes data to Urbit's `@q` format
///
/// Note that it pads the beginning to an even number of bytes (as per the
/// original implementation, [urbit-ob](https://github.com/urbit/urbit-ob)), e.g.
/// ```
/// # use urbit_q::*;
/// let bytes: [u8; 3] = [1, 2, 3];
/// let string = encode(&bytes); // doznec-binwes
/// decode(&string).unwrap(); // [0, 1, 2, 3]
/// ```
pub fn encode(input: &[u8]) -> String {
    let should_pad = input.len() % 2 != 0;
    let length = input.len() + should_pad as usize;
    let dashes = if input.len() > 2 { length / 2 - 1 } else { 0 };
    let capacity = length * 3 + dashes;
    let mut output = String::with_capacity(capacity);
    if should_pad {
        output.push_str(consts::PREFIXES[0]);
    }
    let mut dashes_placed = 0;
    let iter = input.rchunks_exact(2);
    let remainder = iter.remainder();
    if remainder.len() != 0 {
        output.push_str(consts::SUFFIXES[remainder[0] as usize]);
        if dashes_placed != dashes {
            output.push('-');
            dashes_placed += 1;
        }
    }
    for pair in iter.rev() {
        output.push_str(consts::PREFIXES[pair[0] as usize]);
        output.push_str(consts::SUFFIXES[pair[1] as usize]);
        if dashes_placed != dashes {
            output.push('-');
            dashes_placed += 1;
        }
    }
    output
}

/// Decodes data in Urbit's `@q` format
///
/// Note that it ignores any dashes or spaces within the string, e.g.
/// ```
/// # use urbit_q::*;
/// decode("doznec-binwes"); // Some([0, 1, 2, 3])
/// decode("doz nec bin wes"); // Some([0, 1, 2, 3])
/// decode("do-z ne cb inwes"); // Some([0, 1, 2, 3])
/// decode("hello world"); // None
/// ```
pub fn decode(input: &str) -> Option<Vec<u8>> {
    if !input.is_ascii() {
        return None;
    }
    let stripped_input = input.replace(&['-', ' '][..], "");
    if stripped_input.len() % 3 != 0 {
        return None;
    }
    let capacity = stripped_input.len() / 3;
    let mut output: Vec<u8> = Vec::with_capacity(capacity);
    for i in (0..stripped_input.len()).step_by(6) {
        output.push(*consts::PREFIXES_MAP.get(&stripped_input[i..i + 3])?);
        if stripped_input.len() >= i + 6 {
            output.push(*consts::SUFFIXES_MAP.get(&stripped_input[i + 3..i + 6])?);
        }
    }
    Some(output)
}
