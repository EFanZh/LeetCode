pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
    pub fn final_prices(prices: Vec<i32>) -> Vec<i32> {
        let mut result = prices;
        let mut stack = Vec::<i32>::new();

        for target in result.iter_mut().rev() {
            loop {
                if let Some(&top) = stack.last() {
                    match top.cmp(target) {
                        Ordering::Less => {
                            stack.push(*target);

                            *target -= top;

                            break;
                        }
                        Ordering::Equal => {
                            *target -= top;

                            break;
                        }
                        Ordering::Greater => {
                            stack.pop();
                        }
                    }
                } else {
                    stack.push(*target);

                    break;
                }
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn final_prices(prices: Vec<i32>) -> Vec<i32> {
        Self::final_prices(prices)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
