use crate::query_toml_value;

#[test]
/// 测试查询展示小数
fn test_show_fractional() {
    let toml_str = r##"
    # fractional
    flt1 = +1.0
    flt2 = 3.1415
    flt3 = -0.01
    flt8 = 224_617.445_991_228
    "##;
    let mut key: &str;
    let mut expected: String;
    let mut actual: String;
    println!("{}", toml_str);
    // query flt1
    key = "flt1";
    expected = String::from("1.0");
    actual = query_toml_value(toml_str, key).unwrap();
    assert_eq!(
        expected, actual,
        r#"The value of the key "{}" is wrong, it should be "{}", the result is "{}""#,
        key, expected, actual,
    );
    // query flt2
    key = "flt2";
    expected = String::from("3.1415");
    actual = query_toml_value(toml_str, key).unwrap();
    assert_eq!(
        expected, actual,
        r#"The value of the key "{}" is wrong, it should be "{}", the result is "{}""#,
        key, expected, actual,
    );
    // query flt3
    key = "flt3";
    expected = String::from("-0.01");
    actual = query_toml_value(toml_str, key).unwrap();
    assert_eq!(
        expected, actual,
        r#"The value of the key "{}" is wrong, it should be "{}", the result is "{}""#,
        key, expected, actual,
    );

    // query flt8
    key = "flt8";
    expected = String::from("224617.445991228");
    actual = query_toml_value(toml_str, key).unwrap();
    assert_eq!(
        expected, actual,
        r#"The value of the key "{}" is wrong, it should be "{}", the result is "{}""#,
        key, expected, actual,
    );
}

#[test]
/// 测试查询展示指数
fn test_show_exponent() {
    let toml_str = r##"
    # exponent
    flt4 = 5e+22
    flt5 = 1e06
    flt6 = -2E-2
    "##;
    let mut key: &str;
    let mut expected: String;
    let mut actual: String;
    println!("{}", toml_str);
    // query flt4
    key = "flt4";
    expected = String::from("50000000000000000000000.0");
    actual = query_toml_value(toml_str, key).unwrap();
    assert_eq!(
        expected, actual,
        r#"The value of the key "{}" is wrong, it should be "{}", the result is "{}""#,
        key, expected, actual,
    );
    // query flt5
    key = "flt5";
    expected = String::from("1000000.0");
    actual = query_toml_value(toml_str, key).unwrap();
    assert_eq!(
        expected, actual,
        r#"The value of the key "{}" is wrong, it should be "{}", the result is "{}""#,
        key, expected, actual,
    );
    // query flt6
    key = "flt6";
    expected = String::from("-0.02");
    actual = query_toml_value(toml_str, key).unwrap();
    assert_eq!(
        expected, actual,
        r#"The value of the key "{}" is wrong, it should be "{}", the result is "{}""#,
        key, expected, actual,
    );
}

#[test]
/// 测试查询展示指数形式的小数
fn test_show_exponential_fraction() {
    let toml_str = r##"
    # both
    flt7 = 6.626e-34
    "##;
    let key: &str;
    let expected: String;
    let actual: String;
    println!("{}", toml_str);
    // query flt7
    key = "flt7";
    expected = String::from("0.0000000000000000000000000000000006626");
    actual = query_toml_value(toml_str, key).unwrap();
    assert_eq!(
        expected, actual,
        r#"The value of the key "{}" is wrong, it should be "{}", the result is "{}""#,
        key, expected, actual,
    );
}

#[test]
/// 测试查询展示无穷
fn test_show_infinity() {
    let toml_str = r##"
    # infinity
    sf1 = inf  # positive infinity
    sf2 = +inf # positive infinity
    sf3 = -inf # negative infinity
    "##;
    let mut key: &str;
    let mut expected: String;
    let mut actual: String;
    println!("{}", toml_str);
    // query sf1
    key = "sf1";
    expected = String::from("inf");
    actual = query_toml_value(toml_str, key).unwrap();
    assert_eq!(
        expected, actual,
        r#"The value of the key "{}" is wrong, it should be "{}", the result is "{}""#,
        key, expected, actual,
    );
    // query sf2
    key = "sf2";
    expected = String::from("inf");
    actual = query_toml_value(toml_str, key).unwrap();
    assert_eq!(
        expected, actual,
        r#"The value of the key "{}" is wrong, it should be "{}", the result is "{}""#,
        key, expected, actual,
    );
    // query sf3
    key = "sf3";
    expected = String::from("-inf");
    actual = query_toml_value(toml_str, key).unwrap();
    assert_eq!(
        expected, actual,
        r#"The value of the key "{}" is wrong, it should be "{}", the result is "{}""#,
        key, expected, actual,
    );
}

#[test]
/// 测试查询展示非数
fn test_show_nan() {
    let toml_str = r##"
    # not a number
    sf4 = nan  # actual sNaN/qNaN encoding is implementation-specific
    sf5 = +nan # same as `nan`
    sf6 = -nan # valid, actual encoding is implementation-specific
    "##;
    let mut key: &str;
    let mut expected: String;
    let mut actual: String;
    println!("{}", toml_str);
    // query sf4
    key = "sf4";
    expected = String::from("nan");
    actual = query_toml_value(toml_str, key).unwrap();
    assert_eq!(
        expected, actual,
        r#"The value of the key "{}" is wrong, it should be "{}", the result is "{}""#,
        key, expected, actual,
    );
    // query sf5
    key = "sf5";
    expected = String::from("nan");
    actual = query_toml_value(toml_str, key).unwrap();
    assert_eq!(
        expected, actual,
        r#"The value of the key "{}" is wrong, it should be "{}", the result is "{}""#,
        key, expected, actual,
    );
    // query sf6
    key = "sf6";
    expected = String::from("-nan");
    actual = query_toml_value(toml_str, key).unwrap();
    assert_eq!(
        expected, actual,
        r#"The value of the key "{}" is wrong, it should be "{}", the result is "{}""#,
        key, expected, actual,
    );
}
