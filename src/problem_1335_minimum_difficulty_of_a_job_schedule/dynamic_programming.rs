pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

// See <https://leetcode.com/problems/minimum-difficulty-of-a-job-schedule/discuss/490316/JavaC%2B%2BPython3-DP-O(nd)-Solution>.

use std::mem;

impl Solution {
    pub fn min_difficulty(job_difficulty: Vec<i32>, d: i32) -> i32 {
        let n = job_difficulty.len();
        let d = d as usize;

        if n < d {
            -1
        } else {
            let mut buffer = vec![0; n * 2];
            let (mut cache, mut temp) = buffer.split_at_mut(n);
            let mut max = 0;

            for (target, &difficulty) in cache.iter_mut().zip(&job_difficulty) {
                max = max.max(difficulty);
                *target = max;
            }

            let mut stack = Vec::new();

            for start in 1..d {
                for (i, (&prev, &value)) in (start..).zip(cache[start - 1..].iter().zip(&job_difficulty[start..])) {
                    let mut min = prev + value;

                    while let Some(&j) = stack.last() {
                        let left_difficulty = job_difficulty[j];
                        let left_cost = temp[j];

                        if left_difficulty <= value {
                            min = min.min(left_cost + (value - left_difficulty));

                            stack.pop();
                        } else {
                            min = min.min(left_cost);

                            break;
                        }
                    }

                    stack.push(i);
                    temp[i] = min;
                }

                stack.clear();
                mem::swap(&mut cache, &mut temp);
            }

            *cache.last().unwrap()
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_difficulty(job_difficulty: Vec<i32>, d: i32) -> i32 {
        Self::min_difficulty(job_difficulty, d)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
