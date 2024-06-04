use devutils::utils;

#[test]
fn hex_0() {
    assert_eq!("0x0".to_string(), utils::number_in_base(0, 16));
}

#[test]
fn bin_0() {
    assert_eq!("0b0".to_string(), utils::number_in_base(0, 2));
}

#[test]
fn octal_0() {
    assert_eq!("0o0".to_string(), utils::number_in_base(0, 8));
}

#[test]
fn dec_0() {
    assert_eq!("0", utils::number_in_base(0, 10));
}

#[test]
fn bin_2() {
    assert_eq!("0b10".to_string(), utils::number_in_base(2, 2));
}

#[test]
fn octal_2() {
    assert_eq!("0o2".to_string(), utils::number_in_base(2, 8));
}

#[test]
fn dec_2() {
    assert_eq!("2".to_string(), utils::number_in_base(2, 10));
}

#[test]
fn hex_2() {
    assert_eq!("0x2".to_string(), utils::number_in_base(2, 16));
}

#[test]
fn bin_15() {
    assert_eq!("0b1111".to_string(), utils::number_in_base(15, 2));
}

#[test]
fn octal_15() {
    assert_eq!("0o17", utils::number_in_base(15, 8));
}
#[test]
fn dec_15() {
    assert_eq!("15".to_string(), utils::number_in_base(15, 10));
}
#[test]
fn hex_15() {
    assert_eq!("0xF".to_string(), utils::number_in_base(15, 16));
}
