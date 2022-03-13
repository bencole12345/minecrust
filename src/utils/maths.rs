use glm::fmod;

/// Compute the value of `a` modulo `n`
///
/// This function is necessary because Rust's built-in modulo operator doesn't behave in a
/// mathematically correct fashion for negative values of `a`.
#[inline(always)]
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

/// Compute the value of `a` modulo `n`
#[inline(always)]
pub fn modulo_fp(a: f32, n: f32) -> f32 {
    let result = fmod(a, n);
    if result < 0.0 {
        result + n
    } else {
        result
    }
}

#[inline(always)]
pub fn interpolate(a0: f32, a1: f32, w: f32) -> f32 {
    a0 + (a1 - a0) * w
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

    #[rstest]
    #[case(0.0, 1.0, 0.0)]
    #[case(0.5, 1.0, 0.5)]
    #[case(1.0, 1.0, 0.0)]
    #[case(-0.5, 1.0, 0.5)]
    #[case(-1.0, 1.0, 0.0)]
    fn test_modulo_fp(#[case] a: f32, #[case] n: f32, #[case] expected: f32) {
        let actual = modulo_fp(a, n);
        assert_eq!(expected, actual);
    }

    #[rstest]
    #[case(0.0, 1.0, 0.0, 0.0)]
    #[case(0.0, 1.0, 1.0, 1.0)]
    #[case(0.0, 1.0, 0.5, 0.5)]
    #[case(10.0, 6.0, 0.75, 7.0)]
    fn test_interpolate(#[case] a0: f32, #[case] a1: f32, #[case] w: f32, #[case] expected: f32) {
        let actual = interpolate(a0, a1, w);
        assert_eq!(expected, actual);
    }
}
