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
            let mut sum = 1.0;
            let mut result = 0.0;

            if max_pts < n {
                let mut cache = VecDeque::with_capacity(max_pts.into());

                cache.push_back(1.0);

                let (low, high, max_pts_is_low) = if max_pts < k {
                    (max_pts, k, true)
                } else {
                    (k, max_pts, false)
                };

                for _ in 1..low {
                    let value = sum / max_pts_f64;

                    cache.push_back(value);
                    sum += value;
                }

                if max_pts_is_low {
                    for _ in max_pts..k {
                        let value = sum / max_pts_f64;

                        sum -= cache.pop_front().unwrap();
                        cache.push_back(value);
                        sum += value;
                    }
                } else {
                    for _ in k..max_pts {
                        result += sum / max_pts_f64;
                    }
                }

                for _ in high..=n {
                    result += sum / max_pts_f64;
                    sum -= cache.pop_front().unwrap();
                }
            } else {
                for _ in 1..k {
                    sum += sum / max_pts_f64;
                }

                result += (sum / max_pts_f64) * f64::from(n - k + 1);
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
