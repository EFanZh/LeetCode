pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn count_dominant_elements(nums: Vec<i32>) -> i32 {
        let nums = nums.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>();
        let mut iter = nums.iter().copied();

        iter.next().map_or(0, |mut left| {
            iter.next().map_or(1, |mut middle| {
                let mut result = i32::from(left >= middle);

                for right in iter {
                    result += i32::from(middle * 2 >= left + right);
                    left = middle;
                    middle = right;
                }

                result + i32::from(middle >= left)
            })
        })
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_dominant_elements(nums: Vec<i32>) -> i32 {
        Self::count_dominant_elements(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
