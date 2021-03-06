mod result;
mod option;
mod option_result;

use option::Food;
use option_result::run_option_result;

fn main() {
    run_option_result();
    return;
    // let apple = Some(option: Food::Apple);
    // let carrot = Some(Food::Carrot);
    // let potato = None;

    // let cooked_apple = cook(chop(peel(apple)));
    // let cooked_carrot = cook(chop(peel(carrot)));
    //
    // // 现在让我们试试看起来更简单的 `process()`。
    // let cooked_potato = process(potato);
    //
    // eat(cooked_apple);
    // eat(cooked_carrot);
    // eat(cooked_potato);
}
