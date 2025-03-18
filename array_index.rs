use std::io;

fn main () {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];

    // using index to access element in an array
    // let element = arr[1];

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    // shadowing
    let index: usize = index.trim().parse().expect("Index entered was not a number");

    let element = arr[index];

    println!("The value at index: {} is = {}", index, element);
}


