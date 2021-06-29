pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn is_possible(nums: Vec<i32>) -> bool {
        let mut one = 0;
        let mut two = 0;
        let mut three = 0;
        let mut prev_chunk = i32::MIN;
        let mut chunk = i32::MIN;
        let mut length = 0;

        for num in nums {
            if num == chunk {
                length += 1;
            } else {
                if chunk == prev_chunk + 1 {
                    let one_plus_two = one + two;

                    if length < one_plus_two {
                        return false;
                    }

                    let one_plus_two_plus_three = one_plus_two + three;

                    if length < one_plus_two_plus_three {
                        three = two + (length - one_plus_two);
                        two = one;
                        one = 0;
                    } else {
                        three += two;
                        two = one;
                        one = length - one_plus_two_plus_three;
                    }
                } else {
                    if one != 0 || two != 0 {
                        return false;
                    }

                    three = 0;
                    one = length;
                };

                prev_chunk = chunk;
                chunk = num;
                length = 1;
            }
        }

        one == 0 && length <= two + three
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn is_possible(nums: Vec<i32>) -> bool {
        Self::is_possible(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
