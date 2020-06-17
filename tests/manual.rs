#[test]
fn readme() {
    assert_eq!(urbit_q::encode(&[1]), "nec");
    let string = urbit_q::encode(&[1, 2, 3]);
    assert_eq!(string, "doznec-binwes");
    assert_eq!(urbit_q::decode(&string).unwrap(), vec![0, 1, 2, 3]);
    assert_eq!(urbit_q::decode("doz nec bin wes"), Some(vec!(0, 1, 2, 3)));
    assert_eq!(urbit_q::decode("do-z ne cb inwes"), Some(vec!(0, 1, 2, 3)));
    assert_eq!(urbit_q::decode("hello world"), None);
}

#[test]
fn simple() {
    assert_eq!(urbit_q::encode(&[]), "");
    assert_eq!(urbit_q::decode(""), Some(vec![]));
    assert_eq!(urbit_q::encode(&[0]), "zod");
    assert_eq!(urbit_q::decode("zod"), Some(vec![0]));
    assert_eq!(urbit_q::encode(&[1]), "nec");
    assert_eq!(urbit_q::decode("nec"), Some(vec![1]));
    assert_eq!(urbit_q::encode(&[1, 2]), "marbud");
    assert_eq!(urbit_q::decode("marbud"), Some(vec![1, 2]));
    assert_eq!(urbit_q::encode(&[1, 2, 3]), "doznec-binwes");
    assert_eq!(urbit_q::decode("doznec-binwes"), Some(vec![0, 1, 2, 3]));
    assert_eq!(urbit_q::decode("nec-binwes"), Some(vec![1, 2, 3]));
    assert_eq!(urbit_q::encode(&[1, 2, 3, 4]), "marbud-wansev");
    assert_eq!(urbit_q::decode("marbud-wansev"), Some(vec![1, 2, 3, 4]));
    assert_eq!(urbit_q::encode(&[1, 2, 3, 4, 5]), "doznec-binwes-samper");
    assert_eq!(
        urbit_q::decode("doznec-binwes-samper"),
        Some(vec![0, 1, 2, 3, 4, 5])
    );
    assert_eq!(
        urbit_q::decode("nec-binwes-samper"),
        Some(vec![1, 2, 3, 4, 5])
    );
}
