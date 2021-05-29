pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;

impl Solution {
    fn helper(mut n: u32, cache: &mut HashMap<u32, u32>) -> u32 {
        let base = n.trailing_zeros();

        n >>= n.trailing_zeros();

        if n < 4 {
            base + n - 1
        } else if let Some(&value) = cache.get(&n) {
            base + value
        } else {
            let value = 1 + Self::helper(n - 1, cache).min(Self::helper(n + 1, cache));

            cache.insert(n, value);

            base + value
        }
    }

    pub fn integer_replacement(n: i32) -> i32 {
        Self::helper(n as _, &mut HashMap::new()) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn integer_replacement(n: i32) -> i32 {
        Self::integer_replacement(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
