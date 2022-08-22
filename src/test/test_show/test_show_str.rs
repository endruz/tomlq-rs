use crate::query_toml_value;

#[test]
/// 测试查询展示基本字符串
fn test_show_basic_strings() {
    let toml_str = r##"
    str = "I'm a string. \"You can quote me\". Name\tJos\u00E9\nLocation\tSF."
    "##;
    let key: &str;
    let expected: String;
    let actual: String;
    println!("{}", toml_str);
    // query str
    key = "str";
    expected = String::from(r#"I'm a string. \"You can quote me\". Name\tJosé\nLocation\tSF."#);
    actual = query_toml_value(toml_str, key).unwrap();
    assert_eq!(
        expected, actual,
        r#"The value of the key "{}" is wrong, it should be "{}", the result is "{}""#,
        key, expected, actual,
    );
}

#[test]
/// 测试查询展示多行基本字符串
fn test_show_multi_line_basic_strings() {
    let toml_str = r##"
    str1 = """
Roses are red
Violets are blue"""
    str2 = """
The quick brown \


    fox jumps over \
    the lazy dog."""
    str3 = """\
           The quick brown \
           fox jumps over \
           the lazy dog.\
           """
    str4 = """Here are two quotation marks: "". Simple enough."""
    # str5 = """Here are three quotation marks: """."""  # INVALID
    str5 = """Here are three quotation marks: ""\"."""
    str6 = """Here are fifteen quotation marks: ""\"""\"""\"""\"""\"."""

    # "This," she said, "is just a pointless statement."
    str7 = """"This," she said, "is just a pointless statement.""""
    "##;
    let mut key: &str;
    let mut expected: String;
    let mut actual: String;
    println!("{}", toml_str);
    // query str1
    key = "str1";
    if cfg!(target_os = "windows") {
        expected = String::from(r#"Roses are red\r\nViolets are blue"#);
    } else if cfg!(target_os = "linux") {
        expected = String::from(r#"Roses are red\nViolets are blue"#);
    } else {
        expected = String::from(r#"Roses are red\rViolets are blue"#);
    }
    actual = query_toml_value(toml_str, key).unwrap();
    assert_eq!(
        expected, actual,
        r#"The value of the key "{}" is wrong, it should be "{}", the result is "{}""#,
        key, expected, actual,
    );
    // query str2
    key = "str2";
    expected = String::from(r#"The quick brown fox jumps over the lazy dog."#);
    actual = query_toml_value(toml_str, key).unwrap();
    assert_eq!(
        expected, actual,
        r#"The value of the key "{}" is wrong, it should be "{}", the result is "{}""#,
        key, expected, actual,
    );
    // query str3
    key = "str3";
    expected = String::from(r#"The quick brown fox jumps over the lazy dog."#);
    actual = query_toml_value(toml_str, key).unwrap();
    assert_eq!(
        expected, actual,
        r#"The value of the key "{}" is wrong, it should be "{}", the result is "{}""#,
        key, expected, actual,
    );
    // query str4
    key = "str4";
    expected = String::from(r#"Here are two quotation marks: \"\". Simple enough."#);
    actual = query_toml_value(toml_str, key).unwrap();
    assert_eq!(
        expected, actual,
        r#"The value of the key "{}" is wrong, it should be "{}", the result is "{}""#,
        key, expected, actual,
    );
    // query str5
    key = "str5";
    expected = String::from(r#"Here are three quotation marks: \"\"\"."#);
    actual = query_toml_value(toml_str, key).unwrap();
    assert_eq!(
        expected, actual,
        r#"The value of the key "{}" is wrong, it should be "{}", the result is "{}""#,
        key, expected, actual,
    );
    // query str6
    key = "str6";
    expected = String::from(r#"Here are fifteen quotation marks: \"\"\"\"\"\"\"\"\"\"\"\"\"\"\"."#);
    actual = query_toml_value(toml_str, key).unwrap();
    assert_eq!(
        expected, actual,
        r#"The value of the key "{}" is wrong, it should be "{}", the result is "{}""#,
        key, expected, actual,
    );
    // query str7
    key = "str7";
    expected = String::from(r#"\"This,\" she said, \"is just a pointless statement.\""#);
    actual = query_toml_value(toml_str, key).unwrap();
    assert_eq!(
        expected, actual,
        r#"The value of the key "{}" is wrong, it should be "{}", the result is "{}""#,
        key, expected, actual,
    );
}

#[test]
/// 测试查询展示字面量字符串
fn test_show_literal_strings() {
    let toml_str = r##"
    # What you see is what you get.
    winpath  = 'C:\Users\nodejs\templates'
    winpath2 = '\\ServerX\admin$\system32\'
    quoted   = 'Tom "Dubs" Preston-Werner'
    regex    = '<\i\c*\s*>'
    "##;
    let mut key: &str;
    let mut expected: String;
    let mut actual: String;
    println!("{}", toml_str);
    // query winpath
    key = "winpath";
    expected = String::from(r#"C:\\Users\\nodejs\\templates"#);
    actual = query_toml_value(toml_str, key).unwrap();
    assert_eq!(
        expected, actual,
        r#"The value of the key "{}" is wrong, it should be "{}", the result is "{}""#,
        key, expected, actual,
    );
    // query winpath2
    key = "winpath2";
    expected = String::from(r#"\\\\ServerX\\admin$\\system32\\"#);
    actual = query_toml_value(toml_str, key).unwrap();
    assert_eq!(
        expected, actual,
        r#"The value of the key "{}" is wrong, it should be "{}", the result is "{}""#,
        key, expected, actual,
    );
    // query quoted
    key = "quoted";
    expected = String::from(r#"Tom \"Dubs\" Preston-Werner"#);
    actual = query_toml_value(toml_str, key).unwrap();
    assert_eq!(
        expected, actual,
        r#"The value of the key "{}" is wrong, it should be "{}", the result is "{}""#,
        key, expected, actual,
    );
    // query regex
    key = "regex";
    expected = String::from(r#"<\\i\\c*\\s*>"#);
    actual = query_toml_value(toml_str, key).unwrap();
    assert_eq!(
        expected, actual,
        r#"The value of the key "{}" is wrong, it should be "{}", the result is "{}""#,
        key, expected, actual,
    );
}

#[test]
/// 测试查询展示多行字面量字符串
fn test_show_multi_line_literal_strings() {
    let toml_str = r##"
    regex2 = '''I [dw]on't need \d{2} apples'''
    lines  = '''
The first newline is
trimmed in raw strings.
    All other whitespace
    is preserved.
'''
    quot15 = '''Here are fifteen quotation marks: """""""""""""""'''

    # apos15 = '''Here are fifteen apostrophes: ''''''''''''''''''  # INVALID
    apos15 = "Here are fifteen apostrophes: '''''''''''''''"

    # 'That,' she said, 'is still pointless.'
    str8 = ''''That,' she said, 'is still pointless.''''
    "##;
    let mut key: &str;
    let mut expected: String;
    let mut actual: String;
    println!("{}", toml_str);
    // query regex2
    key = "regex2";
    expected = String::from(r#"I [dw]on't need \\d{2} apples"#);
    actual = query_toml_value(toml_str, key).unwrap();
    assert_eq!(
        expected, actual,
        r#"The value of the key "{}" is wrong, it should be "{}", the result is "{}""#,
        key, expected, actual,
    );
    // query lines
    key = "lines";
    expected = String::from(
        r#"The first newline is\ntrimmed in raw strings.\n    All other whitespace\n    is preserved.\n"#,
    );
    actual = query_toml_value(toml_str, key).unwrap();
    assert_eq!(
        expected, actual,
        r#"The value of the key "{}" is wrong, it should be "{}", the result is "{}""#,
        key, expected, actual,
    );
    // query quot15
    key = "quot15";
    expected = String::from(r#"Here are fifteen quotation marks: \"\"\"\"\"\"\"\"\"\"\"\"\"\"\""#);
    actual = query_toml_value(toml_str, key).unwrap();
    assert_eq!(
        expected, actual,
        r#"The value of the key "{}" is wrong, it should be "{}", the result is "{}""#,
        key, expected, actual,
    );
    // query apos15
    key = "apos15";
    expected = String::from(r#"Here are fifteen apostrophes: '''''''''''''''"#);
    actual = query_toml_value(toml_str, key).unwrap();
    assert_eq!(
        expected, actual,
        r#"The value of the key "{}" is wrong, it should be "{}", the result is "{}""#,
        key, expected, actual,
    );
    // query str8
    key = "str8";
    expected = String::from(r#"'That,' she said, 'is still pointless.'"#);
    actual = query_toml_value(toml_str, key).unwrap();
    assert_eq!(
        expected, actual,
        r#"The value of the key "{}" is wrong, it should be "{}", the result is "{}""#,
        key, expected, actual,
    );
}
