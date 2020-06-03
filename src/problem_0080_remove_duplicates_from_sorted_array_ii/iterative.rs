pub struct Solution {}

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if let Some(mut prev) = nums.first().copied() {
            let mut deduped = 1;
            let mut i = 1;

            'outer: while let Some(current) = nums.get(i).copied() {
                nums.swap(deduped, i);

                if current == prev {
                    deduped += 1;
                    i += 1;

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
                } else {
                    deduped += 1;
                    prev = current;
                    i += 1;
                }
            }

            nums.truncate(deduped);
        }

        nums.len() as _
    }
}

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
