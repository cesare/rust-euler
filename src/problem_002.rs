pub struct Fib {
    i: u32,
    previous: u32,
    current: u32,
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
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let next = self.previous + self.current;
        self.i += 1;
        self.previous = self.current;
        self.current = next;
        Some(next)
    }
}


pub struct EvenFib {
    i: u32,
    previous: u32,
    current: u32,
}

impl EvenFib {
    pub fn new() -> EvenFib {
        EvenFib {
            i: 0,
            previous: 2,
            current: 0,
        }
    }
}

impl Iterator for EvenFib {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let next = self.previous + self.current * 4;
        self.i += 1;
        self.previous = self.current;
        self.current = next;
        Some(next)
    }
}

fn main() {
    let fib = Fib::new();
    let result_fib: u32 = fib.take_while(|n| *n < 4_000_000u32).filter(|n| n % 2 == 0).sum();

    let evenfib = EvenFib::new();
    let result_evenfib: u32 = evenfib.take_while(|n| *n < 4_000_000u32).sum();

    println!("fib={}, evenfib={}", result_fib, result_evenfib);
}
