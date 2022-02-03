use std::mem;
fn main() {
    /* Types */
    // i8 u8 i32 u32 i64 u64 isize usize f32 f64
    let x = 13; // Variables are immutable by default

    let mut y = 34; // To make them mutable add keyword "mut"
    println!("Before: {0}, {1}", x, y);

    y = y * 2;

    println!("After: {0}, {1}", x, y);

    // Tuples
    let t: (char, f32, i32) = ('k', 1.2, 8);

    let (_, _, z) = t;

    println!("{:?}", t);
    println!("{}", z);

    // Arrays : Arrays should contain one type(datatype[int, char, float])
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{} {}", a.len(), mem::size_of_val(&a));

    // Strings
    let s = "String"; // This is an array of chars not a complete string;
    let ss = String::from("String!");
    println!("{:?} and {} are array of chars while string is a {}", s, s.to_string(), ss);
    // String Slicing (arrays)
    let xs = &s[0..2];
    println!("{:?}", xs);
}
