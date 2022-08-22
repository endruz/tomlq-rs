use crate::query_toml_value;

#[test]
/// 测试查询展示表
fn test_show_table() {
    let toml_str = r##"
    # Top-level table begins.
    name = "Fido"
    breed = "pug"
    # Top-level table ends.

    fruit.apple.color = "red"
    # Defines a table named fruit
    # Defines a table named fruit.apple

    fruit.apple.taste.sweet = true
    # Defines a table named fruit.apple.taste
    # fruit and fruit.apple were already created

    [owner]
    name = "Regina Dogman"
    member_since = 1999-08-04

    [dog."tater.man"]
    type.name = "pug"
    "##;
    let mut key: &str;
    let mut expected: String;
    let mut actual: String;
    println!("{}", toml_str);
    // query fruit
    key = "fruit";
    expected = String::from(
        r#"[apple]
color = "red"

[apple.taste]
sweet = true
"#,
    );
    actual = query_toml_value(toml_str, key).unwrap();
    assert_eq!(
        expected, actual,
        r#"The value of the key "{}" is wrong, it should be "{}", the result is "{}""#,
        key, expected, actual,
    );
    // query owner
    key = "owner";
    expected = String::from(
        r#"member_since = 1999-08-04
name = "Regina Dogman"
"#,
    );
    actual = query_toml_value(toml_str, key).unwrap();
    assert_eq!(
        expected, actual,
        r#"The value of the key "{}" is wrong, it should be "{}", the result is "{}""#,
        key, expected, actual,
    );
    // query dog
    key = "dog";
    expected = String::from(
        r#"["tater.man".type]
name = "pug"
"#,
    );
    actual = query_toml_value(toml_str, key).unwrap();
    assert_eq!(
        expected, actual,
        r#"The value of the key "{}" is wrong, it should be "{}", the result is "{}""#,
        key, expected, actual,
    );
}

#[test]
/// 测试查询展示内联表
fn test_show_inline_table() {
    let toml_str = r##"
    name = { first = "Tom", last = "Preston-Werner" }
    point = { x = 1, y = 2 }
    animal = { type.name = "pug" }

    [product]
    type = { name = "Nail" }
    "##;
    let mut key: &str;
    let mut expected: String;
    let mut actual: String;
    println!("{}", toml_str);
    // query name
    key = "name";
    expected = String::from(
        r#"first = "Tom"
last = "Preston-Werner"
"#,
    );
    actual = query_toml_value(toml_str, key).unwrap();
    assert_eq!(
        expected, actual,
        r#"The value of the key "{}" is wrong, it should be "{}", the result is "{}""#,
        key, expected, actual,
    );
    // query point
    key = "point";
    expected = String::from(
        r#"x = 1
y = 2
"#,
    );
    actual = query_toml_value(toml_str, key).unwrap();
    assert_eq!(
        expected, actual,
        r#"The value of the key "{}" is wrong, it should be "{}", the result is "{}""#,
        key, expected, actual,
    );
    // query animal
    key = "animal";
    expected = String::from(
        r#"[type]
name = "pug"
"#,
    );
    actual = query_toml_value(toml_str, key).unwrap();
    assert_eq!(
        expected, actual,
        r#"The value of the key "{}" is wrong, it should be "{}", the result is "{}""#,
        key, expected, actual,
    );
    // query product
    key = "product";
    expected = String::from(
        r#"[type]
name = "Nail"
"#,
    );
    actual = query_toml_value(toml_str, key).unwrap();
    assert_eq!(
        expected, actual,
        r#"The value of the key "{}" is wrong, it should be "{}", the result is "{}""#,
        key, expected, actual,
    );
}
