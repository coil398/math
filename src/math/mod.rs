pub fn sum(n: u64) -> u64 {
    let mut res = 0;
    for k in 1..=n {
        res += k;
    }
    res
}

pub fn triangle(n: u64) -> u64 {
    let mut res = 0;
    for k in 0..n {
        res += k + 1;
    }
    res
}
