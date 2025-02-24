fn main() {
    let mut x = 5;
    { // using a block to limit scope
        let y = &mut x;
        *y = 10;
    }
    {
        let z = &mut x;
        *z = 15;
    }
    println!("{}", x); //Prints 15
}

//Alternative using match
fn main() {
    let mut x = 5;
    match x {
        _ => {x = 10; x = 15}
    }
    println!("{}", x); //Prints 15
}