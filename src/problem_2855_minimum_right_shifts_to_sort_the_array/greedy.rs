pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn minimum_right_shifts(nums: Vec<i32>) -> i32 {
        let mut iter = nums.iter().copied();
        let first = iter.next().unwrap();
        let mut prev = first;

        loop {
            if let Some(num) = iter.next() {
                if num < prev {
                    prev = num;

                    break;
                }

                prev = num;
            } else {
                return 0;
            }
        }

        if prev >= first {
            return -1;
        }

        let mut shift = 1;

        for num in iter {
            if num < prev || num > first {
                return -1;
            }

            prev = num;
            shift += 1;
        }

        shift
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_right_shifts(nums: Vec<i32>) -> i32 {
        Self::minimum_right_shifts(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
