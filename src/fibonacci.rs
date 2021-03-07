static FIBO_ENDS: [u8; 60] = [
    0, 1, 1, 2, 3, 5, 8, 3, 1, 4, 5, 9, 4, 3, 7, 0, 7, 7, 4, 1, 5, 6, 1, 7, 8, 5, 3, 8, 1, 9, 0, 9,
    9, 8, 7, 5, 2, 7, 9, 6, 5, 1, 6, 7, 3, 0, 3, 3, 6, 9, 5, 4, 9, 3, 2, 5, 7, 2, 9, 1,
];

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

pub fn fibonacci_ends(n: u64) -> u8 {
    FIBO_ENDS[n as usize % FIBO_ENDS.len()]
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

pub fn fibonacci_sum_ends(n: u64) -> u8 {
    let new_n = (n + 1) as usize % FIBO_ENDS.len();
    FIBO_ENDS
        .iter()
        .take(new_n)
        .fold(0u8, |accum, value| (accum + value) % 10)
}

pub fn fibonacci_sum_range_ends(m: u64, n: u64) -> u8 {
    if m == n {
        return fibonacci_ends(n);
    }
    let new_m = m as usize % FIBO_ENDS.len();
    let new_n = n as usize % FIBO_ENDS.len();
    if new_m < new_n {
        (new_m..new_n + 1)
            .map(|i| FIBO_ENDS[i])
            .fold(0u8, |acc, value| (acc + value) % 10)
    } else {
        let prev = (0..new_n + 1)
            .map(|i| FIBO_ENDS[i])
            .fold(0u8, |acc, value| (acc + value) % 10);
        let next = (new_m..FIBO_ENDS.len())
            .map(|i| FIBO_ENDS[i])
            .fold(0u8, |acc, value| (acc + value) % 10);
        (prev + next) % 10
    }
}

#[cfg(test)]
mod test {
    use crate::fibonacci::{
        fibonacci, fibonacci_ends, fibonacci_modulus, fibonacci_sum_ends, fibonacci_sum_range_ends,
        pisano,
    };

    #[test]
    fn test_seq() {
        let result: Vec<u64> = (0..6).map(fibonacci).collect();
        assert_eq!(result, vec![0, 1, 1, 2, 3, 5]);
    }

    #[test]
    fn test_ends() {
        for i in 0..50 {
            assert_eq!(fibonacci_ends(i) as u64, fibonacci(i as u32) % 10);
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

    #[test]
    fn fibo_sum_ends_example() {
        assert_eq!(fibonacci_sum_ends(3), 4);
    }

    #[test]
    fn fibo_sum_ends_example2() {
        assert_eq!(fibonacci_sum_ends(100), 5);
    }

    #[test]
    fn fibo_sum_ends_big() {
        println!("{}", fibonacci_sum_ends(832564823476));
    }

    #[test]
    fn fibo_sum_range_ends() {
        assert_eq!(fibonacci_sum_range_ends(3, 7), 1);
        assert_eq!(fibonacci_sum_range_ends(10, 200), 2);
    }

    #[test]
    fn fibo_sum_range_ends_flipped_modulus() {
        assert_eq!(fibonacci_sum_range_ends(5618252, 6583591534156), 6);
    }
}
