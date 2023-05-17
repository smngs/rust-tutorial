use std::io;

#[allow(unused_variables)]
fn main() {
    const MAX_POINTS: i32 = 100_000;
    println!("MAX_POINTS is {}", MAX_POINTS);

    let mut x = 5;
    println!("The value of x is {}.", x);
    x = x + 1;
    println!("The value of x is {}.", x);

    {
        let x = x + 2;
        println!("The value of x is {} under the scope.", x);
    }

    println!("The value of x is {}.", x);

    // 型注釈が必要
    let guess: u32 = "42".parse().expect("Not a number!");


    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    // 加
    let sum = 5 + 10;

    // 減
    let difference = 95.5 - 4.3;

    // 乗
    let product = 4 * 30;

    // 除
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3;
    
    // 余
    let reminder = 43 % 5;

    let t: bool = true;
    let f: bool = false;

    let c = 'z';
    let z = 'ℤ'; // Unicode が扱える
    let heart_eyed_cat = '😻';

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup2 = (500, 6.4, 1); // 型注釈は必須ではない
    let (x, y, z) = tup; // Unpack
    println!("The value of y is {}.", y);

    println!("tup.0 = {}", tup.0);
    println!("tup.1 = {}", tup.1);
    println!("tup.2 = {}", tup.2);

    // 配列（スタックにデータメモリを確保）
    // Vec 型みたいに動的に配列を確保することはできない

    let months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun"];
    let a = [1, 2, 3, 4, 5];
    let a2: [i32; 5] = [1, 2, 3, 4, 5]; 

    println!("a[3] = {}", a[3]);

    array_example()
}

fn array_example() {
    let a = [1, 2, 3, 4, 5];
    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line"); // ここで Option<F> を予期してるのかな？？？

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");
    
    let element = a[index];

    println!(
        "The value of the element at index {} is {}",
        index, element
    )
    // 配列外参照が発生すると，以下のような実行時エラーが発生
    //
    // thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 5', src/main.rs:86:19
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
}
