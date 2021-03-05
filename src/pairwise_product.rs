use std::ops::Mul;

pub fn pairwise_product<N : Copy + Ord + Mul>(lst: &[N]) -> <N as Mul>::Output {
    assert!(lst.len() >= 2);
    let mut iter = lst.iter();
    let next_a = iter.next().unwrap();
    let next_b = iter.next().unwrap();
    let init= match (next_a, next_b) {
        (a, b)if a < b => (b, a),
        otherwise => otherwise,
    };
    let (maxa, maxb) = iter.fold(init, |(a, b), n| {
        if n > a {
            (n, a)
        } else if n > b {
            (a, n)
        } else {
            (a, b)
        }
    });
    *maxa * *maxb
}

#[cfg(test)]
mod test {
    use crate::pairwise_product::*;
    use std::hash::Hash;

    fn naive_pairwise_product<N : Copy + Ord + Mul + Hash>(lst: &[N]) -> <N as Mul>::Output {
        let mut sub_lst: Vec<N> = lst.iter().cloned().collect();
        sub_lst.sort();
        let mut iter = sub_lst.iter().rev().take(2);
        *iter.next().unwrap() * *iter.next().unwrap()
    }

    #[test]
    fn test_example() {
        let example = &(0..11).collect::<Vec<u8>>();
        assert_eq!(pairwise_product(&example), naive_pairwise_product(&example));
    }

    #[test]
    fn stress_test() {
        use rand::prelude::*;
        let mut rng = rand::thread_rng();

        for _ in 0..1000 {
            let example: Vec<u64> = (0..100).into_iter().map(|_| rng.next_u32() as u64 ).collect();
            assert_eq!(pairwise_product(&example), naive_pairwise_product(&example));
        }
    }
}