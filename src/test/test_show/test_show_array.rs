use crate::query_toml_value;

#[test]
/// 测试查询展示数组值
fn test_show_array() {
    let toml_str = r##"
    integers = [ 1, 2, 3 ]
    colors = [ "red", "yellow", "green" ]
    nested_arrays_of_ints = [ [ 1, 2 ], [3, 4, 5] ]
    nested_mixed_array = [ [ 1, 2 ], ["a", "b", "c"] ]
    string_array = [ "all", 'strings', """are the same""", '''type''' ]
    "##;
    let mut key: &str;
    let mut expected: String;
    let mut actual: String;
    println!("{}", toml_str);
    // query integers
    key = "integers";
    expected = String::from("[1, 2, 3]");
    actual = query_toml_value(toml_str, key).unwrap();
    assert_eq!(
        expected, actual,
        r#"The value of the key "{}" is wrong, it should be "{}", the result is "{}""#,
        key, expected, actual,
    );
    // query colors
    key = "colors";
    expected = String::from(r#"["red", "yellow", "green"]"#);
    actual = query_toml_value(toml_str, key).unwrap();
    assert_eq!(
        expected, actual,
        r#"The value of the key "{}" is wrong, it should be "{}", the result is "{}""#,
        key, expected, actual,
    );
    // query nested_arrays_of_ints
    key = "nested_arrays_of_ints";
    expected = String::from(r#"[[1, 2], [3, 4, 5]]"#);
    actual = query_toml_value(toml_str, key).unwrap();
    assert_eq!(
        expected, actual,
        r#"The value of the key "{}" is wrong, it should be "{}", the result is "{}""#,
        key, expected, actual,
    );
    // query nested_mixed_array
    key = "nested_mixed_array";
    expected = String::from(r#"[[1, 2], ["a", "b", "c"]]"#);
    actual = query_toml_value(toml_str, key).unwrap();
    assert_eq!(
        expected, actual,
        r#"The value of the key "{}" is wrong, it should be "{}", the result is "{}""#,
        key, expected, actual,
    );
    // query string_array
    key = "string_array";
    expected = String::from(r#"["all", "strings", "are the same", "type"]"#);
    actual = query_toml_value(toml_str, key).unwrap();
    assert_eq!(
        expected, actual,
        r#"The value of the key "{}" is wrong, it should be "{}", the result is "{}""#,
        key, expected, actual,
    );
}

#[test]
/// 测试查询展示混合类型数组值
fn test_show_mixed_type_array() {
    let toml_str = r##"
    # Mixed-type arrays are allowed
    numbers = [ 0.1, 0.2, 0.5, 1, 2, 5 ]
    contributors = [
      "Foo Bar <foo@example.com>",
      { name = "Baz Qux", email = "bazqux@example.com", url = "https://example.com/bazqux" }
    ]
    "##;
    let mut key: &str;
    let mut expected: String;
    let mut actual: String;
    println!("{}", toml_str);
    // query numbers
    key = "numbers";
    expected = String::from(r#"[0.1, 0.2, 0.5, 1, 2, 5]"#);
    actual = query_toml_value(toml_str, key).unwrap();
    assert_eq!(
        expected, actual,
        r#"The value of the key "{}" is wrong, it should be "{}", the result is "{}""#,
        key, expected, actual,
    );
    // query contributors
    key = "contributors";
    expected = String::from(
        r#"["Foo Bar <foo@example.com>"
[[]]
email = "bazqux@example.com"
name = "Baz Qux"
url = "https://example.com/bazqux"
]"#,
    );
    actual = query_toml_value(toml_str, key).unwrap();
    assert_eq!(
        expected, actual,
        r#"The value of the key "{}" is wrong, it should be "{}", the result is "{}""#,
        key, expected, actual,
    );
}

#[test]
/// 测试查询展示表数组
fn test_show_array_of_table() {
    let toml_str = r##"
    [[products]]
    name = "Hammer"
    sku = 738594937

    [[products]]  # empty table within the array

    [[products]]
    name = "Nail"
    sku = 284758393

    color = "gray"


    [[fruits]]
    name = "apple"

    [fruits.physical]  # 子表
    color = "red"
    shape = "round"

    [[fruits.varieties]]  # 嵌套表数组
    name = "red delicious"

    [[fruits.varieties]]
    name = "granny smith"

    [[fruits]]
    name = "banana"

    [[fruits.varieties]]
    name = "plantain"
    "##;
    let mut key: &str;
    let mut expected: String;
    let mut actual: String;
    println!("{}", toml_str);
    // query products
    key = "products";
    expected = String::from(
        r#"[[]]
name = "Hammer"
sku = 738594937

[[]]

[[]]
color = "gray"
name = "Nail"
sku = 284758393
"#,
    );
    actual = query_toml_value(toml_str, key).unwrap();
    assert_eq!(
        expected, actual,
        r#"The value of the key "{}" is wrong, it should be "{}", the result is "{}""#,
        key, expected, actual,
    );
    // query fruits
    key = "fruits";
    expected = String::from(
        r#"[[]]
name = "apple"

[[varieties]]
name = "red delicious"

[[varieties]]
name = "granny smith"

[physical]
color = "red"
shape = "round"

[[]]
name = "banana"

[[varieties]]
name = "plantain"
"#,
    );
    actual = query_toml_value(toml_str, key).unwrap();
    assert_eq!(
        expected, actual,
        r#"The value of the key "{}" is wrong, it should be "{}", the result is "{}""#,
        key, expected, actual,
    );
}
