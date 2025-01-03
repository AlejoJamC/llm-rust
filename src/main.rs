fn main() {
    let sum = (1..13)   
        .filter(|n| n % 2 == 0)
        .sum::<i32>();

    print!("Sum = {}", sum);
}