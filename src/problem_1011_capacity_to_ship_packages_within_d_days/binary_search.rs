pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn check(weights: &[i32], mut days: u32, capacity: u32) -> bool {
        let mut sum = 0;

        days -= 1;

        for &weight in weights {
            let weight = weight as u32;
            let candidate = sum + weight;

            if candidate <= capacity {
                sum = candidate;
            } else {
                if days == 0 {
                    return false;
                }

                days -= 1;
                sum = weight;
            }
        }

        true
    }

    pub fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {
        let weights = weights.as_slice();
        let days = days as u32;
        let mut max = 0;
        let mut total = 0;

        for &weight in weights {
            let weight = weight as u32;

            max = max.max(weight);
            total += weight;
        }

        let mut left = max;
        let mut right = total;

        while left < right {
            let middle = (left + right) / 2;

            if Self::check(weights, days, middle) {
                right = middle;
            } else {
                left = middle + 1;
            }
        }

        left as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {
        Self::ship_within_days(weights, days)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
