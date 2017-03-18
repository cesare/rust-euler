fn gcd(a: u32, b: u32) -> u32 {
    if a < b {
        return gcd(b, a);
    }

    let r = a % b;
    if r == 0 { b } else { gcd(b, r) }
}

fn reverse_seq(to: u32) -> Vec<u32> {
    let range = 2..to + 1;
    let mut ns = range.collect::<Vec<u32>>();
    ns.reverse();
    ns
}

fn filter(ns: &[u32], n: u32) -> Vec<u32> {
    let mut results = Vec::new();
    for x in ns {
        let r = x / gcd(*x, n);
        if r > 1 {
            results.push(r);
        }
    }
    results
}

fn frec(numbers: &mut [u32]) -> Vec<u32> {
    let mut results: Vec<u32> = Vec::new();
    if let Some((n, ns)) = numbers.split_first_mut() {
        let mut xs = filter(ns, *n);
        let ys = frec(&mut xs);
        results.push(*n);
        results.extend_from_slice(&ys);
    }
    results
}

fn main() {
    let mut init = reverse_seq(20);
    let results = frec(&mut init);
    let product = results.iter().fold(1, (|a, b| a * b));
    print!("{:?}", product);
}
