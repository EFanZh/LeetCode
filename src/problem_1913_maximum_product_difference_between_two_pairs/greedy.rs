pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
    pub fn max_product_difference(nums: Vec<i32>) -> i32 {
        let (head, tail) = nums.split_at(4);
        let mut head = <[_; 4]>::try_from(head).ok().unwrap();

        head.sort_unstable();

        let [mut min_1, mut min_2, mut max_1, mut max_2] = head;

        for &num in tail {
            match num.cmp(&max_1) {
                Ordering::Less => {
                    if num < min_2 {
                        if num < min_1 {
                            min_2 = min_1;
                            min_1 = num;
                        } else {
                            min_2 = num;
                        }
                    }
                }
                Ordering::Equal => {}
                Ordering::Greater => {
                    if num > max_2 {
                        max_1 = max_2;
                        max_2 = num;
                    } else {
                        max_1 = num;
                    }
                }
            }
        }

        (max_1 * max_2) - (min_1 * min_2)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_product_difference(nums: Vec<i32>) -> i32 {
        Self::max_product_difference(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
