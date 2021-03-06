fn main() {}

// Into/From 及其在 String 和 &str 互转上的应用
// std::convert 下面，有两个 Trait，Into/From，它们是一对孪生姐妹。它们的作用是配合泛型，进行一些设计上的归一化处理。
//
// 它们的基本形式为： From<T> 和 Into<T>。
#[cfg(test)]
mod test {

    // From
    // 对于类型为 U 的对象 foo，如果它实现了 From<T>，那么，可以通过 let foo = U::from(bar) 来生成自己。这里，bar 是类型为 T 的对象。
    //
    // 下面举一例，因为 String 实现了 From<&str>，所以 String 可以从 &str 生成。
    #[test]
    fn test1() {
        let string = "hello".to_string();
        let other_string = String::from("hello");
        assert_eq!(string, other_string);
    }

    #[test]
    fn test2(){
        fn is_hello<T: Into<Vec<u8>>>(s: T) {
            let bytes = b"hello".to_vec();
            assert_eq!(bytes, s.into());
        }

        let s = "hello".to_string();
        is_hello(s);
    }

    // 参数类型为 S， 是一个泛型参数，表示可以接受不同的类型。
    // S: Into<String> 表示 S 类型必须实现了 Into<String>（约束）。而 &str 类型，符合这个要求。因此 &str 类型可以直接传进来。
    //
    // 而 String 本身也是实现了 Into<String> 的。当然也可以直接传进来。
    //
    // 然后，下面 name: name.into() 这里也挺神秘的。它的作用是将 name 转换成 String 类型的另一个对象。
    // 当 name 是 &str 时，它会转换成 String 对象，会做一次字符串的拷贝（内存的申请、复制）。
    // 而当 name 本身是 String 类型时，name.into() 不会做任何转换，代价为零（有没有恍然大悟）。
    // 根据参考资料，上述内容通过下面三式获得
    // impl<'a> From<&'a str> for String {}
    // impl<T> From<T> for T {}
    // impl<T, U> Into<U> for T where U: From<T> {}
    #[test]
    fn test3(){

        // 在我们设计库的 API 的时候，经常会遇到一个恼人的问题，函数参数如果定为 String，则外部传入实参的时候，对字符串字面量，必须要做 .to_string() 或 .to_owned() 转换，参数一多，就是一件又乏味又丑的事情。（而反过来设计的话，对初学者来说，又会遇到一些生命周期的问题，比较麻烦，这个后面论述）
        //
        // 那存不存在一种方法，能够使传参又能够接受 String 类型，又能够接受 &str 类型呢？答案就是泛型。而仅是泛型的话，太宽泛。因此，标准库中，提供了 Into<T> 来为其做约束，以便方便而高效地达到我们的目的。
        //
        // 比如，我们有如下结构体
        struct Person {
            name: String,
        }

        impl Person {
            fn new<S: Into<String>>(name: S) -> Person {
                Person { name: name.into() }
            }
        }

        let person = Person::new("Herman");
        let person = Person::new("Herman".to_string());
    }

}

// AsRef 和 AsMut
// std::convert 下面，还有另外两个 Trait，AsRef/AsMut，它们功能是配合泛型，在执行引用操作的时候，进行自动类型转换。
// 这能够使一些场景的代码实现得清晰漂亮，大家方便开发。


#[cfg(test)]
mod test_as_ref {
    // AsRef
    // AsRef 提供了一个方法 .as_ref()。
    //
    // 对于一个类型为 T 的对象 foo，如果 T 实现了 AsRef<U>，那么，foo 可执行 .as_ref() 操作，即 foo.as_ref()。操作的结果，我们得到了一个类型为 &U 的新引用。
    //
    // 注：
    //
    // 与 Into<T> 不同的是，AsRef<T> 只是类型转换，foo 对象本身没有被消耗；
    // T: AsRef<U> 中的 T，可以接受 资源拥有者（owned）类型，共享引用（shared referrence）类型 ，可变引用（mutable referrence）类型。
    #[test]
    fn test_AsRef(){
        fn is_hello<T: AsRef<str>>(s: T) {
            assert_eq!("hello", s.as_ref());
        }

        let s = "hello";
        is_hello(s);

        let s = "hello".to_string();
        is_hello(s);
    }

    // AsMut
    // AsMut<T> 提供了一个方法 .as_mut()。它是 AsRef<T> 的可变（mutable）引用版本。
    //
    // 对于一个类型为 T 的对象 foo，如果 T 实现了 AsMut<U>，那么，foo 可执行 .as_mut() 操作，即 foo.as_mut()。
    // 操作的结果，我们得到了一个类型为 &mut U 的可变（mutable）引用。
    //
    // 注：在转换的过程中，foo 会被可变（mutable）借用。
}
#[cfg(test)]
mod test_borrow {
    // Borrow 提供了一个方法 .borrow()。
    //
    // 对于一个类型为 T 的值 foo，如果 T 实现了 Borrow<U>，那么，foo 可执行 .borrow() 操作，即 foo.borrow()。操作的结果，我们得到了一个类型为 &U 的新引用。
    //
    // Borrow 可以认为是 AsRef 的严格版本，它对普适引用操作的前后类型之间附加了一些其它限制。
    //
    // Borrow 的前后类型之间要求必须有内部等价性。不具有这个等价性的两个类型之间，不能实现 Borrow。
    //
    // AsRef 更通用，更普遍，覆盖类型更多，是 Borrow 的超集。
    #[test]
    fn test(){
        use std::borrow::Borrow;

        fn check<T: Borrow<str>>(s: T) {
            assert_eq!("Hello", s.borrow());
        }

        let s = "Hello".to_string();

        check(s);

        let s = "Hello";

        check(s);
    }

    // BorrowMut
    // use std::borrow::BorrowMut;
    //
    // BorrowMut<T> 提供了一个方法 .borrow_mut()。它是 Borrow<T> 的可变（mutable）引用版本。
    //
    // 对于一个类型为 T 的值 foo，如果 T 实现了 BorrowMut<U>，那么，foo 可执行 .borrow_mut() 操作，即 foo.borrow_mut()。
    // 操作的结果我们得到类型为 &mut U 的一个可变（mutable）引用。
    //
    // 注：在转换的过程中，foo 会被可变（mutable）借用。
    //
    // ToOwned
    // use std::borrow::ToOwned;
    //
    // ToOwned 为 Clone 的普适版本。它提供了 .to_owned() 方法，用于类型转换。
    //
    // 有些实现了 Clone 的类型 T 可以从引用状态实例 &T 通过 .clone() 方法，生成具有所有权的 T 的实例。
    // 但是它只能由 &T 生成 T。而对于其它形式的引用，Clone 就无能为力了。
    //
    // 而 ToOwned trait 能够从任意引用类型实例，生成具有所有权的类型实例
}
