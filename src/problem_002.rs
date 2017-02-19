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

fn fibs() -> Vec<u32> {
    let fib = Fib::new();
    let fibs = fib.take_while(|n| *n < 4_000_000u32).filter(|n| n % 2 == 0);
    fibs.collect()
}

fn even_fibs() -> Vec<u32> {
    let fib = EvenFib::new();
    let fibs = fib.take_while(|n| *n < 4_000_000u32);
    fibs.collect()
}

fn main() {
    let fs1 = fibs();
    let fs1sum: u32 = fs1.iter().sum();
    println!("naive fibs: {:?} -> {}", fs1, fs1sum);

    let fs2 = even_fibs();
    let fs2sum: u32 = fs2.iter().sum();
    println!(" even fibs: {:?} -> {}", fs2, fs2sum);
}
