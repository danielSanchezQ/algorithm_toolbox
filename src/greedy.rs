use std::ops::Div;

pub fn coin_exchange(n: u64) -> u64 {
    let mut remained = n;
    let mut counter = 0;
    for c in &[10, 5, 1] {
        if remained >= *c {
            let coins = remained.div(c);
            remained = remained - coins * c;
            counter += coins;
        }
    }
    counter
}

pub fn knapsack(max_weigh: u64, items: &mut [(u64, u64)]) -> f64 {
    fn value_per_weigh((weigh, value): (u64, u64)) -> f64 {
        (value as f64).div(weigh as f64)
    }
    items.sort_by(|a, b| {
        value_per_weigh(a.clone())
            .partial_cmp(&value_per_weigh(b.clone()))
            .unwrap()
    });
    let mut left_weigh = max_weigh;
    let mut total_value = 0f64;
    let mut idx = 0;
    while left_weigh > 0 && idx < items.len() {
        let (value, weigh) = items[idx];
        if weigh <= left_weigh {
            total_value += value as f64;
            left_weigh -= weigh;
        } else {
            total_value += (value as f64).div(weigh as f64) * left_weigh as f64;
            left_weigh = 0;
        }
        idx += 1;
    }
    total_value
}

pub fn gas_stations(distance: u64, gas_distance: u64, gas_stations: &mut Vec<u64>) -> Option<u64> {
    if gas_stations.is_empty() {
        return if gas_distance >= distance {
            Some(0)
        } else {
            None
        };
    }
    gas_stations.push(distance);
    let mut counter = 0;
    let mut max_distance = gas_distance;
    for stations in gas_stations.windows(2) {
        let (p, n) = (stations[0], stations[1]);
        if p <= max_distance && max_distance < n {
            counter += 1;
            max_distance = p + gas_distance;
        }
    }
    return if max_distance >= distance {
        Some(counter)
    } else {
        None
    };
}

pub fn advertisement_revenue(profit_per_click: &mut [i64], average_clicks: &mut [i64]) -> i64 {
    profit_per_click.sort_by(|a, b| b.cmp(a));
    average_clicks.sort_by(|a, b| b.cmp(a));
    profit_per_click
        .iter()
        .zip(average_clicks.iter())
        .map(|(a, b)| a * b)
        .sum()
}

pub fn minimum_segments_shared_points(segments: &mut [(u64, u64)]) -> Vec<u64> {
    let mut res = Vec::new();
    if !segments.is_empty() {
        segments.sort();
        let (_, mut b) = segments[0];
        for (next_a, next_b) in segments.iter().skip(1) {
            if b < *next_a {
                res.push(b);
                b = *next_b;
            } else if b > *next_b {
                b = *next_b;
            }
        }
        res.push(b);
    }
    res
}

pub fn max_prizes(n: u64) -> Vec<u64> {
    let mut res = Vec::new();
    let mut remind = n;
    let mut next = 1;
    while remind != 0 {
        if next > remind {
            res.last_mut().map(|v| *v += remind);
            remind = 0;
        } else {
            res.push(next);
            remind -= next;
            next += 1;
        }
    }
    res
}

pub fn max_number_from_digits(digits: &mut [u64]) {
    digits.sort_by(|init_a, init_b| {
        let a = init_a.to_string();
        let b = init_b.to_string();
        format!("{}{}", a, b).cmp(&format!("{}{}", b, a))
    });
    digits.reverse();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_greedy_coin_exchange_example() {
        assert_eq!(coin_exchange(2), 2);
        assert_eq!(coin_exchange(28), 6);
    }

    #[test]
    fn test_knapsack_example() {
        let max_weigh = 50;
        let mut items = [(60, 20), (100, 50), (120, 30)];
        assert_eq!(knapsack(max_weigh, &mut items), 180f64);

        let res = knapsack(10, &mut [(500, 30)]);
        assert_eq!(&res.to_string()[..6], "166.66");
    }

    #[test]
    fn gas_stations_example() {
        assert_eq!(
            gas_stations(950, 400, &mut vec![200, 375, 550, 750]),
            Some(2)
        );
        assert_eq!(gas_stations(10, 3, &mut vec![1, 2, 5, 9]), None);
        assert_eq!(
            gas_stations(500, 200, &mut vec![100, 200, 300, 400]),
            Some(2)
        );
    }

    #[test]
    fn advertisement_revenue_example() {
        let mut values = [1, 3, -5];
        let mut clicks = [-2, 4, 1];
        assert_eq!(advertisement_revenue(&mut values, &mut clicks), 23);
    }

    #[test]
    fn signatures_example() {
        let mut segments = [(1, 4), (1, 3), (2, 5), (3, 6)];
        assert_eq!(minimum_segments_shared_points(&mut segments), vec![3]);

        let mut segments = [(4, 7), (1, 3), (2, 5), (5, 6)];
        assert_eq!(minimum_segments_shared_points(&mut segments), vec![3, 6]);
    }

    #[test]
    fn max_rewards_examples() {
        assert_eq!(max_prizes(2), vec![2]);
        assert_eq!(max_prizes(6), vec![1, 2, 3]);
        assert_eq!(max_prizes(8), vec![1, 2, 5]);
    }

    #[test]
    fn max_number_from_digits_examples() {
        let mut v = [21, 2];
        max_number_from_digits(&mut v);
        assert_eq!(v, [2, 21]);

        let mut v = [9, 4, 6, 1, 9];
        max_number_from_digits(&mut v);
        assert_eq!(v, [9, 9, 6, 4, 1]);

        let mut v = [23, 39, 92];
        max_number_from_digits(&mut v);
        assert_eq!(v, [92, 39, 23]);

        let mut v = [23, 39, 92, 4];
        max_number_from_digits(&mut v);
        assert_eq!(v, [92, 4, 39, 23]);

        let mut v = [95959995, 95];
        max_number_from_digits(&mut v);
        assert_eq!(v, [95959995, 95]);
    }

    #[test]
    fn max_number_from_digits_ones_example() {
        let mut v = [111, 1, 1, 10, 10];
        max_number_from_digits(&mut v);
        assert_eq!(v, [1, 1, 111, 10, 10]);
    }

    #[test]
    fn max_number_from_digits_23_example() {
        let mut v = [23, 2, 23, 2];
        max_number_from_digits(&mut v);
        assert_eq!(v, [23, 23, 2, 2]);
    }

    #[test]
    fn max_number_from_digits_examples_2() {
        let mut v = [79, 797];
        max_number_from_digits(&mut v);
        assert_eq!(v, [79, 797]);
    }
}
