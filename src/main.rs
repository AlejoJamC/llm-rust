fn main() {
    let sum = (1..13)
        .filter(|n| n % 2 == 0)
        .fold(0, |tally, n| tally + n);

    print!("Sum = {}", sum);
}