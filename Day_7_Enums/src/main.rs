// Match Case with Enu
#![allow(dead_code)]
#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
}
#[derive(Debug)]
enum Direction {
    Up(Point),
    Down(Point),
    Right(Point),
    Left(Point),
}
#[derive(Debug)]
enum Keys {
    UpKey(String),
    DownKey(String),
    RightKey(String),
    LeftKey(String),
}

impl Direction {
    fn match_direction(&self) -> Keys {
        match *self {
            Direction::Up(_) => Keys::UpKey(String::from("Pressed w")),
            Direction::Down(_) => Keys::DownKey(String::from("Pressed s")),
            Direction::Right(_) => Keys::RightKey(String::from("Pressed d")),
            Direction::Left(_) => Keys::LeftKey(String::from("Pressed a"))
            
        }
    }
}
fn main() {
    let u = Direction::Up(Point {x: 0, y: 1});
    let k = u.match_direction();
    println!("{:?}", k);


}
