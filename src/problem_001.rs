fn main() {
    let result: i32 = (1..1000).filter(|i| i % 3 == 0 || i % 5 == 0).sum();
    println!("{}", result);
}
