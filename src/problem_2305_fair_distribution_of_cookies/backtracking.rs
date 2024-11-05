pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::slice::Iter;

impl Solution {
    fn helper(counts: &mut [u32], max_count: u32, result: &mut u32, mut iter: Iter<i32>) {
        if let Some(&bag) = iter.next() {
            let bag = bag as u32;

            for i in 0..counts.len() {
                counts[i] += bag;

                if counts[i] < *result {
                    Self::helper(counts, max_count.max(counts[i]), result, iter.clone());
                }

                counts[i] -= bag;

                if counts[i] == 0 {
                    break;
                }
            }
        } else {
            *result = max_count;
        }
    }

    pub fn distribute_cookies(cookies: Vec<i32>, k: i32) -> i32 {
        let mut result = u32::MAX;

        Self::helper(&mut [0; 8][..k as u32 as usize], 0, &mut result, cookies.iter());

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn distribute_cookies(cookies: Vec<i32>, k: i32) -> i32 {
        Self::distribute_cookies(cookies, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
