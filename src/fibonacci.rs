pub fn fibonacci(n: u32) -> u64 {
    if n <= 1 {
        return n as u64;
    }
    let (mut a, mut b) = (0, 1);
    for _ in 1..n {
        let next = a + b;
        a = b;
        b = next;
    }
    b
}

pub fn fibonacci_ends(n: u32) -> u8 {
    static ENDS: [u8; 60] = [
        0, 1, 1, 2, 3, 5, 8, 3, 1, 4, 5, 9, 4, 3, 7, 0, 7, 7, 4, 1, 5, 6, 1, 7, 8, 5, 3, 8, 1, 9,
        0, 9, 9, 8, 7, 5, 2, 7, 9, 6, 5, 1, 6, 7, 3, 0, 3, 3, 6, 9, 5, 4, 9, 3, 2, 5, 7, 2, 9, 1,
    ];
    ENDS[n as usize % ENDS.len()]
}

pub fn pisano(m: u64) -> u64 {
    let (mut previous, mut current): (u64, u64) = (0, 1);
    let mut counter = 0;
    loop {
        counter += 1;
        let tmp = previous;
        previous = current;
        current = (tmp + current) % m;
        if matches!((previous, current), (0, 1)) {
            return counter;
        }
    }
}

pub fn fibonacci_modulus(n: u64, m: u64) -> u64 {
    let pisano = pisano(m);
    let new_n = n % pisano;
    if new_n <= 1 {
        return new_n % m;
    }
    let (mut a, mut b) = (0, 1);
    for _ in 1..new_n {
        let next = (a + b) % m;
        a = b;
        b = next;
    }
    b % m
}

#[cfg(test)]
mod test {
    use crate::fibonacci::{fibonacci, fibonacci_ends, fibonacci_modulus, pisano};

    #[test]
    fn test_seq() {
        let result: Vec<u64> = (0..6).map(fibonacci).collect();
        assert_eq!(result, vec![0, 1, 1, 2, 3, 5]);
    }

    #[test]
    fn test_ends() {
        for i in 0..50 {
            assert_eq!(fibonacci_ends(i) as u64, fibonacci(i) % 10);
        }
    }

    #[test]
    fn pisano_example() {
        let expected = 20u64;
        assert_eq!(pisano(5), expected);
    }

    #[test]
    fn fibo_mod() {
        let (n, m) = (1548276540, 235);
        let expected = 185;
        assert_eq!(fibonacci_modulus(n, m), expected);
    }

    #[test]
    fn fibo_mod_1000_100() {
        let (n, m) = (1000, 100);
        let expected = 75;
        assert_eq!(fibonacci_modulus(n, m), expected);
    }

    #[test]
    fn fibo_mod_9999999999999_2() {
        let (n, m) = (9999999999999, 2);
        let expected = 0;
        assert_eq!(fibonacci_modulus(n, m), expected);
    }
}
