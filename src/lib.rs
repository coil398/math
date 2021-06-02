pub mod math;

#[cfg(test)]
mod tests {
    use super::math;
    #[test]
    fn sum() {
        assert_eq!(55, math::sum(10));
    }

    #[test]
    fn triangle() {
        assert_eq!(15, math::triangle(5));
        assert_eq!(21, math::triangle(6));
    }
}
