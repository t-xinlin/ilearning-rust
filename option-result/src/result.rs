use std::num::ParseIntError;

// 为带有错误类型 `ParseIntError` 的 `Result` 定义一个泛型别名。
pub type ParseIntResult<T> = Result<T, ParseIntError>;

// 修改了上一节中的返回类型，现在使用模式匹配而不是 `unwrap()`。
fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    match first_number_str.parse::<i32>() {
        Ok(first_number)  => {
            match second_number_str.parse::<i32>() {
                Ok(second_number)  => {
                    Ok(first_number * second_number)
                },
                Err(e) => Err(e),
            }
        },
        Err(e) => Err(e),
    }
}
// 就像 `Option` 那样，我们可以使用 `map()` 之类的组合算子。
// 除去写法外，这个函数与上面那个完全一致，它的作用是：
// 如果值是合法的，计算其乘积，否则返回错误。
fn multiply_v2(first_number_str: &str, second_number_str: &str) -> ParseIntResult<i32> {
    first_number_str.parse::<i32>().and_then(|first_number|{
        second_number_str.parse::<i32>().map(|second_number|{
            first_number * second_number
        })
    })
}

fn print(result: ParseIntResult<i32>) {
    match result {
        Ok(n)  => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}


fn test_result() {
    // 这种情况下仍然会给出正确的答案。
    // let twenty = multiply("10", "2");
    let twenty = multiply_v2("10", "2");
    print(twenty);

    // 这种情况下就会提供一条更有用的错误信息。
    // let tt = multiply("t", "2");
    let tt = multiply_v2("t", "2");
    print(tt);

    multiply_v4("t", "2");
}


// 提前返回
//
// 这也就是说，如果发生错误，我们可以停止函数的执行然后返回错误。对有些人来说，这样 的代码更好写，更易读。这次我们使用提前返回改写之前的例子：
fn multiply_v3(first_number_str: &str, second_number_str: &str) -> ParseIntResult<i32> {
    let first_number = match first_number_str.parse::<i32>() {
        Ok(first_number)  => first_number,
        Err(e) => return Err(e),
    };

    let second_number = match second_number_str.parse::<i32>() {
        Ok(second_number)  => second_number,
        Err(e) => return Err(e),
    };

    Ok(first_number * second_number)
}


// 引入 ?
// 有时我们只是想 unwrap 且避免产生 panic。到现在为止，对 unwrap 的错误处理都在强迫 我们一层层地嵌套，然而我们只是想把里面的变量拿出来。? 正是为这种情况准备的。
//
// 当找到一个 Err 时，可以采取两种行动：
//
// panic!，不过我们已经决定要尽可能避免 panic 了。
// 返回它，因为 Err 就意味着它已经不能被处理了。
// ? 几乎1 就等于一个会返回 Err 而不是 panic 的 unwrap。我们来看看 怎样简化之前使用组合算子的例子：
fn multiply_v4(first_number_str: &str, second_number_str: &str) -> ParseIntResult<i32> {
    let first_number = first_number_str.parse::<i32>()?;
    println!("run second");
    let second_number = second_number_str.parse::<i32>()?;
    Ok(first_number * second_number)

}


// try! 宏
// 在 ? 出现以前，相同的功能是使用 try! 宏完成的。现在我们推荐使用 ? 运算符，但是 在老代码中仍然会看到 try!。如果使用 try! 的话，上一个例子中的 multiply 函数 看起来会像是这样：
//
fn multiply_v5(first_number_str: &str, second_number_str: &str) -> ParseIntResult<i32> {
    // let first_number = try!(first_number_str.parse::<i32>());
    // let second_number = try!(second_number_str.parse::<i32>());
    //
    // Ok(first_number * second_number)
    Ok(0)
}

fn main() {
    test_result();
    return;
    // 这种情形下仍然会给出正确的答案。
    let twenty = multiply("10", "2");
    print(twenty);

    // 这种情况下就会提供一条更有用的错误信息。
    let tt = multiply("t", "2");
    print(tt);
}
