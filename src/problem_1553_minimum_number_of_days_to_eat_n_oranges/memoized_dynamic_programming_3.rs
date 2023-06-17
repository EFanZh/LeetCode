pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;

impl Solution {
    fn helper_2(cache: &mut HashMap<u32, u32>, n: u32) -> u32 {
        if let Some(&result) = cache.get(&n) {
            result
        } else {
            let result = (Self::helper(cache, n / 2) + n % 2).min(Self::helper(cache, n / 3) + n % 3) + 1;

            cache.insert(n, result);

            result
        }
    }

    #[inline]
    fn helper(cache: &mut HashMap<u32, u32>, n: u32) -> u32 {
        if n < 3 {
            n
        } else {
            Self::helper_2(cache, n)
        }
    }

    pub fn min_days(n: i32) -> i32 {
        Self::helper(&mut HashMap::new(), n as _) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_days(n: i32) -> i32 {
        Self::min_days(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
