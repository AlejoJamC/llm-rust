struct Accumulator{
    sum: i32,
}

fn main() {
    let mut acc = Accumulator{sum: 0};
    acc.sum = 10;
    println!("Sum: {}", acc.sum);
}