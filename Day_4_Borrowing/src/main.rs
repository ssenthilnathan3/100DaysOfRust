#[allow(unused_variables)]

fn main() {
    let stack_f64: f64 = 1.;
    let heap_f64: Box<f64> = Box::new(4.);

    stack_procedure(stack_f64);
    println!("In main stack {}", stack_f64);

    heap_procedure(&heap_f64);
    // println!("In main heap {}", heap_f64); The Ownership of the variable is changed in 10th line!
    println!("In main heap {}", heap_f64); // After the execution of the procedure, the ownership of the variable is restored.
}


fn stack_procedure(mut param: f64) {
    param += 0.9;
    println!("In stack_procedure with param {}", param);
}

fn heap_procedure(param: &Box<f64>) {
    println!("In heap_procedure with param {}", param);
}
