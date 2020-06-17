#[test]
fn blns() {
    for ns in naughty_strings::BLNS {
        urbit_q::decode(ns);
    }
}
