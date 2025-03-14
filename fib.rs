fn main() {
    for i in 0..=64 {
        println!("fibonacci number {i:2.}: {}", fibr(i));
    }
}

/// Computes the n-th fibonacci number recursively.
#[allow(dead_code)]
fn fibr(n: u32) -> u64 {
    match n {
        0 | 1 => 1,
        _ => fibr(n - 1) + fibr(n - 2),
    }
}

/// Computes the n-th fibonacci number iteratively.
#[allow(dead_code)]
fn fibi(n: u32) -> u64 {
    if n == 0 || n == 1 {
        return 1;
    }
    let (mut prev, mut curr) = (1, 1);
    for _ in 2..=n {
        let next = prev + curr;
        prev = curr;
        curr = next;
    }
    curr
}
