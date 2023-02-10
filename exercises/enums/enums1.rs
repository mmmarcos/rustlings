// enums1.rs
// No hints this time! ;)

#[derive(Debug)]
enum Message {
    Quit,
    Echo(String),
    Move { x: i32, y: i32 },
    ChangeColor(u8, u8, u8),
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo(String::from("echo")));
    println!("{:?}", Message::Move { x: 1, y: 1 });
    println!("{:?}", Message::ChangeColor(0, 255, 0));
}
