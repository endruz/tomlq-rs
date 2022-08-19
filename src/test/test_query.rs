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

// #[test]
/// TODO
/// 测试查询字符串值
// fn test_query_str_value() {

// }

#[test]
/// 测试查询整数
fn test_query_int_value() {
    let toml_str = r##"int1 = +99
    int2 = 42
    int3 = 0
    int4 = -17
    int5 = 1_000
    int6 = 5_349_221
    int7 = 53_49_221  # Indian number system grouping
    int8 = 1_2_3_4_5  # VALID but discouraged
    # hexadecimal with prefix `0x`
    hex1 = 0xDEADBEEF
    hex2 = 0xdeadbeef
    hex3 = 0xdead_beef

    # octal with prefix `0o`
    oct1 = 0o01234567
    oct2 = 0o755 # useful for Unix file permissions

    # binary with prefix `0b`
    bin1 = 0b11010110
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

#[test]
/// 测试查询浮点数
fn test_query_float_value() {
    let toml_str = r##"# fractional
    flt1 = +1.0
    flt2 = 3.1415
    flt3 = -0.01

    # exponent
    flt4 = 5e+22
    flt5 = 1e06
    flt6 = -2E-2

    # both
    flt7 = 6.626e-34

    flt8 = 224_617.445_991_228

    # infinity
    sf1 = inf  # positive infinity
    sf2 = +inf # positive infinity
    sf3 = -inf # negative infinity

    # not a number
    sf4 = nan  # actual sNaN/qNaN encoding is implementation-specific
    sf5 = +nan # same as `nan`
    sf6 = -nan # valid, actual encoding is implementation-specific
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
    // query flt7
    key = "flt7";
    expected = String::from("0.0000000000000000000000000000000006626");
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

#[test]
/// 测试查询布尔值
fn test_query_bool_value() {
    let toml_str = r##"bool1 = true
    bool2 = false"##;
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

#[test]
/// 测试查询日期
fn test_query_date_value() {
    let toml_str = r##"odt1 = 1979-05-27T07:32:00Z
    odt2 = 1979-05-27T00:32:00-07:00
    odt3 = 1979-05-27T00:32:00.999999-07:00
    odt4 = 1979-05-27 07:32:00Z
    ldt1 = 1979-05-27T07:32:00
    ldt2 = 1979-05-27T00:32:00.999999
    ld1 = 1979-05-27
    lt1 = 07:32:00
    lt2 = 00:32:00.999999
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
    // query ld1
    key = "ld1";
    expected = String::from("1979-05-27");
    actual = query_toml_value(toml_str, key).unwrap();
    assert_eq!(
        expected, actual,
        r#"The value of the key "{}" is wrong, it should be "{}", the result is "{}""#,
        key, expected, actual,
    );
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

#[test]
/// 测试查询数组值
fn test_query_array_value() {
    let toml_str = r##"integers = [ 1, 2, 3 ]
    colors = [ "red", "yellow", "green" ]
    nested_arrays_of_ints = [ [ 1, 2 ], [3, 4, 5] ]
    nested_mixed_array = [ [ 1, 2 ], ["a", "b", "c"] ]
    string_array = [ "all", 'strings', """are the same""", '''type''' ]

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
    // query numbers
    key = "numbers";
    expected = String::from(r#"[0.1, 0.2, 0.5, 1, 2, 5]"#);
    actual = query_toml_value(toml_str, key).unwrap();
    assert_eq!(
        expected, actual,
        r#"The value of the key "{}" is wrong, it should be "{}", the result is "{}""#,
        key, expected, actual,
    );
    // // query contributors
    // key = "contributors";
    // expected = String::from(r#"["Foo Bar <foo@example.com>",
    //     { name = "Baz Qux", email = "bazqux@example.com", url = "https://example.com/bazqux" }
    //   ]"#);
    // actual = query_toml_value(toml_str, key).unwrap();
    // assert_eq!(
    //     expected, actual,
    //     r#"The value of the key "{}" is wrong, it should be "{}", the result is "{}""#,
    //     key, expected, actual,
    // );
}
