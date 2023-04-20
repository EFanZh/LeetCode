pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::VecDeque;
use std::convert::TryInto;

struct Item {
    x_minus_y: i32,
    x_plus_k: i32,
}

impl Solution {
    pub fn find_max_value_of_equation(points: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut queue = VecDeque::<Item>::new();
        let mut result = i32::MIN;

        for point in points {
            let [x, y]: [_; 2] = point.try_into().ok().unwrap();
            let x_plus_y = x + y;

            while let Some(front) = queue.front() {
                if x <= front.x_plus_k {
                    result = result.max(x_plus_y - front.x_minus_y);

                    break;
                }

                queue.pop_front();
            }

            let item = Item {
                x_minus_y: x - y,
                x_plus_k: x + k,
            };

            while let Some(back) = queue.back() {
                if item.x_minus_y <= back.x_minus_y {
                    queue.pop_back();
                } else {
                    break;
                }
            }

            queue.push_back(item);
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_max_value_of_equation(points: Vec<Vec<i32>>, k: i32) -> i32 {
        Self::find_max_value_of_equation(points, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
