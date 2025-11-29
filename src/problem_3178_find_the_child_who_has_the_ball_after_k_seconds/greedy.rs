pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::num::NonZero;

impl Solution {
    pub fn number_of_child(n: i32, k: i32) -> i32 {
        let n = n.cast_unsigned();
        let k = k.cast_unsigned();

        NonZero::new(2 * n - 2)
            .map_or(0, |cycle| {
                let k = k % cycle;

                if k < n { k } else { cycle.get() - k }
            })
            .cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn number_of_child(n: i32, k: i32) -> i32 {
        Self::number_of_child(n, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
