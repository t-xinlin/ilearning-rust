// 有时候把所有不同的错误都视为一种错误类型会简化代码。我们将用一个自定义错误类型来 演示这一点。
//
// Rust 允许我们定义自己的错误类型。一般来说，一个 “好的” 错误类型应当：
//
// 用同一个类型代表了多种错误
// 向用户提供了清楚的错误信息
// 能够容易地与其他类型比较
// 好的例子：Err(EmptyVec)
// 坏的例子：Err("Please use a vector with at least one element".to_owned())
// 能够容纳错误的具体信息
// 好的例子：Err(BadChar(c, position))
// 坏的例子：Err("+ cannot be used here".to_owned())
// 能够与其他错误很好地整合

use std::error;
use std::fmt;
use std::num;

type Result<T> = std::result::Result<T, DoubleError>;

// 定义我们的错误类型，这种类型可以根据错误处理的实际情况定制。
// 我们可以完全自定义错误类型，也可以在类型中完全采用底层的错误实现，
// 也可以介于二者之间。
#[derive(Debug, Clone)]
struct DoubleError;

// 错误的生成与它如何显示是完全没关系的。没有必要担心复杂的逻辑会导致混乱的显示。
//
// 注意我们没有储存关于错误的任何额外信息，也就是说，如果不修改我们的错误类型定义的话，
// 就无法指明是哪个字符串解析失败了。
impl fmt::Display for DoubleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

// 为 `DoubleError` 实现 `Error` trait，这样其他错误可以包裹这个错误类型。
impl error::Error for DoubleError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // 泛型错误，没有记录其内部原因。
        None
    }
}

fn double_first(vec: Vec<&str>) -> Result<i32> {
    vec.first()
        // 把错误换成我们的新类型。
        .ok_or(DoubleError)
        .and_then(|s| {
            s.parse::<i32>()
                // 这里也换成新类型。
                .map_err(|_| DoubleError)
                .map(|i| 2 * i)
        })
}

fn print(result: Result<i32>) {
    match result {
        Ok(n)  => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

// 把错误 “装箱”
// 如果又想写简单的代码，又想保存原始错误信息，一个方法是把它们装箱（Box）。这 样做的坏处就是，被包装的错误类型只能在运行时了解，而不能被静态地 判别。
//
// 对任何实现了 Error trait 的类型，标准库的 Box 通过 From 为它们提供了 到 Box<Error> 的转换。
// 为 `Box<error::Error>` 取别名。
type ResultBox<T> = std::result::Result<T, Box<error::Error>>;

#[derive(Debug, Clone)]
struct EmptyVec;

impl fmt::Display for EmptyVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

impl error::Error for EmptyVec {
    fn description(&self) -> &str {
        "invalid first item to double"
    }

    fn cause(&self) -> Option<&error::Error> {
        // 泛型错误。没有记录其内部原因。
        None
    }
}

// impl From<DoubleError> for EmptyVec {
//     fn from(error: DoubleError) -> Self {
//         EmptyVec::source(error)
//     }
// }

impl From<DoubleError> for DoubleError {
    fn from(err: DoubleError) -> DoubleError {
        DoubleError::Parse(err)
    }
}

fn double_first_v1(vec: Vec<&str>) -> ResultBox<i32> {
    vec.first()
        .ok_or_else(|| EmptyVec.into())  // 装箱
        .and_then(|s| {
            s.parse::<i32>()
                .map_err(|e| e.into())  // 装箱
                .map(|i| 2 * i)
        })
}

fn print_v1(result: ResultBox<i32>) {
    match result {
        Ok(n)  => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

// ? 的其他用法
// 注意在上一个例子中，我们调用 parse 后总是立即把错误从标准库错误 map 到装箱的错误。
//
//
// .and_then(|s| s.parse::<i32>()
// .map_err(|e| e.into())
// 因为这个操作很简单常见，如果有省略写法就好了。and_then 不够灵活，所以不能实现 这样的写法。不过，我们可以使用 ? 来代替它。
//
// 之前我们说 ? 就是 “要么 unwrap 要么 return Err(error)”，这大部分是对的，但 事实上 ? 是 “要么 unwrap 要么 return Err(From::from(err))”。From::from 是 不同类型间的转换工具，也就是说，如果在错误能够转换成返回类型的地方使用 ?，它就 会自动转换成返回类型。
//
// 这里，我们使用 ? 重写之前的例子。这样，只要为我们的错误类型实现 From::from，就 可以不再使用
// 这里的结构和之前一样，但是这次没有把所有的 `Results` 和 `Options` 串起来，
// 而是使用 `?` 立即得到内部值。
fn double_first_v2(vec: Vec<&str>) -> Result<i32> {
    let first = vec.first().ok_or(EmptyVec)?;
    let parsed = first.parse::<i32>()?;
    Ok(2 * parsed)
}

fn print_v2(result: Result<i32>) {
    match result {
        Ok(n)  => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));
}
