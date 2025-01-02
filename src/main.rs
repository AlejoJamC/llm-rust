fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}

fn main() {
    println!("Hello, world!");

    let values = [8, 7, 5, 3, 0];
    let mut sum = 0;

    // Slices in rust
    for n in &values[0..2]{
        sum = add(sum, *n);
    }

    for n in &values[2..]{  // Slice from 2 to end  
        sum = add(sum,  *n);
    }

    print!("Sum: {}", sum);
}
