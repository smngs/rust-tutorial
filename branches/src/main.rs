use std::println;

fn main() {
    // let number = 3;
    let number = 7;

    // if の条件は bool 型じゃないとエラー
    // error[E0308]: mismatched types
    if number < 5 {
        println!("Condition was true.");
    } else {
        println!("Condition was false.");
    }

    if number != 0 {
        println!("number was something other than zero.");
    }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    
    let num = if condition {5} else {6};
    // let num = if condition {5} else {"six"}; // Error
    // error[E0308]: `if` and `else` have incompatible types

    println!("The value of number is {}", num);


    let mut count = 0;

    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                // 外側の loop にラベル 'counting_up を付与しておくことで，一気に loop
                // を抜けられる！ 便利！
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }

    println!("End count = {}", count);
    
    // println!("{}", remaining); // Error
    // スコープ外（ライフタイムが切れてる）


    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    
    println!("LIFTOFF!!");

    // Collection を舐めていく

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    // ここを index < 6 としてもコンパイラは怒ってくれない（実行時エラーになる）
    while index < 5 {
        println!("the value is {}", a[index]);
        index += 1;
    }

    // C++ でいうところの範囲 for みたいなこともできる
    // 安全でヨシ！
    for element in a {
        println!("the value is {}", element)
    }

    for number in (1..4).rev() {
        println!("{}!", number);
        // number は mutable じゃないから書き換えられない（シャドーイングはできる →
        // あんぜんでよい！）
    }
    println!("LIFTOFF!!!");
}
