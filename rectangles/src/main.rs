#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

// fn area(rec: Rectangle) -> u32 {
//     rec.width * rec.height
// }

// Rectangle のメソッドとして定義する
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size, height: size
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rec2: &Rectangle) -> bool {
        (self.width > rec2.width) && (self.height > rec2.height)
    }
}

fn main() {
    let rec1 = Rectangle{
        width: 30,
        height: 50
    };

    println!("{:?}", rec1);
    // let rec1_area = area(rec1);
    let rec1_area = rec1.area();
    println!("rec1_area is {}", rec1_area);

    let rec2 = Rectangle::square(20);
    println!("{:?}", rec2);
    println!("rec1 < rec2: {}", rec1.can_hold(&rec2));
    println!("rec2_area is: {}", rec2.area());
}

