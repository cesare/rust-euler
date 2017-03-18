mod prime;

use prime::PrimeSequence;

fn main() {
    fn find_minimum_factor(n: u64, primes: &mut PrimeSequence) -> u64 {
        primes.take_while(|&p| p <= n).find(|&p| n % p == 0).unwrap()
    }

    let mut the_number = 600851475143;
    let mut primes = PrimeSequence::new();

    let mut fs = Vec::<u64>::new();
    while the_number > 1 {
        let f = find_minimum_factor(the_number, &mut primes);
        fs.push(f);
        the_number = the_number / f;
        primes.reset();
    }

    println!("{:?}", fs);
}
