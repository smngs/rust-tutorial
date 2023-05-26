// enum IPAddrKind {
//     V4, 
//     V6
// }
// 
// struct IpAddr {
//     kind: IPAddrKind,
//     address: String
// }

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String)
}

enum Message {
    Quit, 
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32)
}

impl Message {
    fn Call(&self) {
        println!("Call!");
    }
}


fn main() {
    // let home = IpAddr {
    //     kind: IPAddrKind::V4,
    //     address: String::from("127.0.0.1")
    // };

    // let loopback = IpAddr {
    //     kind: IPAddrKind::V6,
    //     address: String::from("::1")
    // };

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    println!("home: {:?}", home);
    println!("loopback: {:?}", loopback);

    let mes = Message::Quit;
    mes.Call();

    let some_number = Some(5);
    let some_string = Some("a string");

    // let absent_number = None; // Type annotations needed for Option<T>
    let absent_number: Option<i32> = None; // Type annotations needed for Option<T>
    
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // 
    // let sum = x + y; // エラー
    //
    // Option<T> 型はエラーが起きうる場合に利用 → match
    // でエラー発生時の処理を考慮しないといけない！！！

    match y {
        Some(i) => {
            let sum = x + i;
            println!("x + y = {}", sum);
        },
        None => {
            println!("y is None");
        }
    }

    // こっちでもいいね
    if let Some(i) = y {
        let sum = x + i;
        println!("x + y = {}", sum)
    }

    // unwrap を使うと，None のとき panic する
    let sum = x + y.unwrap();
    println!("x + y = {}", sum);
}
