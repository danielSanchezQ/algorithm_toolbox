use std::collections::HashSet;

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

pub fn common_items(sequences: &[&[usize]]) -> HashSet<usize> {
    let mut res: HashSet<usize> = HashSet::new();
    for s in sequences {
        res.extend(s.iter())
    }
    res
}

pub fn nub_sequence(seq: &[usize], items: &HashSet<usize>) -> Vec<usize> {
    seq.iter().filter(|i| items.contains(i)).cloned().collect()
}

pub fn longest_common_consecutive_subsequence(sequences: &[&[usize]]) -> usize {
    let min_len = sequences.iter().map(|seq| seq.len()).min().unwrap();
    for i in (2..min_len).rev() {
        let collides = sequences
            .iter()
            .map(|seq| HashSet::from(seq.windows(i).collect()))
            .reduce(|s1, s2| s1.intersection(&s2).cloned().collect::<HashSet<&[usize]>>())
            .unwrap();
        if collides.len() != 0 {
            return collides.len();
        }
    }
    0
}

pub fn longest_common_subsequence(sequences: &[&[usize]]) -> usize {
    let common = common_items(sequences);
    println!("{:?}", common);
    let consecutive: Vec<Vec<usize>> = sequences
        .iter()
        .map(|ss| nub_sequence(ss, &common))
        .collect();
    println!("{:?}", consecutive);
    let slices: Vec<&[usize]> = consecutive.iter().map(|e| e.as_slice()).collect();
    longest_common_consecutive_subsequence(&slices)
}

fn longest_common_subsequences_matrix(s1: &[usize], s2: &[usize]) -> Vec<Vec<usize>> {
    let n = s1.len() + 1;
    let m = s2.len() + 1;
    let mut d: Vec<Vec<usize>> = vec![vec![0usize; m]; n];
    for j in 1..m {
        for i in 1..n {
            let equals = d[i - 1][j - 1] + 1;
            let not_equals_1 = d[i - 1][j];
            let not_equals_2 = d[i][j - 1];
            if s1[i - 1] == s2[j - 1] {
                d[i][j] = equals;
            } else {
                d[i][j] = not_equals_1.max(not_equals_2);
            }
        }
    }
    d
}

pub fn dynamic_longest_common_subsequence(s1: &[usize], s2: &[usize]) -> usize {
    let d = longest_common_subsequences_matrix(s1, s2);
    d[s1.len()][s2.len()]
}

pub fn dynamic_longest_common_subsequence_3(s1: &[usize], s2: &[usize], s3: &[usize]) -> usize {
    let m = s1.len() + 1;
    let n = s2.len() + 1;
    let o = s3.len() + 1;
    if [m, n, o].iter().any(|v| *v == 1) {
        return 0;
    }
    let mut d: Vec<Vec<Vec<usize>>> = vec![vec![vec![0usize; o]; n]; m];
    for i in 1..m {
        for j in 1..n {
            for k in 1..o {
                let equals = d[i - 1][j - 1][k - 1] + 1;
                let remove_i = d[i - 1][j][k];
                let remove_j = d[i][j - 1][k];
                let remove_k = d[i][j][k - 1];
                if s1[i - 1] == s2[j - 1] && s2[j - 1] == s3[k - 1] {
                    d[i][j][k] = equals;
                } else {
                    d[i][j][k] = *[remove_i, remove_j, remove_k].iter().max().unwrap();
                }
            }
        }
    }
    d[s1.len()][s2.len()][s3.len()]
}

pub fn maximum_sum_in_capacity(weight: usize, values: &[usize]) -> usize {
    let mut m = vec![vec![0; values.len() + 1]; weight + 1];
    for item in 1..values.len() + 1 {
        for w in 1..weight + 1 {
            m[w][item] = m[w][item - 1];
            let v = values[item - 1];
            if v <= weight {
                let new_val = m[weight - w][item - 1] + v;
                m[w][item] = new_val.max(m[w][item - 1]);
            }
        }
    }
    println!("{:?}", m);
    m[weight][values.len()]
}

#[cfg(test)]
mod test {
    use crate::dynamic::{
        dynamic_longest_common_subsequence, dynamic_longest_common_subsequence_3, edit_distance,
        maximum_sum_in_capacity, minimum_coins_exchange, primitive_calculator,
    };

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

    #[test]
    fn longest_common_subsequence_size_2_example() {
        let s1 = [2, 7, 8, 3];
        let s2 = [5, 2, 8, 7];
        assert_eq!(dynamic_longest_common_subsequence(&s1, &s2), 2);
    }

    #[test]
    fn longest_common_subsequence_size_3_empty() {
        assert_eq!(dynamic_longest_common_subsequence_3(&[], &[], &[]), 0);
    }

    #[test]
    fn longest_common_subsequence_size_3_example() {
        assert_eq!(
            dynamic_longest_common_subsequence_3(&[1, 2, 3], &[2, 1, 3], &[1, 3, 5]),
            2
        );
    }

    #[test]
    fn maximum_sum_capacity_example() {
        assert_eq!(maximum_sum_in_capacity(10, &[1, 4, 8]), 9);
    }
}
