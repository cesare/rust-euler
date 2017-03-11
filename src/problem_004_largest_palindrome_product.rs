fn split_number(n: u32) -> Vec<u32> {
    if n < 10 {
        vec![n]
    } else {
        let mut ns = split_number(n / 10);
        ns.push(n % 10);
        ns
    }
}

fn is_palindrome(n: u32) -> bool {
    let mut ns = split_number(n);
    ns.reverse();
    ns.iter().fold(0, |a, n| a * 10 + n) == n
}

fn main() {
    let mut max_parindrome = 0;

    // = 999 down to 100
    for a in (100..1000).rev() {
        // = a down to 100
        let bs = (100..a + 1).rev();
        let v = bs.map(|b| a * b).take_while(|&n| n > max_parindrome).find(|&n| is_palindrome(n));
        if let Some(p) = v {
            max_parindrome = p;
        }
    }

    println!("{:?}", max_parindrome);
}
