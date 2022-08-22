use crate::query_toml_value;

#[test]
/// 测试查询展示坐标日期时刻
fn test_show_offset_date_time() {
    let toml_str = r##"
    odt1 = 1979-05-27T07:32:00Z
    odt2 = 1979-05-27T00:32:00-07:00
    odt3 = 1979-05-27T00:32:00.999999-07:00
    odt4 = 1979-05-27 07:32:00Z
    "##;
    let mut key: &str;
    let mut expected: String;
    let mut actual: String;
    println!("{}", toml_str);
    // query odt1
    key = "odt1";
    expected = String::from("1979-05-27T07:32:00Z");
    actual = query_toml_value(toml_str, key).unwrap();
    assert_eq!(
        expected, actual,
        r#"The value of the key "{}" is wrong, it should be "{}", the result is "{}""#,
        key, expected, actual,
    );
    // query odt2
    key = "odt2";
    expected = String::from("1979-05-27T00:32:00-07:00");
    actual = query_toml_value(toml_str, key).unwrap();
    assert_eq!(
        expected, actual,
        r#"The value of the key "{}" is wrong, it should be "{}", the result is "{}""#,
        key, expected, actual,
    );
    // query odt3
    key = "odt3";
    expected = String::from("1979-05-27T00:32:00.999999-07:00");
    actual = query_toml_value(toml_str, key).unwrap();
    assert_eq!(
        expected, actual,
        r#"The value of the key "{}" is wrong, it should be "{}", the result is "{}""#,
        key, expected, actual,
    );
    // query odt4
    key = "odt4";
    expected = String::from("1979-05-27T07:32:00Z");
    actual = query_toml_value(toml_str, key).unwrap();
    assert_eq!(
        expected, actual,
        r#"The value of the key "{}" is wrong, it should be "{}", the result is "{}""#,
        key, expected, actual,
    );
}

#[test]
/// 测试查询展示各地日期时刻
fn test_show_local_date_time() {
    let toml_str = r##"
    ldt1 = 1979-05-27T07:32:00
    ldt2 = 1979-05-27T00:32:00.999999
    "##;
    let mut key: &str;
    let mut expected: String;
    let mut actual: String;
    println!("{}", toml_str);
    // query ldt1
    key = "ldt1";
    expected = String::from("1979-05-27T07:32:00");
    actual = query_toml_value(toml_str, key).unwrap();
    assert_eq!(
        expected, actual,
        r#"The value of the key "{}" is wrong, it should be "{}", the result is "{}""#,
        key, expected, actual,
    );
    // query ldt2
    key = "ldt2";
    expected = String::from("1979-05-27T00:32:00.999999");
    actual = query_toml_value(toml_str, key).unwrap();
    assert_eq!(
        expected, actual,
        r#"The value of the key "{}" is wrong, it should be "{}", the result is "{}""#,
        key, expected, actual,
    );
}

#[test]
/// 测试查询展示各地日期
fn test_show_local_date() {
    let toml_str = r##"
    ld1 = 1979-05-27
    "##;
    let key: &str;
    let expected: String;
    let actual: String;
    println!("{}", toml_str);
    // query ld1
    key = "ld1";
    expected = String::from("1979-05-27");
    actual = query_toml_value(toml_str, key).unwrap();
    assert_eq!(
        expected, actual,
        r#"The value of the key "{}" is wrong, it should be "{}", the result is "{}""#,
        key, expected, actual,
    );
}

#[test]
/// 测试查询展示各标时刻
fn test_show_local_time() {
    let toml_str = r##"
    lt1 = 07:32:00
    lt2 = 00:32:00.999999
    "##;
    let mut key: &str;
    let mut expected: String;
    let mut actual: String;
    println!("{}", toml_str);
    // query lt1
    key = "lt1";
    expected = String::from("07:32:00");
    actual = query_toml_value(toml_str, key).unwrap();
    assert_eq!(
        expected, actual,
        r#"The value of the key "{}" is wrong, it should be "{}", the result is "{}""#,
        key, expected, actual,
    );
    // query lt2
    key = "lt2";
    expected = String::from("00:32:00.999999");
    actual = query_toml_value(toml_str, key).unwrap();
    assert_eq!(
        expected, actual,
        r#"The value of the key "{}" is wrong, it should be "{}", the result is "{}""#,
        key, expected, actual,
    );
}
