/// Compute the value of `a` modulo `n`
///
/// This function is necessary because Rust's built-in modulo operator doesn't behave in a
/// mathematically correct fashion for negative values of `a`.
#[inline]
pub fn modulo(a: i32, n: u32) -> u32 {
    let result = if a == 0 {
        0
    } else if a > 0 {
        a % (n as i32)
    } else {
        let offset = ((-a) as u32 / n) + 1;
        let a_adjusted = a + (n * offset) as i32;
        a_adjusted % (n as i32)
    };
    result as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(0, 1, 0)]
    #[case(1, 1, 0)]
    #[case(-1, 1, 0)]
    #[case(0, 5, 0)]
    #[case(1, 5, 1)]
    #[case(5, 5, 0)]
    #[case(-1, 5, 4)]
    #[case(-5, 5, 0)]
    fn test_modulo(#[case] a: i32, #[case] n: u32, #[case] expected: u32) {
        assert_eq!(expected, modulo(a, n));
    }
}
