use super::CustomFunction;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
    pub fn find_solution(customfunction: &CustomFunction, z: i32) -> Vec<Vec<i32>> {
        let mut x = 1;
        let mut y = 1000;
        let mut result = Vec::new();

        loop {
            match customfunction.f(x, y).cmp(&z) {
                Ordering::Less => x += 1,
                Ordering::Equal => {
                    result.push(vec![x, y]);

                    x += 1;
                    y -= 1;
                }
                Ordering::Greater => y -= 1,
            }

            if x == 1001 || y == 0 {
                break;
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_solution(customfunction: &CustomFunction, z: i32) -> Vec<Vec<i32>> {
        Self::find_solution(customfunction, z)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
