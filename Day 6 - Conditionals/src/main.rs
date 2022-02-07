fn main() {
    // If, else-if, else statements in Rust
    let n = 10;
    if n > 5 {
        println!("n is greater than 5")
    }
    else if n < 5 {
        println!("n is lesser than 5")
    }
    else {
        println!("n is equal to 5")
    }

    // Conditional bindings
    let c = false;

    let m = if c {
        36
    } else {
        76
    };

    println!("m: {}", m);

    // Loops (indefinite and finite)

    // loop {
    //     println!("infinite")
    // }
    let mut k = 0;
    loop {
        println!("Hello, world");
        k += 1;

        if k >= 10 {
            break;
        }
    }

    // While Loops
    let mut b = 10;
    while b != 0 {
        println!("b: {}", b);

        b = b - 1;
    }

    // for loops
    // let l = vec![1, 2, 3, 4];
    // for i in l {
    //     println!("l: {}", i);
    // }

    for i in 1..=3 {
        println!("i: {}", i)
    }

    
}
