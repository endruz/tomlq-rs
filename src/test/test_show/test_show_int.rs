use crate::query_toml_value;

#[test]
/// 测试查询展示十进制整数
fn test_show_decimal() {
    let toml_str = r##"
    int1 = +99
    int2 = 42
    int3 = 0
    int4 = -17
    int5 = 1_000
    int6 = 5_349_221
    int7 = 53_49_221  # Indian number system grouping
    int8 = 1_2_3_4_5  # VALID but discouraged
    "##;
    let mut key: &str;
    let mut expected: String;
    let mut actual: String;
    println!("{}", toml_str);
    // query int1
    key = "int1";
    expected = String::from("99");
    actual = query_toml_value(toml_str, key).unwrap();
    assert_eq!(
        expected, actual,
        r#"The value of the key "{}" is wrong, it should be "{}", the result is "{}""#,
        key, expected, actual,
    );
    // query int2
    key = "int2";
    expected = String::from("42");
    actual = query_toml_value(toml_str, key).unwrap();
    assert_eq!(
        expected, actual,
        r#"The value of the key "{}" is wrong, it should be "{}", the result is "{}""#,
        key, expected, actual,
    );
    // query int3
    key = "int3";
    expected = String::from("0");
    actual = query_toml_value(toml_str, key).unwrap();
    assert_eq!(
        expected, actual,
        r#"The value of the key "{}" is wrong, it should be "{}", the result is "{}""#,
        key, expected, actual,
    );
    // query int4
    key = "int4";
    expected = String::from("-17");
    actual = query_toml_value(toml_str, key).unwrap();
    assert_eq!(
        expected, actual,
        r#"The value of the key "{}" is wrong, it should be "{}", the result is "{}""#,
        key, expected, actual,
    );
    // query int5
    key = "int5";
    expected = String::from("1000");
    actual = query_toml_value(toml_str, key).unwrap();
    assert_eq!(
        expected, actual,
        r#"The value of the key "{}" is wrong, it should be "{}", the result is "{}""#,
        key, expected, actual,
    );
    // query int6
    key = "int6";
    expected = String::from("5349221");
    actual = query_toml_value(toml_str, key).unwrap();
    assert_eq!(
        expected, actual,
        r#"The value of the key "{}" is wrong, it should be "{}", the result is "{}""#,
        key, expected, actual,
    );
    // query int7
    key = "int7";
    expected = String::from("5349221");
    actual = query_toml_value(toml_str, key).unwrap();
    assert_eq!(
        expected, actual,
        r#"The value of the key "{}" is wrong, it should be "{}", the result is "{}""#,
        key, expected, actual,
    );
    // query int8
    key = "int8";
    expected = String::from("12345");
    actual = query_toml_value(toml_str, key).unwrap();
    assert_eq!(
        expected, actual,
        r#"The value of the key "{}" is wrong, it should be "{}", the result is "{}""#,
        key, expected, actual,
    );
}

#[test]
/// 测试查询展示十六进制整数
fn test_show_hexadecimal() {
    let toml_str = r##"
    # hexadecimal with prefix `0x`
    hex1 = 0xDEADBEEF
    hex2 = 0xdeadbeef
    hex3 = 0xdead_beef
    "##;
    let mut key: &str;
    let mut expected: String;
    let mut actual: String;
    println!("{}", toml_str);
    // query hex1
    key = "hex1";
    expected = String::from("3735928559");
    actual = query_toml_value(toml_str, key).unwrap();
    assert_eq!(
        expected, actual,
        r#"The value of the key "{}" is wrong, it should be "{}", the result is "{}""#,
        key, expected, actual,
    );
    // query hex2
    key = "hex2";
    expected = String::from("3735928559");
    actual = query_toml_value(toml_str, key).unwrap();
    assert_eq!(
        expected, actual,
        r#"The value of the key "{}" is wrong, it should be "{}", the result is "{}""#,
        key, expected, actual,
    );
    // query hex3
    key = "hex3";
    expected = String::from("3735928559");
    actual = query_toml_value(toml_str, key).unwrap();
    assert_eq!(
        expected, actual,
        r#"The value of the key "{}" is wrong, it should be "{}", the result is "{}""#,
        key, expected, actual,
    );
}

#[test]
/// 测试查询展示八进制整数
fn test_show_octal() {
    let toml_str = r##"
    # octal with prefix `0o`
    oct1 = 0o01234567
    oct2 = 0o755 # useful for Unix file permissions
    "##;
    let mut key: &str;
    let mut expected: String;
    let mut actual: String;
    println!("{}", toml_str);
    // query oct1
    key = "oct1";
    expected = String::from("342391");
    actual = query_toml_value(toml_str, key).unwrap();
    assert_eq!(
        expected, actual,
        r#"The value of the key "{}" is wrong, it should be "{}", the result is "{}""#,
        key, expected, actual,
    );
    // query oct2
    key = "oct2";
    expected = String::from("493");
    actual = query_toml_value(toml_str, key).unwrap();
    assert_eq!(
        expected, actual,
        r#"The value of the key "{}" is wrong, it should be "{}", the result is "{}""#,
        key, expected, actual,
    );
}

#[test]
/// 测试查询展示二进制整数
fn test_show_binary() {
    let toml_str = r##"
    # binary with prefix `0b`
    bin1 = 0b11010110
    "##;
    let key: &str;
    let expected: String;
    let actual: String;
    println!("{}", toml_str);
    // query bin1
    key = "bin1";
    expected = String::from("214");
    actual = query_toml_value(toml_str, key).unwrap();
    assert_eq!(
        expected, actual,
        r#"The value of the key "{}" is wrong, it should be "{}", the result is "{}""#,
        key, expected, actual,
    );
}
