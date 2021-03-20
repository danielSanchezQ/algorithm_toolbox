use std::cmp::Ordering;
use std::fmt::Debug;

pub fn quick_sorted<T>(values: &[T]) -> Vec<T>
where
    T: Copy + Ord,
{
    if values.is_empty() {
        Vec::new()
    } else {
        let v0 = &values[0];
        let (lowers, equals, highers) =
            values
                .iter()
                .fold((vec![], vec![], vec![]), |(mut l, mut e, mut h), v| {
                    match v.cmp(v0) {
                        Ordering::Less => {
                            l.push(*v);
                        }
                        Ordering::Equal => {
                            e.push(*v);
                        }
                        Ordering::Greater => {
                            h.push(*v);
                        }
                    };
                    (l, e, h)
                });
        quick_sorted(&lowers)
            .into_iter()
            .chain(equals.into_iter())
            .chain(quick_sorted(&highers).into_iter())
            .collect()
    }
}

fn merge<T>(s1: &[T], s2: &[T]) -> Vec<T>
where
    T: Copy + Ord,
{
    let mut it1 = s1.into_iter();
    let mut it2 = s2.into_iter();
    let mut res = Vec::new();
    let mut next1 = it1.next();
    let mut next2 = it2.next();
    'merge_loop: loop {
        match (next1, next2) {
            (Some(v1), Some(v2)) => match v1.cmp(v2) {
                Ordering::Less => {
                    res.push(*v1);
                    next1 = it1.next();
                }
                Ordering::Equal => {
                    res.push(*v1);
                    res.push(*v2);
                    next1 = it1.next();
                    next2 = it2.next();
                }
                Ordering::Greater => {
                    res.push(*v2);
                    next2 = it2.next();
                }
            },
            (Some(v), None) => {
                res.push(*v);
                next1 = it1.next();
            }
            (None, Some(v)) => {
                res.push(*v);
                next2 = it2.next();
            }
            (None, None) => {
                break 'merge_loop;
            }
        }
    }
    res
}

pub fn merge_sort<T>(slice: &[T]) -> Vec<T>
where
    T: Copy + Ord,
{
    if slice.len() <= 1 {
        return slice.to_vec();
    }

    let pivot = slice.len() / 2;

    merge(&merge_sort(&slice[0..pivot]), &merge_sort(&slice[pivot..]))
}

fn merge_inversions<T>(s1: &[T], s2: &[T], accum: u64) -> (Vec<T>, u64)
where
    T: Copy + Ord,
{
    let mut it1 = s1.into_iter();
    let mut it2 = s2.into_iter();
    let mut res = Vec::new();
    let mut next1 = it1.next();
    let mut next2 = it2.next();
    let s1_len = s1.len() as u64;
    let mut i1: u64 = 0;
    let mut counter: u64 = 0;
    'merge_loop: loop {
        match (next1, next2) {
            (Some(v1), Some(v2)) => match v1.cmp(v2) {
                Ordering::Less | Ordering::Equal => {
                    res.push(*v1);
                    next1 = it1.next();
                    i1 += 1;
                }
                Ordering::Greater => {
                    res.push(*v2);
                    next2 = it2.next();
                    counter += s1_len - i1;
                }
            },
            (Some(v), None) => {
                res.push(*v);
                next1 = it1.next();
                i1 += 1;
            }
            (None, Some(v)) => {
                res.push(*v);
                next2 = it2.next();
            }
            (None, None) => {
                break 'merge_loop;
            }
        }
    }
    (res, counter + accum)
}

pub fn number_of_inversions<T>(slice: &[T]) -> (Vec<T>, u64)
where
    T: Copy + Ord,
{
    if slice.len() <= 1 {
        return (slice.to_vec(), 0);
    }

    let pivot = slice.len() / 2;
    let (lv, li) = number_of_inversions(&slice[0..pivot]);
    let (rv, ri) = number_of_inversions(&slice[pivot..]);
    merge_inversions(&lv, &rv, li + ri)
}

#[cfg(test)]
mod test {
    use crate::sort::{merge, merge_sort, number_of_inversions, quick_sorted};

    #[test]
    fn test_quicksort_example() {
        let s = [2, 3, 9, 2, 2];
        assert_eq!(quick_sorted(&s), vec![2, 2, 2, 3, 9]);
    }

    #[test]
    fn test_merge() {
        assert_eq!(merge(&[1, 2], &[3, 4]), vec![1, 2, 3, 4])
    }

    #[test]
    fn merge_sort_small() {
        let s = [2, 1, 3];
        assert_eq!(merge_sort(&s), vec![1, 2, 3]);
    }

    #[test]
    fn test_quicksort_mergesort() {
        let s = [2, 3, 9, 2, 2];
        assert_eq!(quick_sorted(&s), merge_sort(&s));
    }

    #[test]
    fn number_of_inversions_example() {
        let s = [2, 3, 9, 2, 9];
        assert_eq!(number_of_inversions(&s), (vec![2, 2, 3, 9, 9], 2));
    }

    #[test]
    fn number_of_inversions_example2() {
        let s = [9, 8, 7, 3, 2, 1];
        assert_eq!(number_of_inversions(&s), (vec![1, 2, 3, 7, 8, 9], 15));
    }
}
