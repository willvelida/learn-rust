pub fn triple(n: i32) -> i32 {
    n * 3
}

pub fn abs_diff(a: i32, b: i32) -> i32 {
    if a > b { a - b } else { b - a}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn triples_a_number() {
        assert_eq!(triple(4), 12);
    }

    #[test]
    fn checks_the_difference() {
        assert!(abs_diff(3, 10)  > 0);
        assert_eq!(abs_diff(3, 10), 7);
        assert_ne!(abs_diff(3, 10), 0);
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn panics_on_bad_index() {
        let v = vec![1, 2, 3];
        let _ = v[99];
    }

    #[test]
    fn parses_a_number() -> Result<(), std::num::ParseIntError> {
        let n: i32 = "42".parse()?;
        assert_eq!(n, 42);
        Ok(())
    }
}