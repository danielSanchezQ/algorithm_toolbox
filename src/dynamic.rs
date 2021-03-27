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

pub fn primitive_calculator(
    value: usize,
    operations: &mut [impl FnMut(usize) -> usize],
) -> (usize, Vec<usize>) {
    let mut res = vec![1];
    let mut init_i = 0;
    let mut counter = 0;
    'outer: loop {
        let next_init_i = res.len();

        for i in init_i..res.len() {
            for op in operations.iter_mut() {
                let op_res = op(res[i]);
                if op_res <= value {
                    res.push(op_res);
                }
                if op_res == value {
                    counter += 1;
                    break 'outer;
                }
            }
        }
        counter += 1;
        init_i = next_init_i;
    }
    (counter, res)
}

#[cfg(test)]
mod test {
    use crate::dynamic::{minimum_coins_exchange, primitive_calculator};

    #[test]
    fn test_exchange_example() {
        assert_eq!(minimum_coins_exchange(34, &[1, 3, 4]), 9);
    }

    #[test]
    fn test_seq() {
        let mut ops = [|v| v + 1, |v| v * 2, |v| v * 3];
        let res = primitive_calculator(10, &mut ops);

        println!("{:?}", res);
    }
}
