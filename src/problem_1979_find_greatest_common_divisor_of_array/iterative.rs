pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn min_max(nums: &[i32]) -> (i32, i32) {
        let mut iter = nums.iter().copied();
        let mut min = iter.next().unwrap();
        let mut max = min;

        while let Some(left) = iter.next() {
            if let Some(right) = iter.next() {
                let (new_min, new_max) = if right < left { (right, left) } else { (left, right) };

                min = min.min(new_min);
                max = max.max(new_max);
            } else {
                if left < min {
                    min = left;
                } else if left > max {
                    max = left;
                }

                break;
            }
        }

        (min, max)
    }

    fn gcd(x: i32, y: i32) -> i32 {
        let mut x = x as u32;
        let mut y = y as u32;

        while y != 0 {
            let z = x % y;

            x = y;
            y = z;
        }

        x as _
    }

    pub fn find_gcd(nums: Vec<i32>) -> i32 {
        let (min, max) = Self::min_max(&nums);

        Self::gcd(min, max)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_gcd(nums: Vec<i32>) -> i32 {
        Self::find_gcd(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
