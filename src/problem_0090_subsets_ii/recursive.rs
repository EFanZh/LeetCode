pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn subsets_with_dup_helper(first: i32, mut rest: &[i32], base: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        let saved_size = base.len();
        let mut count = 0;

        loop {
            if let Some((&num, new_rest)) = rest.split_first() {
                if num == first {
                    count += 1;
                    rest = new_rest;
                } else {
                    Self::subsets_with_dup_helper(num, new_rest, base, result);

                    loop {
                        base.push(first);

                        Self::subsets_with_dup_helper(num, new_rest, base, result);

                        if count == 0 {
                            break;
                        }

                        count -= 1;
                    }

                    base.truncate(saved_size);

                    break;
                }
            } else {
                result.push(base.clone());

                loop {
                    base.push(first);
                    result.push(base.clone());

                    if count == 0 {
                        break;
                    }

                    count -= 1;
                }

                base.truncate(saved_size);

                break;
            }
        }
    }

    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();

        let (&first, rest) = nums.split_first().unwrap();
        let mut result = Vec::new();

        Self::subsets_with_dup_helper(first, rest, &mut Vec::new(), &mut result);

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        Self::subsets_with_dup(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
