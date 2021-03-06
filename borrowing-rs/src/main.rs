// 借用
// 多数情况下，我们更希望能访问数据，同时不取得其所有权。为实现这点，Rust 使用 了借用（borrowing）机制。对象可以通过引用（&T）来传递，从而取代通过 值（T）来传递。
//
// 编译器（通过借用检查）静态地保证了引用总是指向有效的对象。也就是说，当存在 引用指向一个对象时，该对象不能被销毁。
// 此函数取得一个 box 的所有权并销毁它
fn eat_box_i32(boxed_i32: Box<i32>) {
    println!("Destroying box that contains {}", boxed_i32);
}

// 此函数借用了一个 i32 类型
fn borrow_i32(borrowed_i32: &i32) {
    println!("This int is: {}", borrowed_i32);
}

fn main1() {
    // 创建一个装箱的 i32 类型，以及一个存在栈中的 i32 类型。
    let boxed_i32 = Box::new(5_i32);
    let stacked_i32 = 6_i32;

    // 借用了 box 的内容，但没有取得所有权，所以 box 的内容之后可以再次借用。
    // 译注：请注意函数自身就是一个作用域，因此下面两个函数运行完成以后，
    // 在函数中临时创建的引用也就不复存在了。
    borrow_i32(&boxed_i32);
    borrow_i32(&stacked_i32);

    {
        // 取得一个对 box 中数据的引用
        let _ref_to_i32: &i32 = &boxed_i32;

        // 报错！
        // 当 `boxed_i32` 里面的值之后在作用域中被借用时，不能将其销毁。
        eat_box_i32(boxed_i32);
        // 改正 ^ 注释掉此行

        // 在 `_ref_to_i32` 里面的值被销毁后，尝试借用 `_ref_to_i32`
        //（译注：如果此处不借用，则在上一行的代码中，eat_box_i32(boxed_i32)可以将 `boxed_i32` 销毁。）
        // borrow_i32(_ref_to_i32);
        // `_ref_to_i32` 离开作用域且不再被借用。
    }

    // `boxed_i32` 现在可以将所有权交给 `eat_i32` 并被销毁。
    //（译注：能够销毁是因为已经不存在对 `boxed_i32` 的引用）
    // eat_box_i32(boxed_i32);
}


#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Book {
    // `&'static str` 是一个对分配在只读内存区的字符串的引用
    author: &'static str,
    title: &'static str,
    year: u32,
}

// 此函数接受一个对 Book 类型的引用
fn borrow_book(book: &Book) {
    println!("I immutably borrowed {} - {} edition", book.title, book.year);
}

// 此函数接受一个对可变的 Book 类型的引用，它把年份 `year` 改为 2014 年
fn new_edition(book: &mut Book) {
    book.year = 2014;
    println!("I mutably borrowed {} - {} edition", book.title, book.year);
}

fn main33() {
    // 创建一个名为 `immutabook` 的不可变的 Book 实例
    let immutabook = Book {
        // 字符串字面量拥有 `&'static str` 类型
        author: "Douglas Hofstadter",
        title: "Gödel, Escher, Bach",
        year: 1979,
    };

    // 创建一个 `immutabook` 的可变拷贝，命名为 `mutabook`
    let mut mutabook = immutabook;

    // 不可变地借用一个不可变对象
    borrow_book(&immutabook);

    // 不可变地借用一个可变对象
    borrow_book(&mutabook);

    // 可变地借用一个可变对象
    new_edition(&mut mutabook);

    // 报错！不能可变地借用一个不可变对象
    // new_edition(&mut immutabook);
    // 改正 ^ 注释掉此行
}

// 冻结
// 当数据被不可变地借用时，它还会冻结（freeze）。已冻结的数据无法通过原始 对象来修改，直到对这些数据的所有引用离开作用域为止。
fn main2() {
    // let mut _mutable_integer = 7i32;
    //
    // {
    //     // 借用 `_mutable_integer`
    //     let large_integer = &_mutable_integer;
    //
    //     // 报错！`_mutable_integer` 在本作用域被冻结
    //     _mutable_integer = 50;
    //     // 改正 ^ 注释掉此行
    //
    //     println!("Immutably borrowed {}", large_integer);
    //
    //     // `large_integer` 离开作用域
    // }
    //
    // // 正常运行！`_mutable_integer` 在这作用域没有冻结
    // _mutable_integer = 3;
}
// 别名使用
// 数据可以进行多次不可变借用，但是在不可变借用的期间，原始数据不可进行可变借用。另 一方面，在同一时刻内只允许有一个可变借用。只有在可变引用离开作用域之后，原始 数据才可再次被借用。

struct Point { x: i32, y: i32, z: i32 }

fn main4() {
    let mut point = Point { x: 0, y: 0, z: 0 };

    {
        let borrowed_point = &point;
        let another_borrow = &point;

        // 通过引用和原始所有者来访问数据
        println!("Point has coordinates: ({}, {}, {})", borrowed_point.x, another_borrow.y, point.z);

        // 报错！不能可变地借用 `point` ，因为现在它有不可变的借用。
        //let mutable_borrow = &mut point;
        // 试一试 ^ 取消此行注释。

        // 此处再次使用被借用的值
        println!("Point has coordinates: ({}, {}, {})", borrowed_point.x, another_borrow.y, point.z);

        // 不可变引用离开作用域
    }

    {
        let mutable_borrow = &mut point;

        // 通过可变引用来改变数据
        mutable_borrow.x = 5;
        mutable_borrow.y = 2;
        mutable_borrow.z = 1;

        // 报错！不能不可变地借用 `point`，因为现在它有可变的借用。
        //let y = &point.y;
        // 试一试 ^ 取消此行注释。

        // 报错！不能打印，因为 `println!` 会创建一个不可变引用。
        //println!("Point Z coordinate is {}", point.z);
        // 试一试 ^ 取消此行注释。

        // 可以工作！可变引用可以作为不可变的传给 `println!`。
        println!("Point has coordinates: ({}, {}, {})", mutable_borrow.x, mutable_borrow.y, mutable_borrow.z);

        // 可变引用离开作用域
    }

    // 现在又可以不可变地借用 `point` 了。
    let borrowed_point = &point;
    println!("Point now has coordinates: ({}, {}, {})", borrowed_point.x, borrowed_point.y, borrowed_point.z);
}

// ref 模式
// 在通过 let 绑定来进行模式匹配或解构时，ref 关键字可用来创建结构体/元组的 字段的引用。下面的例子展示了几个实例，可看到 ref 的作用：
#[derive(Clone, Copy)]
struct Point2 { x: i32, y: i32 }

fn main() {
    // let c = 'Q';
    //
    // // 赋值语句中左边的 `ref` 关键字等价于右边的 `&` 符号。
    // let ref ref_c1 = c;
    // let ref_c2 = &c;
    //
    // println!("ref_c1 equals ref_c2: {}", *ref_c1 == *ref_c2);
    //
    // let point = Point2 { x: 0, y: 0 };
    //
    // // 在解构一个结构体时 `ref` 同样有效。
    // let _copy_of_x = {
    //     // `ref_to_x` 是一个指向 `point` 的 `x` 字段的引用。
    //     let Point2 { x: ref ref_to_x, y: _ } = point;
    //
    //     // 返回一个 `point` 的 `x` 字段的拷贝。
    //     *ref_to_x
    // };
    //
    // // `point` 的可变拷贝
    // let mut mutable_point = point;
    //
    // {
    //     // `ref` 可以与 `mut` 结合以创建可变引用。
    //     let Point2 { x: _, y: ref mut mut_ref_to_y } = mutable_point;
    //
    //     // 通过可变引用来改变 `mutable_point` 的字段 `y`。
    //     *mut_ref_to_y = 1;
    // }
    //
    // println!("point is ({}, {})", point.x, point.y);
    // println!("mutable_point is ({}, {})", mutable_point.x, mutable_point.y);
    //
    // // 包含一个指针的可变元组
    // let mut mutable_tuple = (Box::new(5u32), 3u32);
    //
    // {
    //     // 解构 `mutable_tuple` 来改变 `last` 的值。
    //     let (_, ref mut last) = mutable_tuple;
    //     *last = 2u32;
    // }
    //
    // println!("tuple is {:?}", mutable_tuple);
}
