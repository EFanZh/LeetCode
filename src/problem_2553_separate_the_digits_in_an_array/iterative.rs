pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn separate_digits(nums: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::with_capacity(nums.len() * 2);
        let mut buffer = [0; 10];

        for num in nums {
            let mut num = num as u32;
            let mut i = buffer.len();

            loop {
                i -= 1;
                buffer[i] = (num % 10) as i32;
                num /= 10;

                if num == 0 {
                    break;
                }
            }

            result.extend(&buffer[i..]);
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn separate_digits(nums: Vec<i32>) -> Vec<i32> {
        Self::separate_digits(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
