#![allow(unused_variables)]
#![allow(dead_code)]

use std::fs::File;
fn main() {
    let s = Some('c');
    // If-Lets are if conditions in a structured way in rust
    if let Some(i) = s {
        println!("{}", i);
    } else {
        {}
    }

    // Result Enums
    enum Result<T, E> {
        Ok(T),
        Err(E)
    }

    // Example for Result Enum
    let f = File::open("file.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("There was a problem in opening the file {:?}", error);
        }
    };
}
