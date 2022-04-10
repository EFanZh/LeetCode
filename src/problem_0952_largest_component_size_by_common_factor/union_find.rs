pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::collections::HashMap;
use std::num::{NonZeroU16, NonZeroU32};

macro_rules! primes_list {
    ($($x:expr,)*) => { [$(NonZeroU16::new($x).unwrap(),)*] };
}

impl Solution {
    fn get_prime_factors(mut value: u32, mut f: impl FnMut(u32)) {
        let primes = primes_list![
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101, 103,
            107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193, 197, 199, 211, 223,
            227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307, 311, 313,
        ];

        for &prime in &primes {
            let prime = NonZeroU32::from(prime);

            if prime.get() <= value {
                if value % prime == 0 {
                    f(prime.get());

                    value = value / prime;

                    while value % prime == 0 {
                        value = value / prime;
                    }
                }
            } else {
                break;
            }
        }

        if value != 1 {
            f(value);
        }
    }

    fn find_root(nodes: &mut [(u16, u16, u16)], node: u16) -> (u16, u16, u16) {
        let (parent, rank, size) = nodes[usize::from(node)];

        if parent == u16::MAX {
            (node, rank, size)
        } else {
            let (root, root_rank, root_size) = Self::find_root(nodes, parent);

            nodes[usize::from(node)].0 = root;

            (root, root_rank, root_size)
        }
    }

    pub fn largest_component_size(nums: Vec<i32>) -> i32 {
        // Build relations.

        let n = nums.len();
        let mut numbers_by_factors = HashMap::new();

        for (id, num) in (0..).zip(nums) {
            Self::get_prime_factors(num as _, |factor| {
                numbers_by_factors
                    .entry(factor)
                    .and_modify(|numbers| Vec::push(numbers, id))
                    .or_insert_with(|| vec![id]);
            });
        }

        // Union find.

        let mut nodes = vec![(u16::MAX, 0_u16, 1_u16); n]; // [(parent, rank, size)];
        let mut result = 1;

        for numbers in numbers_by_factors.values() {
            let (&x, rest) = numbers.split_first().unwrap();
            let (mut x_root, mut x_rank, mut x_size) = Self::find_root(&mut nodes, x);

            for &y in rest {
                let (y_root, y_rank, y_size) = Self::find_root(&mut nodes, y);

                if y_root != x_root {
                    match y_rank.cmp(&x_rank) {
                        Ordering::Less => {
                            nodes[usize::from(y_root)].0 = x_root;
                            nodes[usize::from(x_root)].2 += y_size;
                        }
                        Ordering::Equal => {
                            nodes[usize::from(y_root)].0 = x_root;

                            let x_node = &mut nodes[usize::from(x_root)];

                            x_node.1 += 1;
                            x_node.2 += y_size;
                            x_rank += 1;
                        }
                        Ordering::Greater => {
                            nodes[usize::from(x_root)].0 = y_root;
                            nodes[usize::from(y_root)].2 += x_size;

                            x_root = y_root;
                            x_rank = y_rank;
                        }
                    }

                    x_size += y_size;
                }
            }

            result = result.max(x_size);
        }

        i32::from(result)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn largest_component_size(nums: Vec<i32>) -> i32 {
        Self::largest_component_size(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
