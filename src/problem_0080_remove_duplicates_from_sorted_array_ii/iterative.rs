pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut prev = nums.first().copied().unwrap();
        let mut deduped = 1;
        let mut i = 1;

        'outer: while let Some(current) = nums.get(i).copied() {
            nums.swap(deduped, i);
            deduped += 1;
            i += 1;

            if current == prev {
                while let Some(current) = nums.get(i).copied() {
                    if current == prev {
                        i += 1;
                    } else {
                        nums.swap(deduped, i);

                        deduped += 1;
                        prev = current;
                        i += 1;

                        continue 'outer;
                    }
                }

                break;
            }

            prev = current;
        }

        nums.truncate(deduped);

        nums.len() as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        Self::remove_duplicates(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
