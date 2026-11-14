pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::num::NonZero;

impl Solution {
    pub fn find_the_winner(n: i32, k: i32) -> i32 {
        let n = n.cast_unsigned();
        let k = k.cast_unsigned();

        let Some(k_minus_1) = NonZero::new(k - 1) else {
            return n.cast_signed();
        };

        let mut result = 0;
        let mut count = 2;

        while count <= n {
            let max_non_wrap_steps = (count - result - 2) / k_minus_1;
            let skip = max_non_wrap_steps.min(n - count);

            result += skip * k;
            count += skip;
            result = (result + k) % count;
            count += 1;
        }

        (result + 1).cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_the_winner(n: i32, k: i32) -> i32 {
        Self::find_the_winner(n, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
