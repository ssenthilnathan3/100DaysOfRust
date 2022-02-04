#[allow(unused_variables)]

fn main() {
    /* Ownership means "Owning😋", But that is the literal meaning because when 
    a value is assigned to a variable, it owns the value. Lets say when we assign to
    a another variable. The "Ownership" of the first variable is lost. Lets see an
    example */

    let s = String::from("String!");
    let y = s;
    // println!("{}", s); // This shows an error because the ownership of var s is lost;

    // To avoid this kind of things, we can do this
    let ss = String::from("String!");
    let z = &ss;
    println!("{}", ss); // In this block of code, the variable's pointer is referenced instead of the variable value itself
}
