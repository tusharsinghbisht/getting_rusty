fn main() {
    // by default variable in rust is mutable it means you can't change it to chagne it you have to make it mutable by using `mut` keyword
    /*
    let x = 5;
    println!("Value of x is {}", x);
    x = 6;
    println!("New value of x is {}", x);
    */

    // creating variable using `mut` keyword
    let mut y = 5;
    println!("Value of y is {}", y);
    y = 6;
    println!("New value of y is {}", y);



    // Shadowing
    let x = 5;
    println!("Value of x is {}", x);
    let x = x + 1; // the first variable is shadowed by the second, which means that the second variable is what the compiler will see when you use the name of the variable. In effect, the second variable overshadows the first, taking any uses of the variable name to itself until either it itself is shadowed or the scope ends.

    { // inner scope
        let x = x * 2;
        println!("New value of x in inner-scope is {}", x);
    }

    println!("New value of x is {}", x); // scope ends shadowing ends

}
