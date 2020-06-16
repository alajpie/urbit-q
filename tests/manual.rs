#[test]
fn readme() {
    let bytes: [u8; 3] = [1, 2, 3];
    let string = urbit_q::encode(&bytes);
    assert_eq!(string, "doznec-binwes");
    assert_eq!(urbit_q::decode(&string).unwrap(), vec![0, 1, 2, 3]);
    assert_eq!(urbit_q::decode("doz nec bin wes"), Some(vec!(0, 1, 2, 3)));
    assert_eq!(urbit_q::decode("do-z ne cb inwes"), Some(vec!(0, 1, 2, 3)));
    assert_eq!(urbit_q::decode("hello world"), None);
}
