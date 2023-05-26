fn main() {
    let mut s = String::from("hello world");
    // let word = first_word(&s);

    // s.clear();
    // println!("{}", word);
    // もう word はないわけだから，usize で番号だけ返すのはナンセンス
    
    // 文字列スライス: String への一部の参照
    // あくまで参照
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{}, {}", hello, world);

    let word = first_word(&s);
    println!("{}", word);

    s.clear(); // ここで clear() に可変参照が取られる → s はもう参照できない
    // println!("{}", word); // word は文字列スライス → 参照だからエラー
    
    // 一般に，任意の Vec 型に対してスライスが取れる
    let a = [1, 2, 3, 4, 5];
    let a_slice = &a[1..3]; // 要素は 2, 3

    for i in a_slice {
        println!("{}", i);
    }
}

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();
// 
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i
//         }
//     }
// 
//     s.len()
// }

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]
        }
    }

    &s[..]
}

