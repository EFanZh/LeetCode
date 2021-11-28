pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::VecDeque;

impl Solution {
    pub fn new21_game(n: i32, k: i32, max_pts: i32) -> f64 {
        let (n, k, max_pts) = (n as u16, k as u16, max_pts as u16);

        if k == 0 || max_pts <= n - k + 1 {
            1.0
        } else {
            let max_pts_f64 = f64::from(max_pts);
            let mut cache = VecDeque::with_capacity(max_pts.into());

            cache.push_back(1.0);

            let mut sum = 1.0;
            let mut result = 0.0;

            for i in 1..=n {
                let value = sum / max_pts_f64;

                if i >= max_pts {
                    sum -= cache.pop_front().unwrap();
                }

                if i < k {
                    cache.push_back(value);
                    sum += value;
                } else {
                    result += value;
                }
            }

            result
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn new21_game(n: i32, k: i32, max_pts: i32) -> f64 {
        Self::new21_game(n, k, max_pts)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
