fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    // x = 6; // x is immutable variable
    // println!("The value of x is: {}", x);

    let mut y = 3;
    println!("The value of y is: {}", y);   // out: 3

    y = 10;
    println!("The value of y is: {}", y);   // out: 10

    // var shadowing
    let z = 5;

    let z = z + 1;  // new variable 'z'(1), shadowing origin 'z'.

    let z = z * 2;  // new variable 'z'(2), shadowing 'z'(1).

    println!("The value of z is: {}", z);   // print 'z'(2).
}