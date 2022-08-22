use crate::query_toml_value;

#[test]
/// 测试查询展示布尔值
fn test_show_bool() {
    let toml_str = r##"
    bool1 = true
    bool2 = false
    "##;
    let mut key: &str;
    let mut expected: String;
    let mut actual: String;
    println!("{}", toml_str);
    // query bool1
    key = "bool1";
    expected = String::from("true");
    actual = query_toml_value(toml_str, key).unwrap();
    assert_eq!(
        expected, actual,
        r#"The value of the key "{}" is wrong, it should be "{}", the result is "{}""#,
        key, expected, actual,
    );
    // query bool2
    key = "bool2";
    expected = String::from("false");
    actual = query_toml_value(toml_str, key).unwrap();
    assert_eq!(
        expected, actual,
        r#"The value of the key "{}" is wrong, it should be "{}", the result is "{}""#,
        key, expected, actual,
    );
}
