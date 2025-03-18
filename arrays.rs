fn main() {
    // Another way to have a collection of multiple values is with an array. Unlike a tuple, every element of an array must have the same type. Unlike arrays in some other languages, arrays in Rust have a fixed length.

    let arr = [1, 2, 3, 4, 5];


    // Arrays are useful when you want your data allocated on the stack, the same as the other types we have seen so far, rather than the heap or when you want to ensure you always have a fixed number of elements.

    // Explicit type annotation
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    // You can also initialize an array to contain the same value for each element by specifying the initial value, followed by a semicolon, and then the length of the array in square brackets, as shown here:
    let b = [3; 5]; // [3, 3, 3, 3, 3]

    // accessing elemetns in an array
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];

    println!("The value of first is: {}", first);  
    println!("The value of second is: {}", second);
}