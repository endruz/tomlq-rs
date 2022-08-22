use crate::query_toml_value;

#[test]
/// 测试解析无效 TOML 文件：有键无值
fn test_parse_invalid_toml_1() -> Result<(), String> {
    let toml_str = "key = # INVALID";
    let key = "key";
    println!("{}", toml_str);
    match query_toml_value(toml_str, key) {
        Ok(_) => Err(String::from("Invalid TOML parsing succeeded!")),
        Err(msg) => match msg.as_str() {
            "Parsing failed!" => Ok(()),
            _ => Err(String::from("Parse invalid TOML error exception!")),
        },
    }
}

#[test]
/// 测试解析无效 TOML 文件：键值对不换行
fn test_parse_invalid_toml_2() -> Result<(), String> {
    let toml_str = r#"first = "Tom" last = "Preston-Werner" # INVALID"#;
    let key = "first";
    println!("{}", toml_str);
    match query_toml_value(toml_str, key) {
        Ok(_) => Err(String::from("Invalid TOML parsing succeeded!")),
        Err(msg) => match msg.as_str() {
            "Parsing failed!" => Ok(()),
            _ => Err(String::from("Parse invalid TOML error exception!")),
        },
    }
}

#[test]
/// 测试解析无效 TOML 文件：裸键不为空
fn test_parse_invalid_toml_3() -> Result<(), String> {
    let toml_str = r#"= "no key name"  # INVALID"#;
    let key = "key";
    println!("{}", toml_str);
    match query_toml_value(toml_str, key) {
        Ok(_) => Err(String::from("Invalid TOML parsing succeeded!")),
        Err(msg) => match msg.as_str() {
            "Parsing failed!" => Ok(()),
            _ => Err(String::from("Parse invalid TOML error exception!")),
        },
    }
}

#[test]
/// 测试解析无效 TOML 文件：多次定义同一个键 1
fn test_parse_invalid_toml_4() -> Result<(), String> {
    let toml_str = r#"name = "Tom"
    name = "Pradyun"#;
    let key = "name";
    println!("{}", toml_str);
    match query_toml_value(toml_str, key) {
        Ok(_) => Err(String::from("Invalid TOML parsing succeeded!")),
        Err(msg) => match msg.as_str() {
            "Parsing failed!" => Ok(()),
            _ => Err(String::from("Parse invalid TOML error exception!")),
        },
    }
}

#[test]
/// 测试解析无效 TOML 文件：多次定义同一个键 2
fn test_parse_invalid_toml_5() -> Result<(), String> {
    let toml_str = r#"spelling = "favorite"
    "spelling" = "favourite""#;
    let key = "spelling";
    println!("{}", toml_str);
    match query_toml_value(toml_str, key) {
        Ok(_) => Err(String::from("Invalid TOML parsing succeeded!")),
        Err(msg) => match msg.as_str() {
            "Parsing failed!" => Ok(()),
            _ => Err(String::from("Parse invalid TOML error exception!")),
        },
    }
}

#[test]
/// 测试解析无效 TOML 文件：将整数键设置为表
fn test_parse_invalid_toml_6() -> Result<(), String> {
    let toml_str = r#"# THE FOLLOWING IS INVALID
    # This defines the value of fruit.apple to be an integer.
    fruit.apple = 1
    # But then this treats fruit.apple like it's a table.
    # You can't turn an integer into a table.
    fruit.apple.smooth = true"#;
    let key = "fruit.apple.smooth";
    println!("{}", toml_str);
    match query_toml_value(toml_str, key) {
        Ok(_) => Err(String::from("Invalid TOML parsing succeeded!")),
        Err(msg) => match msg.as_str() {
            "Parsing failed!" => Ok(()),
            _ => Err(String::from("Parse invalid TOML error exception!")),
        },
    }
}

#[test]
/// 测试解析无效 TOML 文件：多行字面量字符串中写三个以上的单引号序列
fn test_parse_invalid_toml_7() -> Result<(), String> {
    let toml_str = r#"apos15 = '''Here are fifteen apostrophes: ''''''''''''''''''"#;
    let key = "apos15";
    println!("{}", toml_str);
    match query_toml_value(toml_str, key) {
        Ok(_) => Err(String::from("Invalid TOML parsing succeeded!")),
        Err(msg) => match msg.as_str() {
            "Parsing failed!" => Ok(()),
            _ => Err(String::from("Parse invalid TOML error exception!")),
        },
    }
}

#[test]
/// 测试解析无效 TOML 文件：非法的浮点数
fn test_parse_invalid_toml_8() -> Result<(), String> {
    let toml_str = r#"# INVALID FLOATS
    invalid_float_1 = .7
    invalid_float_2 = 7.
    invalid_float_3 = 3.e+20"#;
    let key = "invalid_float_1";
    println!("{}", toml_str);
    match query_toml_value(toml_str, key) {
        Ok(_) => Err(String::from("Invalid TOML parsing succeeded!")),
        Err(msg) => match msg.as_str() {
            "Parsing failed!" => Ok(()),
            _ => Err(String::from("Parse invalid TOML error exception!")),
        },
    }
}

#[test]
/// 测试解析无效 TOML 文件：重复定义一个表
fn test_parse_invalid_toml_9() -> Result<(), String> {
    let toml_str = r#"[fruit]
    apple = "red"

    [fruit]
    orange = "orange""#;
    let key = "fruit.apple";
    println!("{}", toml_str);
    match query_toml_value(toml_str, key) {
        Ok(_) => Err(String::from("Invalid TOML parsing succeeded!")),
        Err(msg) => match msg.as_str() {
            "Parsing failed!" => Ok(()),
            _ => Err(String::from("Parse invalid TOML error exception!")),
        },
    }
}

#[test]
/// 测试解析无效 TOML 文件：内联表添加键与子表
fn test_parse_invalid_toml_10() -> Result<(), String> {
    let toml_str = r#"[product]
    type = { name = "Nail" }
    type.edible = false  # INVALID"#;
    let key = "product.type";
    println!("{}", toml_str);
    match query_toml_value(toml_str, key) {
        Ok(_) => Err(String::from("Invalid TOML parsing succeeded!")),
        Err(msg) => match msg.as_str() {
            "Parsing failed!" => Ok(()),
            _ => Err(String::from("Parse invalid TOML error exception!")),
        },
    }
}

#[test]
/// 测试解析无效 TOML 文件：内联表用于向一个已定义的表添加键或子表
fn test_parse_invalid_toml_11() -> Result<(), String> {
    let toml_str = r#"[product]
    type.name = "Nail"
    type = { edible = false }  # INVALID"#;
    let key = "product.type";
    println!("{}", toml_str);
    match query_toml_value(toml_str, key) {
        Ok(_) => Err(String::from("Invalid TOML parsing succeeded!")),
        Err(msg) => match msg.as_str() {
            "Parsing failed!" => Ok(()),
            _ => Err(String::from("Parse invalid TOML error exception!")),
        },
    }
}

#[test]
/// 测试解析无效 TOML 文件：先定义表或表数组，在定义其父级为一个数组元素
fn test_parse_invalid_toml_12() -> Result<(), String> {
    let toml_str = r#"# INVALID TOML DOC
    [fruit.physical]  # subtable, but to which parent element should it belong?
    color = "red"
    shape = "round"

    [[fruit]]  # parser must throw an error upon discovering that "fruit" is
               # an array rather than a table
    name = "apple""#;
    let key = "fruit.physical.color";
    println!("{}", toml_str);
    match query_toml_value(toml_str, key) {
        Ok(_) => Err(String::from("Invalid TOML parsing succeeded!")),
        Err(msg) => match msg.as_str() {
            "Parsing failed!" => Ok(()),
            _ => Err(String::from("Parse invalid TOML error exception!")),
        },
    }
}

#[test]
/// 测试解析无效 TOML 文件：向一个静态定义的数组追加内容
fn test_parse_invalid_toml_13() -> Result<(), String> {
    let toml_str = r#"# INVALID TOML DOC
    fruits = []

    [[fruits]] # Not allowed"#;
    let key = "fruit";
    println!("{}", toml_str);
    match query_toml_value(toml_str, key) {
        Ok(_) => Err(String::from("Invalid TOML parsing succeeded!")),
        Err(msg) => match msg.as_str() {
            "Parsing failed!" => Ok(()),
            _ => Err(String::from("Parse invalid TOML error exception!")),
        },
    }
}

#[test]
/// 测试解析无效 TOML 文件：用已经确定为数组的名称定义表
fn test_parse_invalid_toml_14() -> Result<(), String> {
    let toml_str = r#"[[fruits]]
    name = "apple"

    [[fruits.varieties]]
    name = "red delicious"

    # INVALID: This table conflicts with the previous array of tables
    [fruits.varieties]
    name = "granny smith""#;
    let key = "fruit.varieties.name";
    println!("{}", toml_str);
    match query_toml_value(toml_str, key) {
        Ok(_) => Err(String::from("Invalid TOML parsing succeeded!")),
        Err(msg) => match msg.as_str() {
            "Parsing failed!" => Ok(()),
            _ => Err(String::from("Parse invalid TOML error exception!")),
        },
    }
}

#[test]
/// 测试解析无效 TOML 文件：表数组与之前的表冲突
fn test_parse_invalid_toml_15() -> Result<(), String> {
    let toml_str = r#"[fruits.physical]
    color = "red"
    shape = "round"

    # INVALID: This array of tables conflicts with the previous table
    [[fruits.physical]]
    color = "green""#;
    let key = "fruit.physical.color";
    println!("{}", toml_str);
    match query_toml_value(toml_str, key) {
        Ok(_) => Err(String::from("Invalid TOML parsing succeeded!")),
        Err(msg) => match msg.as_str() {
            "Parsing failed!" => Ok(()),
            _ => Err(String::from("Parse invalid TOML error exception!")),
        },
    }
}
