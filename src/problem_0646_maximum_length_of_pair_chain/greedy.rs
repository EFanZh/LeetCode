pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::convert::TryInto;

impl Solution {
    pub fn find_longest_chain(pairs: Vec<Vec<i32>>) -> i32 {
        let mut pairs = pairs
            .iter()
            .map(|pair| {
                let [left, right]: [i32; 2] = pair.as_slice().try_into().unwrap();

                (left, right)
            })
            .collect::<Vec<_>>();

        pairs.sort_unstable_by_key(|&(_, right)| right);

        let mut stack_top = i32::MIN;
        let mut stack_length = 0;

        for (left, right) in pairs {
            if stack_top < left {
                stack_top = right;
                stack_length += 1;
            }
        }

        stack_length
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_longest_chain(pairs: Vec<Vec<i32>>) -> i32 {
        Self::find_longest_chain(pairs)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
