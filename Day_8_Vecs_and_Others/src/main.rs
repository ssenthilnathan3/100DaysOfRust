// Vector is simply an array. It has dynamically-allocated memory means that
// the size of the vector can be changed.It can be 1, 2, 3-dims. By default it is 1-dim, 
// in rust vectors can be created in two ways: 
use std::collections::HashMap;
fn main() {
    // first method,
    let a = vec![1, 2, 3];
    println!("{:?}", a);

    // second method
    let mut v = Vec::new();
    v.push(5);
    v.push(9);

    println!("{:?} {} {}", &v, v.len(), v.capacity());
    v.push(1);
    v.push(0);
    v.push(8);
    println!("See! the capacity changes ;{:?} len: {} Capacity: {}", &v, v.len(), v.capacity());
    // for i in &v {
    //     println!("{}", i);
    // }

    // HashMaps
    let mut hm = HashMap::new();
    hm.insert(String::from("Senthil"), 18);


}
