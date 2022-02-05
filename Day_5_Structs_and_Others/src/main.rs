#[derive(Debug)]
// Structs are custom-data types which allows us to stretch the data to our needs.

struct Object {
    width: u32,
    height: u32,
}

// fn area(obj: &Object) -> u32 {
//     obj.width * obj.height
// }

// Methods
impl Object {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn show(&self) {
        println!("{}x{} with area {}", self.width, self.height, self.area());
    }
}

// Related Functions
impl Object {
   
    fn new(width: u32, height: u32) -> Object {
        Object { width, height }
    }
}
fn main() {
    let o = Object {
        width: 40,
        height: 70,
    };

    let obj = Object::new(58, 10);
    // println!("{}x{} with area {}", o.width, o.height, o.area()/*area(&o)*/);
    // println!("{}x{} with area {}", obj.width, obj.height, obj.area());

    o.show();
    obj.show();
    println!("{:#?}", o);
    println!("{:#?}", obj);
}
