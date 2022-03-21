pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
    fn iter_diff(mut prev: i32, nums: &[i32], mut f: impl FnMut(Ordering)) {
        let mut iter = nums.iter().copied();

        loop {
            if let Some(x) = iter.next() {
                f(prev.cmp(&x));
                prev = x;
            } else {
                break;
            }

            if let Some(x) = iter.next() {
                f(x.cmp(&prev));
                prev = x;
            } else {
                break;
            }
        }
    }

    pub fn max_turbulence_size(arr: Vec<i32>) -> i32 {
        arr.split_first().map_or(0, |(&first, rest)| {
            let mut result = 1;
            let mut prev = Ordering::Equal;
            let mut length = 1;

            Self::iter_diff(first, rest, |order| match order {
                Ordering::Less | Ordering::Greater => {
                    if order == prev {
                        length += 1;
                    } else {
                        length = 2;
                        prev = order;
                    }

                    result = result.max(length);
                }
                Ordering::Equal => prev = Ordering::Equal,
            });

            result
        })
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_turbulence_size(arr: Vec<i32>) -> i32 {
        Self::max_turbulence_size(arr)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
