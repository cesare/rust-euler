mod prime;
use prime::PrimeSequence;

fn main() {
    let ps = PrimeSequence::new();
    let result = ps.take_while(|&p| p < 2_000_000u64).fold(0, |a, n| a + n);
    println!("{:?}", result);
}
