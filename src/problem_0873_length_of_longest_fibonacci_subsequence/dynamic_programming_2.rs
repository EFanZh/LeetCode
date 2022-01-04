pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;

impl Solution {
    pub fn len_longest_fib_subseq(arr: Vec<i32>) -> i32 {
        assert!(arr.len() > 2);

        let indices = arr
            .iter()
            .enumerate()
            .map(|(i, &value)| (value, i))
            .collect::<HashMap<_, _>>();

        let n = arr.len();
        let mut lengths = vec![0; (n - 1) * (n - 2) / 2];
        let mut target_index = 0;

        for (right_index, &right) in (2..).zip(&arr[2..]) {
            if right == arr[0] + arr[1] {
                lengths[target_index] = 1;
            }

            target_index += 1;

            for (middle_index, &middle) in (2..).zip(&arr[2..right_index]) {
                let left = right - middle;

                if left < middle {
                    if let Some(&left_index) = indices.get(&left) {
                        lengths[target_index] = if left_index == 0 {
                            1
                        } else {
                            lengths[(middle_index - 1) * (middle_index - 2) / 2 + left_index - 1] + 1
                        }
                    }
                }

                target_index += 1;
            }
        }

        let result = lengths.iter().copied().max().unwrap();

        if result == 0 {
            result
        } else {
            result + 2
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn len_longest_fib_subseq(arr: Vec<i32>) -> i32 {
        Self::len_longest_fib_subseq(arr)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
