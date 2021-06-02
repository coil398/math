pub fn sum(n: u64) -> u64 {
    let mut res = 0;
    for k in 1..=n {
        res += k;
    }
    res
}
