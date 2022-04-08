pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn gcd(mut x: usize, mut y: usize) -> usize {
        while y != 0 {
            let next_y = x % y;

            x = y;
            y = next_y;
        }

        x
    }

    #[allow(clippy::ptr_arg)]
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let length = nums.len();
        let k = (k as usize) % length;
        let gcd = Self::gcd(length, k);

        for i in 0..gcd {
            let mut next = i;

            loop {
                next = if next < length - k {
                    next + k
                } else {
                    next.wrapping_add(k).wrapping_sub(length)
                };

                if next == i {
                    break;
                }

                nums.swap(i, next);
            }
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn rotate(nums: &mut Vec<i32>, k: i32) {
        Self::rotate(nums, k);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
