use proptest::prelude::*;
use urbit_q::*;

proptest! {
    #[test]
    fn doesnt_crash(s in "\\PC*") {
        decode(&s);
    }
    #[test]
    fn doesnt_crash_ascii(s in "[a-z- ]*") {
       decode(&s);
    }

    #[test]
    fn roundtrip(bytes: Vec<u8>) {
        let mut padded = bytes.clone();
        if padded.len() != 1 && padded.len() % 2 != 0 {
           padded.insert(0, 0);
        }
        let tripped = decode(&encode(&bytes)).unwrap();
        assert_eq!(tripped, padded);
    }
}
