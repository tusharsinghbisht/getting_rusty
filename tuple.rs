fn main () {
    // Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.

    // Tuples have a fixed length: once declared, they cannot grow or shrink in size. 
    // Each position in the tuple has a type, and the types of the different values in the tuple don’t have to be the same.

    // let tup: (i32, f64, u8) = (500, 6.4, 1); // explicit type annotation


    let tup = (500, 6.4, 1);
    let (x, y, z) = tup; // destructuring a tuple

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);


    // Accessing tuple elements directly
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    println!("The value of five_hundred is: {}", five_hundred); 
    println!("The value of six_point_four is: {}", six_point_four);         
    println!("The value of one is: {}", one);
}