fn main() {
    let mut v = vec![1, 2, 3];
    v.push(6);
    v.push(7);
    println!("Hello, vector{:?},{:?}", "Hillo", v);

    match v.get(10) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // 为什么第一个元素的引用会关心 vector 结尾的变化？不能这么做的原因是由于 vector 的工作方式：在 vector 的结尾增加新元素时，在没有足够空间将所有所有元素依次相邻存放的情况下，可能会要求分配新内存并将老的元素拷贝到新的空间中。这时，第一个元素的引用就指向了被释放的内存。借用规则阻止程序陷入这种状况。
    // let first = &v[0];
    // v.push(6);
    // println!("The first element is: {}", first);

    let first = v[0];
    v.push(6);
    println!("The first element is: {}", first);

    for i in &v {
        println!("for range {:?}", i)
    }

    // 遍历修改值
    for i in &mut v {
        *i += 50;
    }
    for i in &v {
        println!("after modify for range {:?}", i)
    }

    // 使用枚举来储存多种类型
    // 在本章的开始，我们提到 vector 只能储存相同类型的值。这是很不方便的；绝对会有需要储存一系列不同类型的值的用例。幸运的是，枚举的成员都被定义为相同的枚举类型，所以当需要在 vector 中储存不同类型值时，我们可以定义并使用一个枚举！
    //
    // 例如，假如我们想要从电子表格的一行中获取值，而这一行的有些列包含数字，有些包含浮点值，还有些是字符串。我们可以定义一个枚举，其成员会存放这些不同类型的值，同时所有这些枚举成员都会被当作相同类型，那个枚举的类型。接着可以创建一个储存枚举值的 vector，这样最终就能够储存不同类型的值了。示例 ：
    //
    //

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
