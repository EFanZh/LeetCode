pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;

impl Solution {
    fn helper(x: i16, y: i16, cache: &mut HashMap<(i16, i16), f64>) -> f64 {
        if x <= 0 {
            if y <= 0 {
                0.5
            } else {
                1.0
            }
        } else if y <= 0 {
            0.0
        } else {
            let key = (x, y);

            if let Some(&result) = cache.get(&key) {
                result
            } else {
                let result = (Self::helper(x - 4, y, cache)
                    + Self::helper(x - 3, y - 1, cache)
                    + Self::helper(x - 2, y - 2, cache)
                    + Self::helper(x - 1, y - 3, cache))
                    * 0.25;

                cache.insert(key, result);

                result
            }
        }
    }

    pub fn soup_servings(n: i32) -> f64 {
        // Actually `n < 4451` is enough for passing the tests.

        if n < 16801 {
            let n = n as i16;
            let n = (n + 24) / 25;

            Self::helper(n, n, &mut HashMap::new())
        } else {
            1.0 // Close enough.
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn soup_servings(n: i32) -> f64 {
        Self::soup_servings(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
