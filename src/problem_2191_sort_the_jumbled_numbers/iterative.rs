pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn get_digits(mapping: &[u8; 10], num: i32) -> u32 {
        let mut num = num as u32;

        if num == 0 {
            u32::from(mapping[0])
        } else {
            let mut result = 0;
            let mut base = 1;

            loop {
                result += base * u32::from(mapping[(num % 10) as usize]);
                num /= 10;
                base *= 10;

                if num == 0 {
                    break;
                }
            }

            result
        }
    }

    pub fn sort_jumbled(mapping: Vec<i32>, nums: Vec<i32>) -> Vec<i32> {
        let mapping = <[_; 10]>::try_from(mapping).ok().unwrap().map(|x| x as u8);
        let mut nums = nums;

        nums.sort_by_cached_key(|&num| Self::get_digits(&mapping, num));

        nums
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn sort_jumbled(mapping: Vec<i32>, nums: Vec<i32>) -> Vec<i32> {
        Self::sort_jumbled(mapping, nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
