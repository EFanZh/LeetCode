pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashSet;

impl Solution {
    #[allow(clippy::if_then_some_else_none)]
    pub fn fair_candy_swap(alice_sizes: Vec<i32>, bob_sizes: Vec<i32>) -> Vec<i32> {
        let alice_total = alice_sizes.iter().sum::<i32>();
        let bob_total = bob_sizes.iter().sum::<i32>();
        let diff = (bob_total - alice_total) / 2;
        let alice_sizes = alice_sizes.into_iter().collect::<HashSet<_>>();

        bob_sizes
            .into_iter()
            .find_map(|bob_size| {
                let alice_size = bob_size - diff;

                if alice_sizes.contains(&alice_size) {
                    Some(vec![alice_size, bob_size])
                } else {
                    None
                }
            })
            .unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn fair_candy_swap(alice_sizes: Vec<i32>, bob_sizes: Vec<i32>) -> Vec<i32> {
        Self::fair_candy_swap(alice_sizes, bob_sizes)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
