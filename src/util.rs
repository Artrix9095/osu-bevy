pub fn ar_to_ms(ar: f32) -> f32 {
    if ar < 5.0 {
        1200.0 + 600.0 * (5.0 - ar) / 5.0
    } else if ar == 5.0 {
        1200.0
    } else {
        1200.0 - 750.0 * (ar - 5.0) / 5.0
    }
}

pub fn od_to_ms(od: f32) -> (f32, f32, f32) {
    (80.0 - 6.0 * od, 140.0 - 8.0 * od, 200.0 - 10.0 * od)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ar_to_ms() {
        assert_eq!(ar_to_ms(5.0), 1200.0);
        assert_eq!(ar_to_ms(10.0), 450.0);
        assert_eq!(ar_to_ms(4.0), 1320.0);
    }
}
