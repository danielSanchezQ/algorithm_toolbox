pub fn fibonacci(n: u32) -> u64 {
    if n <= 1 {
        return n as u64;
    }
    let (mut a, mut b) = (0, 1);
    for _ in 1..n {
        let next = a+b;
        a = b;
        b = next;
    }
    b
}

pub fn fibonacci_ends(n: u32) -> u8 {
    static ENDS: [u8; 60] = [
        0, 1, 1, 2, 3, 5, 8, 3, 1, 4, 5, 9, 4, 3, 7, 0, 7, 7, 4, 1, 5, 6, 1, 7, 8, 5, 3, 8, 1, 9,
        0, 9, 9, 8, 7, 5, 2, 7, 9, 6, 5, 1, 6, 7, 3, 0, 3, 3, 6, 9, 5, 4, 9, 3, 2, 5, 7, 2, 9, 1
    ];
    ENDS[n as usize % ENDS.len()]
}


#[cfg(test)]
mod test {
    use crate::fibonacci::{fibonacci, fibonacci_ends};

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
}