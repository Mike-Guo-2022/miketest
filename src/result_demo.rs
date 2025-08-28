use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{self, Read};
use std::num::ParseIntError;

// A small custom error to show how to define and use your own error types
#[derive(Debug)]
enum DemoError {
    Io(io::Error),
    Parse(ParseIntError),
    BusinessRule(&'static str),
}

impl Display for DemoError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DemoError::Io(e) => write!(f, "IO error: {}", e),
            DemoError::Parse(e) => write!(f, "Parse error: {}", e),
            DemoError::BusinessRule(msg) => write!(f, "Business error: {}", msg),
        }
    }
}

impl Error for DemoError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            DemoError::Io(e) => Some(e),
            DemoError::Parse(e) => Some(e),
            DemoError::BusinessRule(_) => None,
        }
    }
}

impl From<io::Error> for DemoError {
    fn from(err: io::Error) -> Self { DemoError::Io(err) }
}

impl From<ParseIntError> for DemoError {
    fn from(err: ParseIntError) -> Self { DemoError::Parse(err) }
}

// Public entry to run all demos
pub fn run() -> Result<(), Box<dyn Error>> {
    println!("=== Result 基本用法 ===");
    basics()?;

    println!("\n=== ? 运算符传播错误 ===");
    demonstrate_question_mark_operator()?;

    println!("\n=== map / map_err / and_then 组合器 ===");
    demonstrate_combinators();

    println!("\n=== 自定义错误类型与 From 转换 ===");
    match read_number_from_file("numbers.txt") {
        Ok(n) => println!("读取成功: {}", n),
        Err(e) => eprintln!("读取失败: {}", e),
    }

    println!("\n=== 将具体错误抹平为 Box<dyn Error> ===");
    let boxed: Result<u32, Box<dyn Error>> = read_number_generic("numbers.txt");
    match boxed {
        Ok(n) => println!("boxed 读取成功: {}", n),
        Err(e) => eprintln!("boxed 读取失败: {}", e),
    }

    Ok(())
}

fn basics() -> Result<(), Box<dyn Error>> {
    let ok_value: Result<i32, &str> = Ok(42);
    let err_value: Result<i32, &str> = Err("boom");

    // Inspect results
    println!("ok_value.is_ok() = {}", ok_value.is_ok());
    println!("err_value.is_err() = {}", err_value.is_err());

    // Unwrap with default
    println!("unwrap_or: {}", err_value.unwrap_or(-1));

    // Match
    match ok_value {
        Ok(v) => println!("match Ok: {}", v),
        Err(e) => println!("match Err: {}", e),
    }

    Ok(())
}

// Use ? to propagate errors upward as DemoError
fn demonstrate_question_mark_operator() -> Result<(), DemoError> {
    // Simulate parsing from string
    let s = "123";
    let n: i32 = s.parse()?; // ParseIntError -> DemoError via From
    println!("parsed = {}", n);

    // Simulate IO: read current source file just to demo
    let mut file = File::open("src/main.rs")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    println!("main.rs length = {}", content.len());

    // Trigger a business rule error path
    if n % 2 == 0 {
        return Err(DemoError::BusinessRule("数字不能是偶数"));
    }

    Ok(())
}

// Combinators showcase
fn demonstrate_combinators() {
    let input = "10";

    // map: transform Ok value
    let doubled = input.parse::<i32>().map(|v| v * 2);
    println!("map doubled = {:?}", doubled);

    // map_err: transform Err value
    let bad_input = "abc";
    let mapped_err = bad_input.parse::<i32>().map_err(|e| format!("解析失败: {}", e));
    println!("map_err => {:?}", mapped_err);

    // and_then: chain computations that also return Result
    fn reciprocal(x: f64) -> Result<f64, &'static str> {
        if x == 0.0 { Err("除数不能为 0") } else { Ok(1.0 / x) }
    }
    let chained = "5".parse::<f64>().map_err(|_| "不是数字").and_then(reciprocal);
    println!("and_then chained = {:?}", chained);
}

// Read a number from a file, demonstrating custom error usage
fn read_number_from_file(path: &str) -> Result<u32, DemoError> {
    let mut buf = String::new();
    File::open(path)?.read_to_string(&mut buf)?;
    let trimmed = buf.trim();
    let n: u32 = trimmed.parse()?;
    Ok(n)
}

// Erase specific errors into Box<dyn Error>
fn read_number_generic(path: &str) -> Result<u32, Box<dyn Error>> {
    let mut buf = String::new();
    File::open(path)?.read_to_string(&mut buf)?;
    Ok(buf.trim().parse()?)
}


