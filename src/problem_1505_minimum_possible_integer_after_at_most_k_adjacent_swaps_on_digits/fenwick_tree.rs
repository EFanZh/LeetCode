pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::VecDeque;

impl Solution {
    fn fenwick_tree_add(tree: &mut [u16], mut index: usize, diff: u16) {
        while let Some(value) = tree.get_mut(index.wrapping_sub(1)) {
            *value = value.wrapping_add(diff);
            index += index & index.wrapping_neg();
        }
    }

    fn fenwick_tree_sum(tree: &[u16], mut index: usize) -> u16 {
        let mut result = 0;

        while let Some(&value) = tree.get(index.wrapping_sub(1)) {
            result += value;
            index &= index - 1;
        }

        result
    }

    fn helper(num: &mut [u8], k: usize) -> Vec<u8> {
        let n = num.len();
        let mut k = k as u32 as usize;
        let mut digit_indices = [(); 10].map(|()| VecDeque::<u16>::new());
        let mut non_empty_digits = 0_u16;
        let mut fenwick_tree = vec![0; n].into_boxed_slice();
        let mut end = 0;
        let mut result = Vec::with_capacity(n);

        while k != 0 {
            // Extend `end` to at least `result.len() + k + 1`.

            let new_end = (result.len() + k + 1).min(n);

            if end < new_end {
                for (index, &digit) in (end..).zip(&num[end..new_end]) {
                    let digit = usize::from(digit) - usize::from(b'0');

                    digit_indices[digit].push_back(index as _);
                    non_empty_digits |= 1 << digit;
                    Self::fenwick_tree_add(&mut fenwick_tree, index + 1, 1);
                }

                end = new_end;
            }

            // Find the smallest digit in window.

            let mut digits_iter = non_empty_digits;

            loop {
                let digit = digits_iter.trailing_zeros() as u8;
                let indices = &mut digit_indices[usize::from(digit)];
                let front_index = usize::from(*indices.front().unwrap());
                let position_in_window = usize::from(Self::fenwick_tree_sum(&fenwick_tree, front_index + 1) - 1);

                // If we have enough moves left to use this digit.

                if position_in_window <= k {
                    // Swap the digit at `front_index` at cost of `position_in_window`;

                    k -= position_in_window;

                    indices.pop_front();

                    if indices.is_empty() {
                        non_empty_digits ^= 1 << digit;
                    }

                    // Remove the digit used from the Fenwick tree.

                    Self::fenwick_tree_add(&mut fenwick_tree, front_index + 1, u16::MAX);

                    // Mark the selected digit as used.

                    num[front_index] = 0;

                    // Append the selected digit to `result`.

                    result.push(b'0' + digit);

                    break;
                }

                digits_iter &= digits_iter - 1;
            }

            if result.len() == n {
                return result;
            }
        }

        // Add remaining digits to result.

        let (left, right) = num.split_at(end);

        for &c in left {
            if c != 0 {
                result.push(c);
            }
        }

        result.extend(right);

        result
    }

    pub fn min_integer(num: String, k: i32) -> String {
        String::from_utf8(Self::helper(&mut num.into_bytes(), k as u32 as usize)).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_integer(num: String, k: i32) -> String {
        Self::min_integer(num, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
