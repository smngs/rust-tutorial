// 関数の引数に対しては明示的な型注釈が必要
fn another_function(x: i32) {
    // 前方定義が可能
    println!("Another function.");
    println!("The value of x is {}", x)
}

fn main() {
    println!("Hello, world!");

    another_function(5);
    print_labeled_measurement(5, 'h');

    // 式 (expression) と文 (statement)
    let x = five();
    println!("The value of x is {}", x);

    let x = plus_one(x);
    println!("The value of x is {}", x);
}

fn five() -> i32 {
    5 // 最後の式を暗黙的に return
}

fn plus_one(x: i32) -> i32 {
    return x + 1;
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is {}{}", value, unit_label);
}
