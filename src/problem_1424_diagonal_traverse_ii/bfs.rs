pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::VecDeque;

impl Solution {
    pub fn find_diagonal_order(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut queue = VecDeque::new();

        for row in &nums {
            queue.push_front(row.iter().copied());

            for _ in 0..queue.len() {
                let mut iter = queue.pop_front().unwrap();

                if let Some(num) = iter.next() {
                    result.push(num);
                    queue.push_back(iter);
                }
            }
        }

        while let Some(mut iter) = queue.pop_front() {
            if let Some(num) = iter.next() {
                result.push(num);
                queue.push_back(iter);
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_diagonal_order(nums: Vec<Vec<i32>>) -> Vec<i32> {
        Self::find_diagonal_order(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
