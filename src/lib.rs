//! # urbit-q
//!
//! Based on [urbit-ob](https://github.com/urbit/urbit-ob), supports only the
//! `@q` format.
//!
//! ## usage
//!
//! Note that when encoding more than one byte, `encode` pads from the beginning
//! to an even number of bytes (as per the original implementation) and `decode`
//! ignores any dashes or spaces within the string.
//! ```rust
//! urbit_q::encode(&[1]); // nec
//! let string = urbit_q::encode(&[1, 2, 3]); // doznec-binwes
//! urbit_q::decode(&string).unwrap(); // [0, 1, 2, 3]
//! urbit_q::decode("doz nec bin wes"); // Some([0, 1, 2, 3])
//! urbit_q::decode("do-z ne cb inwes"); // Some([0, 1, 2, 3])
//! urbit_q::decode("hello world"); // None
//! ```

mod consts;

/// Encodes data to Urbit's `@q` format
///
/// Note that when encoding more than one byte, it  pads from the beginning to
/// an even number of bytes (as per the original implementation,
/// [urbit-ob](https://github.com/urbit/urbit-ob)), e.g.
/// ```
/// # use urbit_q::*;
/// encode(&[1]); // nec
/// let string = encode(&[1, 2, 3]); // doznec-binwes
/// decode(&string).unwrap(); // [0, 1, 2, 3]
/// ```
pub fn encode(input: &[u8]) -> String {
    if input.len() == 0 {
        return String::new();
    }
    if input.len() == 1 {
        return String::from(consts::SUFFIXES[input[0] as usize]);
    }
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
    let mut bytes = Vec::from(input);
    bytes.retain(|x| *x != ('-' as u8) && *x != (' ' as u8));
    if bytes.len() == 3 {
        bytes[0] = *consts::SUFFIXES_MAP.get(&bytes[..])?;
        bytes.truncate(1);
        return Some(bytes);
    }
    match bytes.len() % 6 {
        0 => {
            for i in (0..bytes.len()).step_by(6) {
                let j = i / 3;
                bytes[j] = *consts::PREFIXES_MAP.get(&bytes[i..i + 3])?;
                bytes[j + 1] = *consts::SUFFIXES_MAP.get(&bytes[i + 3..i + 6])?;
            }
        }
        3 => {
            bytes[0] = *consts::SUFFIXES_MAP.get(&bytes[0..3])?;
            for i in (3..bytes.len()).step_by(6) {
                let j = i / 3;
                bytes[j] = *consts::PREFIXES_MAP.get(&bytes[i..i + 3])?;
                bytes[j + 1] = *consts::SUFFIXES_MAP.get(&bytes[i + 3..i + 6])?;
            }
        }
        _ => return None,
    }
    bytes.truncate(bytes.len() / 3);
    Some(bytes)
}
