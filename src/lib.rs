pub mod math;

#[cfg(test)]
mod tests {
    use super::math;
    #[test]
    fn sum() {
        assert_eq!(55, math::sum(10));
    }
}
