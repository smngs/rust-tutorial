#[warn(unused_variables)]
fn main() {
    let mut s1 = String::from("Hello");
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
    
    // mutable な借用を行う際は，&mut をつける（可変参照）
    change(&mut s1);

    // 一つの変数につき，可変参照はスコープの中に一個だけしか持てない
    // （可変参照を一つでも持ったら，もうほかに参照は持てない）
    let r1 = &mut s1;
    // let r2 = &mut s1; // これだとエラー
    // println!("{}, {}", r1, r2);

    // 例えばダングリングポインタなんかはいい感じに阻止してくれる
    // let reference_to_nothing = dangle(); // エラー
    let s3 = no_dangle();
}

// Copy トレイトを有していない型の参照を得ることが可能
// このとき，calculate_length に所有権は move されない
fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn dangle() -> &String {
//     let s = String::from("Hello");
//     &s
//     // ここで s のスコープは終わるから開放される →s のポインタだけが main に行っちゃうから危険！
// }

fn no_dangle() -> String {
    let s = String::from("Hello");
    s
}
