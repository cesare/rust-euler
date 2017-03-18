mod prime;

use prime::PrimeSequence;

fn main() {
    let target_n = 10001;
    let ps = PrimeSequence::new();
    let prime = ps.take(target_n).last().unwrap();
    println!("{:?}", prime);
}
