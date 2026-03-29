pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn largest_integer(nums: Vec<i32>, k: i32) -> i32 {
        match *nums {
            [] => -1,
            [x] => x,
            [head, .., tail] => {
                let k = k.cast_unsigned() as usize;
                let mut result = -1;

                if k == nums.len() {
                    for &num in &nums {
                        result = result.max(num);
                    }
                } else {
                    let mut counts = [0_u8; 51];

                    for &num in &nums {
                        counts[num.cast_unsigned() as usize] += 1;
                    }

                    if k == 1 {
                        if let Some(i) = counts.iter().rposition(|&count| count == 1) {
                            result = i as _;
                        }
                    } else {
                        if counts[head.cast_unsigned() as usize] == 1 {
                            result = head;
                        }

                        if counts[tail.cast_unsigned() as usize] == 1 {
                            result = result.max(tail);
                        }
                    }
                }

                result
            }
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn largest_integer(nums: Vec<i32>, k: i32) -> i32 {
        Self::largest_integer(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
