pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::ptr;

impl Solution {
    pub fn recover_array(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let n = nums.len() / 2;
        let mut visited = vec![false; nums.len()].into_boxed_slice();

        nums.sort_unstable();

        let first_num = nums[0];

        for mut j in 1..n {
            let diff = nums[j] - first_num;

            if diff & 1 != 0 || diff == 0 {
                continue;
            }

            let mut i = 1;

            'next: loop {
                visited[j] = true;
                j += 1;

                loop {
                    if let Some(&left) = nums.get(i) {
                        let visited_value = visited[i];

                        i += 1;

                        if !visited_value {
                            let target = left + diff;

                            loop {
                                if let Some(&right) = nums.get(j) {
                                    match right.cmp(&target) {
                                        Ordering::Less => j += 1,
                                        Ordering::Equal => continue 'next,
                                        Ordering::Greater => break 'next,
                                    }
                                } else {
                                    break 'next;
                                }
                            }
                        }
                    } else {
                        let half_diff = diff >> 1;
                        let start_address = nums.as_ptr() as usize;

                        nums.retain_mut(|num| {
                            let retain = !visited[(ptr::from_ref(num) as usize - start_address) / size_of::<i32>()];

                            if retain {
                                *num += half_diff;
                            }

                            retain
                        });

                        return nums;
                    }
                }
            }

            visited.fill(false);
        }

        let half_diff = (nums[n] - first_num) >> 1;

        nums.truncate(n);
        nums.iter_mut().for_each(|x| *x += half_diff);

        nums
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn recover_array(nums: Vec<i32>) -> Vec<i32> {
        Self::recover_array(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
