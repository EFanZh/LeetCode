pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::num::NonZero;

impl Solution {
    fn gcd(mut x: NonZero<u32>, mut y: u32) -> NonZero<u32> {
        while let Some(z) = NonZero::<u32>::new(y) {
            (x, y) = (z, x.get() % z);
        }

        x
    }

    fn divides_gcd(r: u32, k: NonZero<u32>) -> NonZero<u32> {
        NonZero::new(k.get() / Self::gcd(k, r)).unwrap()
    }

    pub fn resilient_subarray(nums: Vec<i32>, k: i32) -> i32 {
        let nums = nums.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>();
        let k = NonZero::<u32>::new(k.cast_unsigned()).unwrap();
        let mut iter = nums.iter().copied();
        let mut remainder = iter.next().unwrap() % k;
        let mut length = 1;
        let mut result = 0;

        for num in iter {
            let r = num % k;

            if r == remainder {
                length += 1;
            } else {
                if length > result {
                    result = result.max(length - (length - 1) % Self::divides_gcd(remainder, k));
                }

                remainder = r;
                length = 1;
            }
        }

        if length > result {
            result = result.max(length - (length - 1) % Self::divides_gcd(remainder, k));
        }

        result.cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn resilient_subarray(nums: Vec<i32>, k: i32) -> i32 {
        Self::resilient_subarray(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
