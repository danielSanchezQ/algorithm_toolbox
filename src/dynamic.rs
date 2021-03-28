pub fn minimum_coins_exchange(value: usize, coins: &[usize]) -> usize {
    let mut slots: Vec<usize> = vec![0];
    for i in 1..=value {
        let min_c = coins
            .iter()
            .filter_map(|c| {
                if i >= *c {
                    Some(slots[i - c] + 1)
                } else {
                    None
                }
            })
            .min()
            .unwrap_or(0);
        slots.push(min_c);
    }
    slots[value]
}

pub fn primitive_calculator(value: usize) -> Vec<usize> {
    let mut min_steps = vec![0usize; value + 1];
    let mut predecessor = vec![0usize; value + 1];
    for i in 2..=value {
        min_steps[i] = min_steps[i - 1] + 1;
        predecessor[i] = i - 1;
        for n in 2..=3 {
            if i % n == 0 {
                let i_div_n = i / n;
                if min_steps[i_div_n] < min_steps[i] {
                    min_steps[i] = min_steps[i_div_n] + 1;
                    predecessor[i] = i_div_n;
                }
            }
        }
    }
    let mut res = Vec::new();
    let mut i = value;
    while i != 0 {
        res.push(i);
        i = predecessor[i];
    }
    res.reverse();
    res
}

pub fn edit_distance(s1: &str, s2: &str) -> usize {
    let s1: Vec<_> = s1.chars().collect();
    let s2: Vec<_> = s2.chars().collect();
    let n = s1.len() + 1;
    let m = s2.len() + 1;
    let mut d: Vec<Vec<usize>> = vec![vec![0usize; m]; n];
    // empty strings
    d[0] = (0..m).collect();
    for (i, item) in d.iter_mut().enumerate() {
        item[0] = i;
    }
    for j in 1..m {
        for i in 1..n {
            let insertion = d[i][j - 1] + 1;
            let deletion = d[i - 1][j] + 1;
            let _match = d[i - 1][j - 1];
            let mismatch = d[i - 1][j - 1] + 1;
            if s1[i - 1] == s2[j - 1] {
                d[i][j] = *[insertion, deletion, _match].iter().min().unwrap();
            } else {
                d[i][j] = *[insertion, deletion, mismatch].iter().min().unwrap();
            }
        }
    }
    d[s1.len()][s2.len()]
}

#[cfg(test)]
mod test {
    use crate::dynamic::{edit_distance, minimum_coins_exchange, primitive_calculator};

    #[test]
    fn test_exchange_example() {
        assert_eq!(minimum_coins_exchange(34, &[1, 3, 4]), 9);
    }

    #[test]
    fn test_primitive_calculator_example() {
        let res = primitive_calculator(5);
        assert_eq!(res.as_slice(), [1, 2, 4, 5]);
    }

    #[test]
    fn test_edit_distance_example() {
        assert_eq!(edit_distance("ab", "ab"), 0);
    }

    #[test]
    fn test_edit_distance_example2() {
        assert_eq!(edit_distance("ports", "shorts"), 3);
    }

    #[test]
    fn test_edit_distance_example3() {
        assert_eq!(edit_distance("editing", "distance"), 5);
    }
}
