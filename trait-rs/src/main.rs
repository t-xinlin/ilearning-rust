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
// Deref
// Deref 是 deref 操作符 * 的 trait，比如 *v。
//
// 一般理解，*v 操作，是 &v 的反向操作，即试图由资源的引用获取到资源的拷贝（如果资源类型实现了 Copy），或所有权（资源类型没有实现 Copy）。
//
// Rust 中，本操作符行为可以重载。这也是 Rust 操作符的基本特点。本身没有什么特别的。
//
// 强制隐式转换（coercion）
// Deref 神奇的地方并不在本身 解引 这个意义上，Rust 的设计者在它之上附加了一个特性：强制隐式转换，这才是它神奇之处。
//
// 这种隐式转换的规则为：
//
// 一个类型为 T 的对象 foo，如果 T: Deref<Target=U>，那么，相关 foo 的某个智能指针或引用（比如 &foo）在应用的时候会自动转换成 &U。
//
// 粗看这条规则，貌似有点类似于 AsRef，而跟 解引 似乎风马牛不相及。实际里面有些玄妙之处。
//
// Rust 编译器会在做 *v 操作的时候，自动先把 v 做引用归一化操作，即转换成内部通用引用的形式 &v，整个表达式就变成 *&v。这里面有两种情况：
//
// 把其它类型的指针（比如在库中定义的，Box, Rc, Arc, Cow 等），转成内部标准形式 &v；
// 把多重 & （比如：&&&&&&&v），简化成 &v（通过插入足够数量的 * 进行解引）。
// 所以，它实际上在解引用之前做了一个引用的归一化操作。
//
// 为什么要转呢？ 因为编译器设计的能力是，只能够对 &v 这种引用进行解引用。其它形式的它不认识，所以要做引用归一化操作。
//
// 使用引用进行过渡也是为了能够防止不必要的拷贝。
//
// 下面举一些例子：
#[cfg(test)]
mod test_eref {
    #[test]
    fn test() {
        fn foo(s: &str) {
            // borrow a string for a second
        }

        // String implements Deref<Target=str>
        let owned = "Hello".to_string();

        // therefore, this works:
        foo(&owned);
    }
    #[test]
    fn test_v1() {
        // 因为 String 实现了 Deref<Target=str>。

        use std::rc::Rc;

        fn foo(s: &str) {
            // borrow a string for a second
        }

        // String implements Deref<Target=str>
        let owned = "Hello".to_string();
        let counted = Rc::new(owned);

        // therefore, this works:
        foo(&counted);
    }

    #[test]
    fn test_v2() {
        // 因为 Rc<T> 实现了 Deref<Target=T>。

        fn foo(s: &[i32]) {
            // borrow a slice for a second
        }

        // Vec<T> implements Deref<Target=[T]>
        let owned = vec![1, 2, 3];

        foo(&owned);
    }

    #[test]
    fn test_v3() {
        // 因为 Vec<T> 实现了 Deref<Target=[T]>。

        struct Foo;

        impl Foo {
            fn foo(&self) {
                println!("Foo");
            }
        }

        let f = &&Foo;

        f.foo();
        (&f).foo();
        (&&f).foo();
        (&&&&&&&&f).foo();
    }

    // 上面那几种函数的调用，效果是一样的。
    // coercion 的设计，是 Rust 中仅有的类型隐式转换，设计它的目的，是为了简化程序的书写，让代码不至于过于繁琐。
    // 把人从无尽的类型细节中解脱出来，让书写 Rust 代码变成一件快乐的事情。
}

// Cow
// 直译为奶牛！开玩笑。 Cow 是一个枚举类型，通过 use std::borrow::Cow; 引入。它的定义是 Clone-on-write，即写时克隆。本质上是一个智能指针。
//
// 它有两个可选值：
//
// Borrowed，用于包裹对象的引用（通用引用）；
// Owned，用于包裹对象的所有者；
// Cow 提供
//
// 对此对象的不可变访问（比如可直接调用此对象原有的不可变方法）；
// 如果遇到需要修改此对象，或者需要获得此对象的所有权的情况，Cow 提供方法做克隆处理，并避免多次重复克隆。
// Cow 的设计目的是提高性能（减少复制）同时增加灵活性，因为大部分情况下，业务场景都是读多写少。利用 Cow，可以用统一，规范的形式实现，需要写的时候才做一次对象复制。这样就可能会大大减少复制的次数。
//
// 它有以下几个要点需要掌握：
//
// Cow<T> 能直接调用 T 的不可变方法，因为 Cow 这个枚举，实现了 Deref；
// 在需要写 T 的时候，可以使用 .to_mut() 方法得到一个具有所有权的值的可变借用；
// 注意，调用 .to_mut() 不一定会产生克隆；
// 在已经具有所有权的情况下，调用 .to_mut() 有效，但是不会产生新的克隆；
// 多次调用 .to_mut() 只会产生一次克隆。
// 在需要写 T 的时候，可以使用 .into_owned() 创建新的拥有所有权的对象，这个过程往往意味着内存拷贝并创建新对象；
// 如果之前 Cow 中的值是借用状态，调用此操作将执行克隆；
// 本方法，参数是self类型，它会“吃掉”原先的那个对象，调用之后原先的对象的生命周期就截止了，在 Cow 上不能调用多次；

#[cfg(test)]
mod cow {
    // .to_mut() 举例
    #[test]
    fn test_v1() {
        use std::borrow::Cow;

        let mut cow: Cow<[_]> = Cow::Owned(vec![1, 2, 3]);

        let hello = cow.to_mut();

        assert_eq!(hello, &[1, 2, 3]);
    }
    // .into_owned() 举例
    #[test]
    fn test_v1() {
        use std::borrow::Cow;

        let cow: Cow<[_]> = Cow::Owned(vec![1, 2, 3]);

        let hello = cow.into_owned();

        assert_eq!(vec![1, 2, 3], hello);
    }

    // 综合举例
    #[test]
    fn test_v1() {
        use std::borrow::Cow;

        fn abs_all(input: &mut Cow<[i32]>) {
            for i in 0..input.len() {
                let v = input[i];
                if v < 0 {
                    // clones into a vector the first time (if not already owned)
                    input.to_mut()[i] = -v;
                }
            }
        }
    }

    // Cow 在函数返回值上的应用实例
    #[test]
    fn test_v2() {
        // 题目：写一个函数，过滤掉输入的字符串中的所有空格字符，并返回过滤后的字符串。
        //
        // 对这个简单的问题，不用思考，我们都可以很快写出代码：

        fn remove_spaces(input: &str) -> String {
            let mut buf = String::with_capacity(input.len());

            for c in input.chars() {
                if c != ' ' {
                    buf.push(c);
                }
            }

            buf
        }
        //
        // 设计函数输入参数的时候，我们会停顿一下，这里，用 &str 好呢，还是 String 好呢？思考一番，从性能上考虑，有如下结论：
        //
        // 如果使用 String， 则外部在调用此函数的时候，
        // 如果外部的字符串是 &str，那么，它需要做一次克隆，才能调用此函数；
        // 如果外部的字符串是 String，那么，它不需要做克隆，就可以调用此函数。但是，一旦调用后，外部那个字符串的所有权就被 move 到此函数中了，外部的后续代码将无法再使用原字符串。
        // 如果使用 &str，则不存在上述两个问题。但可能会遇到生命周期的问题，需要注意。
        // 继续分析上面的例子，我们发现，在函数体内，做了一次新字符串对象的生成和拷贝。
        //
        // 让我们来仔细分析一下业务需求。最坏的情况下，如果字符串中没有空白字符，那最好是直接原样返回。这种情况做这样一次对象的拷贝，完全就是浪费了。
        //
        // 于是我们心想改进这个算法。很快，又遇到了另一个问题，返回值是 String 的嘛，我不论怎样，要把 &str 转换成 String 返回，始终都要经历一次复制。于是我们快要放弃了。
        //
        // 好吧，Cow君这时出马了。奶牛君很快写出了如下代码：

        use std::borrow::Cow;

        fn remove_spaces_v1<'a>(input: &'a str) -> Cow<'a, str> {
            if input.contains(' ') {
                let mut buf = String::with_capacity(input.len());

                for c in input.chars() {
                    if c != ' ' {
                        buf.push(c);
                    }
                }

                return Cow::Owned(buf);
            }

            return Cow::Borrowed(input);
        }
    }

    // 完美解决了业务逻辑与返回值类型冲突的问题。本例可细细品味。
    //
    // 外部程序，拿到这个 Cow 返回值后，按照我们上文描述的 Cow 的特性使用就好了。
}
