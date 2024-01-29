pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn build_kmp_table(values: &[i32], target: &mut [usize]) {
        target[0] = 0;

        let mut matched = 0;
        let mut i = 1;

        while let Some(&value) = values.get(i) {
            loop {
                if value == values[matched] {
                    matched += 1;

                    break;
                } else if let Some(&next_matched) = target.get(matched.wrapping_sub(1)) {
                    matched = next_matched;
                } else {
                    break;
                }
            }

            target[i] = matched;
            i += 1;
        }
    }

    pub fn can_choose(groups: Vec<Vec<i32>>, nums: Vec<i32>) -> bool {
        let mut iter = nums.iter().copied();
        let mut kmp_table = Vec::new();

        for group in groups {
            kmp_table.resize(group.len(), 0);

            Self::build_kmp_table(&group, &mut kmp_table);

            let mut matched = 0;

            'outer: while let Some(&expected) = group.get(matched) {
                let mut expected = expected;

                loop {
                    if let Some(value) = iter.next() {
                        loop {
                            if value == expected {
                                matched += 1;

                                continue 'outer;
                            } else if let Some(&next_matched) = kmp_table.get(matched.wrapping_sub(1)) {
                                matched = next_matched;
                                expected = group[matched];
                            } else {
                                break;
                            }
                        }
                    } else {
                        return false;
                    }
                }
            }
        }

        true
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn can_choose(groups: Vec<Vec<i32>>, nums: Vec<i32>) -> bool {
        Self::can_choose(groups, nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
