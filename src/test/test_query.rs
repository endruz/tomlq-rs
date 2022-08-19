use super::super::query_toml_value;

#[test]
/// 测试查询有效键
fn test_query_valid_key() {
    let toml_str = r##"# This is a full-line comment
    key = "value"  # This is a comment at the end of a line
    another = "# This is not a comment"
    "" = "blank"
    site."google.com" = true
    "127.0.0.1" = "localhost"
    "##;
    let mut key: &str;
    let mut expected: String;
    let mut actual: String;
    println!("{}", toml_str);
    // query key
    key = "key";
    expected = String::from("value");
    actual = query_toml_value(toml_str, key).unwrap();
    assert_eq!(
        expected, actual,
        r#"The value of the key "{}" is wrong, it should be "{}", the result is "{}""#,
        key, expected, actual,
    );
    // query another
    key = "another";
    expected = String::from("# This is not a comment");
    actual = query_toml_value(toml_str, key).unwrap();
    assert_eq!(
        expected, actual,
        r#"The value of the key "{}" is wrong, it should be "{}", the result is "{}""#,
        key, expected, actual,
    );
    // query ""
    key = "";
    expected = String::from("blank");
    actual = query_toml_value(toml_str, key).unwrap();
    assert_eq!(
        expected, actual,
        r#"The value of the key "{}" is wrong, it should be "{}", the result is "{}""#,
        key, expected, actual,
    );
    // query site."google.com"
    key = r#"site."google.com""#;
    expected = String::from("true");
    actual = query_toml_value(toml_str, key).unwrap();
    assert_eq!(
        expected, actual,
        r#"The value of the key "{}" is wrong, it should be "{}", the result is "{}""#,
        key, expected, actual,
    );
    // query "127.0.0.1"
    key = r#""127.0.0.1""#;
    expected = String::from("localhost");
    actual = query_toml_value(toml_str, key).unwrap();
    assert_eq!(
        expected, actual,
        r#"The value of the key "{}" is wrong, it should be "{}", the result is "{}""#,
        key, expected, actual,
    );
}

#[test]
#[should_panic(expected = "Key key not found!")]
/// 测试查询不存在的键
fn test_query_nonexistent_key() {
    let toml_str = r#"name = "Orange"
    physical.color = "orange"
    physical.shape = "round"
    site."google.com" = true
    "#;
    let key = "key";
    println!("{}", toml_str);
    query_toml_value(toml_str, key).unwrap();
}
