#[derive(Debug)]
struct Accumulator{
    sum: i32,
}

impl Accumulator{
    fn new(init:i32 ) -> Accumulator{
        Accumulator{sum: init}
    }
}

fn main() {
    let acc: Accumulator = Accumulator::new(0);

    println!("acc = {:?}", acc);
}