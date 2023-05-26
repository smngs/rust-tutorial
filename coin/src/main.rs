#[derive(Debug)]
enum UsState {
    Alabama, 
    Alaska
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    // match は包括的なので，例えば以下のコードだとエラー
    // match x {
    //     Some(i) => Some(i + 1),
    //     // missing match arm: None not covered.
    // }

    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let penny_coin = Coin::Penny;
    let quarter_alabama_coin = Coin::Quarter(UsState::Alabama);

    println!("{}", value_in_cents(penny_coin));
    println!("{}", value_in_cents(quarter_alabama_coin));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let some_u8_value = Some(4u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => println!("not three")
    }

    // 以下の構文と同義
    if let Some(3) = some_u8_value {
        println!("three")
    } else {
        println!("not three");
    }
}
