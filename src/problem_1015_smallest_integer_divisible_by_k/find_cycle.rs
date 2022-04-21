pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashSet;
use std::num::NonZeroU32;

impl Solution {
    pub fn smallest_repunit_div_by_k(k: i32) -> i32 {
        let k = NonZeroU32::new(k as _).unwrap();
        let mut x = 1 % k;
        let mut length = 1;
        let mut visited = HashSet::from([x]);

        while x != 0 {
            x = (x * 10 + 1) % k;

            if visited.insert(x) {
                length += 1;
            } else {
                return -1;
            }
        }

        length
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn smallest_repunit_div_by_k(k: i32) -> i32 {
        Self::smallest_repunit_div_by_k(k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
