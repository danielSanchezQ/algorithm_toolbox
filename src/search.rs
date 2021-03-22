use std::cmp::Ordering;
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::{Div, RangeInclusive};

pub fn binary_search<T>(slice: &[T], element: &T) -> Option<usize>
where
    T: Ord,
{
    let mut min = 0i32;
    let mut max: i32 = (slice.len() - 1) as i32;
    while min <= max {
        let i = min + (max - min).div(2);
        match element.cmp(slice.get(i as usize).unwrap()) {
            Ordering::Less => {
                max = i - 1;
            }
            Ordering::Equal => {
                if let Some(v) = slice.get((i + 1) as usize) {
                    if v == element {
                        return Some((i + 1) as usize);
                    }
                }
                return Some(i as usize);
            }
            Ordering::Greater => {
                min = i + 1;
            }
        }
    }
    None
}

pub fn binary_search_all<T>(slice: &[T], to_find: &[T]) -> Vec<Option<usize>>
where
    T: Ord,
{
    to_find.iter().map(|e| binary_search(slice, e)).collect()
}

pub fn majority_element<T>(slice: &[T]) -> usize
where
    T: Eq + Hash + Clone,
{
    let accum: HashMap<T, u64> = HashMap::new();
    slice
        .iter()
        .fold(accum, |mut acc, value| {
            let entry = acc.entry(value.clone()).or_default();
            *entry += 1;
            acc
        })
        .into_iter()
        .filter(|(_, total)| *total > ((slice.len() / 2) as u64))
        .count()
}

/// From theorem:
/// https://d3c33hcgiwev3.cloudfront.net/imageAssetProxy.v1/ia-ZEETbQzSvmRBE23M0hw_748fb2abc394115c179a09b8b1ed1f6d_20200622_160953.jpg?expiry=1616544000000&hmac=W4J9KK2DkhHNDEi2nY3IVcJaCuTiSjbTTZKFVW3f46k
pub fn points_on_segments(segments: &[(i64, i64)], points: &[i64]) -> Vec<u64> {
    let (mut starts, mut ends) =
        segments
            .iter()
            .cloned()
            .fold((Vec::new(), Vec::new()), |(mut inits, mut ends), (i, e)| {
                inits.push(i);
                ends.push(e);
                (inits, ends)
            });
    starts.sort();
    ends.sort();
    println!("{:?} {:?}", starts, ends);
    points
        .iter()
        .map(|p| {
            let mut l = 0;
            for (i, s) in starts.iter().enumerate().rev() {
                if s <= p {
                    l = i + 1;
                    break;
                }
            }
            let mut h = 0;
            for (i, e) in ends.iter().enumerate().rev() {
                if e < p {
                    h = i + 1;
                    break;
                }
            }
            println!("{} {} {}", p, h, l);
            (l - h) as u64
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::RngCore;

    #[test]
    fn test_finds_example() {
        let slice = [1, 5, 8, 12, 13];
        assert_eq!(binary_search(&slice, &1), Some(0));
        let slice = [1, 5, 8, 12, 13, 14];
        assert_eq!(binary_search(&slice, &1), Some(0));
    }

    #[test]
    fn test_one_weird() {
        let s = [
            4, 5, 20, 44, 54, 72, 82, 89, 91, 94, 121, 135, 139, 156, 169, 176, 181, 181, 181, 186,
            202, 204, 209, 218, 222, 238,
        ];
        assert_eq!(binary_search(&s, &181), Some(18))
    }

    #[test]
    fn test_many() {
        for _ in 0..1000 {
            let mut rng = rand::thread_rng();
            let mut v: Vec<u8> = (0..25).map(|_| (rng.next_u32() % 256) as u8).collect();
            v.sort();
            let value: u8 = (rng.next_u32() % 256) as u8;
            println!("{:?} {}", v, value);
            assert_eq!(binary_search(&v, &value), v.binary_search(&value).ok());
        }
    }

    #[test]
    fn test_finds_all_example() {
        let slice = [1, 5, 8, 12, 13, 14];
        let other = [8, 1, 23, 1, 11];
        let result = [Some(2), Some(0), None, Some(0), None];
        assert_eq!(binary_search_all(&slice, &other).as_slice(), result);
    }

    #[test]
    fn majority_example() {
        let s = [2, 3, 9, 2, 2];
        assert_eq!(majority_element(&s), 1);

        let s = [1, 2, 3, 4];
        assert_eq!(majority_element(&s), 0);

        let s = [1, 3, 9, 2, 1];
        assert_eq!(majority_element(&s), 0);
    }

    #[test]
    fn points_on_segments_example() {
        let segments = [(0, 5), (-3, 2), (7, 10)];
        let points = [1, 6];
        let output = [2, 0];
        assert_eq!(&points_on_segments(&segments, &points), &output)
    }
}
