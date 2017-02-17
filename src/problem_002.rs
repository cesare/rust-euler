pub struct Fib {
    i: u64,
    previous: u64,
    current: u64,
}

impl Fib {
    pub fn new() -> Fib {
        Fib {
            i: 0,
            previous: 0,
            current: 1,
        }
    }
}

impl Iterator for Fib {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let next = self.previous + self.current;
        self.i += 1;
        self.previous = self.current;
        self.current = next;
        Some(next)
    }
}

fn main() {
    let fib = Fib::new();
    let result: u64 = fib.take_while(|n| *n < 4_000_000u64).filter(|n| n % 2 == 0).sum();
    println!("{}", result);
}
