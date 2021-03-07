use std::ops::Div;

pub fn gcd(a: u64, b: u64) -> u64 {
    let (mut a, mut b) = match (a, b) {
        (a, b) if a < b => (b, a),
        otherswise => otherswise,
    };
    while b != 0 {
        let tmp = b;
        b = a % b;
        a = tmp;
    }
    a
}

pub fn lcm(a: u64, b: u64) -> u64 {
    match (a, b) {
        (0, 0) => 0,
        (a, b) => a.div(gcd(a, b)) * b,
    }
}

#[cfg(test)]
mod test {
    use crate::nums::{gcd, lcm};

    #[test]
    fn test_gdc_example() {
        let (a, b) = (28851538, 1183019);
        let expected = 17657;
        assert_eq!(gcd(a, b), expected);
    }

    #[test]
    fn test_lcm_example() {
        let (a, b) = (761457, 614573);
        let expected = 467970912861;
        assert_eq!(lcm(a, b), expected);
    }
}
