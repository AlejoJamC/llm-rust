#[derive(Debug)]
struct Accumulator{
    sum: i32,
}

impl Accumulator{
    fn new(sum:i32 ) -> Self{
        Self{sum}
    }
}

fn main() {
    let acc: Accumulator = Accumulator::new(0);

    println!("acc = {:?}", acc);
}