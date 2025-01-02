fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}


fn main() {
    println!("Hello, world!");

    let values = [8, 7, 5, 3, 0];
    let mut sum = 0;

    for n in values {
        sum = add(sum,  n);
    }

    print!("Sum: {}", sum);
}
