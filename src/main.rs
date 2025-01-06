#[derive(Debug)]
struct Accumulator{
    sum: i32,
}

impl Accumulator{
    fn new(sum:i32 ) -> Self{
        Self{sum}
    }

    fn get(self) -> i32{
        // access properties and methods of the struct
        self.sum
    }

    fn add(mut self, increment: i32) -> Self{
        Self {
            sum: self.sum + increment
        }
    }
}

fn main() {
    let acc: Accumulator = Accumulator::new(0);

    println!("acc = {:?}", acc);
}