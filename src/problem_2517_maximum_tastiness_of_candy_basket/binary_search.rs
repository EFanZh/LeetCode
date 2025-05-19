pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::iter::Copied;
use std::slice::Iter;

impl Solution {
    fn check(first_price: u32, mut price_iter: Copied<Iter<u32>>, k: u32, candidate: u32) -> bool {
        let mut prev = first_price;

        (1..k).all(|_| {
            let target = prev + candidate;

            price_iter.find(|&value| value >= target).is_some_and(|next| {
                prev = next;

                true
            })
        })
    }

    pub fn maximum_tastiness(price: Vec<i32>, k: i32) -> i32 {
        let mut price = price.into_iter().map(|x| x as u32).collect::<Vec<_>>();
        let k = k as u32;

        price.sort_unstable();

        let mut iter = price.iter().copied();
        let first_price = iter.next().unwrap();
        let mut left = 0;
        let mut right = (price.last().unwrap() - first_price) / (k - 1) + 1;

        while left < right {
            let middle = u32::midpoint(left, right);

            if Self::check(first_price, iter.clone(), k, middle) {
                left = middle + 1;
            } else {
                right = middle;
            }
        }

        (left - 1) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximum_tastiness(price: Vec<i32>, k: i32) -> i32 {
        Self::maximum_tastiness(price, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
