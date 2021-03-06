#![allow(dead_code)]

#[derive(Debug)]
pub enum Food {
    Apple,
    Carrot,
    Potato,
}
#[derive(Debug)] enum Day { Monday, Tuesday, Wednesday }

#[derive(Debug)]
pub struct Peeled(Food);
#[derive(Debug)]
pub struct Chopped(Food);
#[derive(Debug)]
pub struct Cooked(Food);
// 组合算子：map
// match 是处理 Option 的一个可用的方法，但你会发现大量使用它会很繁琐，特别是当 操作只对一种输入是有效的时。这时，可以使用组合算子（combinator），以 模块化的风格来管理控制流。
//
// Option 有一个内置方法 map()，这个组合算子可用于 Some -> Some 和 None -> None 这样的简单映射。多个不同的 map() 调用可以串起来，这样更加灵活。
//
// 在下面例子中，process() 轻松取代了前面的所有函数，且更加紧凑。

// 削皮。如果没有食物，就返回 `None`。否则返回削好皮的食物。
fn peel(food: Option<Food>) -> Option<Peeled> {
    match food {
        Some(food) => Some(Peeled(food)),
        None => None,
    }
}

// 切食物。如果没有食物，就返回 `None`。否则返回切好的食物。
fn chop(peeled: Option<Peeled>) -> Option<Chopped> {
    match peeled {
        Some(Peeled(food)) => Some(Chopped(food)),
        None => None,
    }
}

// 烹饪食物。这里，我们使用 `map()` 来替代 `match` 以处理各种情况。
fn cook(chopped: Option<Chopped>) -> Option<Cooked> {
    chopped.map(|Chopped(food)| {
        println!("====cooked run.");
        Cooked(food)
    })
}

// 这个函数会完成削皮切块烹饪一条龙。我们把 `map()` 串起来，以简化代码。
fn process(food: Option<Food>) -> Option<Cooked> {
    food.map(|f| Peeled(f))
        .map(|Peeled(f)| Chopped(f))
        .map(|Chopped(f)| Cooked(f))
}

// 在尝试吃食物之前确认食物是否存在是非常重要的！
fn eat(food: Option<Cooked>) {
    match food {
        Some(food) => println!("Mmm. I love {:?}", food),
        None => println!("Oh no! It wasn't edible."),
    }
}

// 组合算子：and_then
// map() 以链式调用的方式来简化 match 语句。然而，如果以返回类型是 Option<T> 的函数作为 map() 的参数，会导致出现嵌套形式 Option<Option<T>>。这样多层串联 调用就会变得混乱。所以有必要引入 and_then()，在某些语言中它叫做 flatmap。
//
// and_then() 使用被 Option 包裹的值来调用其输入函数并返回结果。 如果 Option 是 None，那么它返回 None。
//
// 在下面例子中，cookable_v2() 会产生一个 Option<Food>。如果在这里使用 map() 而不是 and_then() 将会得到 Option<Option<Food>>，这对 eat() 来说是一个 无效类型。

// 我们没有制作寿司所需的原材料（ingredient）（有其他的原材料）。
fn have_ingredients(food: Food) -> Option<Food> {
    match food {
        Food::Carrot => None,
        _ => Some(food),
    }
}

// 我们拥有全部食物的食谱，除了法国蓝带猪排（Cordon Bleu）的。
fn have_recipe(food: Food) -> Option<Food> {
    match food {
        Food::Carrot => None,
        _ => Some(food),
    }
}

// 要做一份好菜，我们需要原材料和食谱。
// 我们可以借助一系列 `match` 来表达这个逻辑：
fn cookable_v1(food: Food) -> Option<Food> {
    match have_ingredients(food) {
        None => None,
        Some(food) => match have_recipe(food) {
            None => None,
            Some(food) => Some(food),
        },
    }
}

// 也可以使用 `and_then()` 把上面的逻辑改写得更紧凑：
fn cookable_v2(food: Food) -> Option<Food> {
    have_ingredients(food).and_then(have_recipe)
}

fn eatppp(food: Food, day: Day) {
    match cookable_v2(food) {
        Some(food) => println!("Yay! On {:?} we get to eat {:?}.", day, food),
        None       => println!("Oh no. We don't get to eat on {:?}?", day),
    }
}

fn main11() {
    let apple = Some(Food::Apple);
    // let carrot = Some(Food::Carrot);
    // let potato = None;

    let cooked_apple = cook(chop(peel(apple)));
    // let cooked_carrot = cook(chop(peel(carrot)));
    //
    // // 现在让我们试试看起来更简单的 `process()`。
    // let cooked_potato = process(potato);
    //
    // eat(cooked_apple);
    // eat(cooked_carrot);
    // eat(cooked_potato);
}
