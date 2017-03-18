use std::ops::Range;

fn sequence_upto(max: u32) -> Range<u32> {
    1u32..max + 1
}

fn main() {
    let max = 100;

    let sum_of_square = sequence_upto(max).map(|n| n * n).fold(0, |a, n| a + n);
    let square_of_sum = sequence_upto(max).fold(0, |a, n| a + n).pow(2);
    let result = square_of_sum - sum_of_square;
    print!("{:?}", result);
}
