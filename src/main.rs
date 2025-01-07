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

    fn add(n1: i32, n2: i32) -> i32{
        let mut sum = n1;
        let (count, increment) = if n2 > 0 {(n2, 1)} else {(-n2, -1)};
        let mut handles = vec![];

        for _ in 0..count {
            handles.push(
                thread::spawn( || {
                    sum += increment;
                })
            );
            sum += increment;
        }

        for handle in handles{
            handle.join().unwrap();
        }

        sum
    }
}

fn main(){
    let acc = Accumulator::new(0);
    let sum = Accumulator::add(0, 10);
    println!("Sum: {}", sum);
}