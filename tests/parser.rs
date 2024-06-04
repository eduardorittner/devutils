use devutils::utils;

#[test]
fn hex_0() {
    assert_eq!(Some(0), utils::parse_number("0x0"));
}

#[test]
fn bin_0() {
    assert_eq!(Some(0), utils::parse_number("0b0"));
}

#[test]
fn octal_0() {
    assert_eq!(Some(0), utils::parse_number("0o0"));
}

#[test]
fn dec_0() {
    assert_eq!(Some(0), utils::parse_number("0"));
}

#[test]
fn dec_lots_of_0s() {
    assert_eq!(Some(0), utils::parse_number("0000000"));
}

#[test]
fn bin_2() {
    assert_eq!(Some(2), utils::parse_number("0b10"));
}

#[test]
fn octal_2() {
    assert_eq!(Some(2), utils::parse_number("0o2"));
}

#[test]
fn dec_2() {
    assert_eq!(Some(2), utils::parse_number("2"));
}

#[test]
fn hex_2() {
    assert_eq!(Some(2), utils::parse_number("0x2"));
}

#[test]
fn bin_15() {
    assert_eq!(Some(15), utils::parse_number("0b1111"));
}

#[test]
fn octal_15() {
    assert_eq!(Some(15), utils::parse_number("0o17"));
}
#[test]
fn dec_15() {
    assert_eq!(Some(15), utils::parse_number("15"));
}
#[test]
fn hex_15() {
    assert_eq!(Some(15), utils::parse_number("0xF"));
}
