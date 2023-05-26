fn main() {
    // &str 型
    {
        let s: &str = "hello";
        println!("{}", s);
    }
    // println!("{}", s); //error
    
    // String 型
    // ヒープに保存される →動的確保可能
    let mut s = String::from("hello");
    // mutable じゃないとコケる
    s.push_str(", world!");
    println!("{}", s);


    // move
    let s1 = String::from("Hello");
    let s2 = s1; // 所有権が s2 に移る → s1 はもう使えない
    // String 型はヒープにブチ込まれるから
    println!("{}", s2);
    // println!("{}", s1);
    
    // String をハードコピーしたい場合は clone() すればよい
    let s3 = s2.clone();
    println!("{}", s3);

    // clone
    
    // str だと？？
    let s1 = "Hello";
    let s2 = s1;
    println!("{}", s2);
    println!("{}", s1); // エラーにはならないね
    // str 型や i32 型とかはスタックに格納される → Copy トレイトに適合
    
    let s = String::from("Hello");
    takes_ownership(s);
    // println!("{}", s); // これはコケる
    // takes_onwership 関数に所有権が移っているから，main() ではもうつかえない
    
    let x = 5;
    makes_copy(x);
    println!("{}", x); // これは使える（コピーされて makes_copy に渡される）
    
    // gives_ownership から String 型を受け取る
    let s1 = gives_ownership();
    let s2 = String::from("Hello");
    let s3 = takes_and_gives_back(s1);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("Hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
