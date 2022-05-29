pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn shortest_common_supersequence(str1: String, str2: String) -> String {
        let rows = str1.len();
        let columns = str2.len();
        let mut cache = vec![0_u16; columns * (rows + 1)];
        let mut iter = cache.chunks_exact_mut(columns);
        let bottom_row = iter.next_back().unwrap();

        for (value, target) in (1..).zip(bottom_row.iter_mut().rev()) {
            *target = value;
        }

        let mut bottom_row = &*bottom_row;

        for (i, (targets, lhs)) in (0..).zip(iter.zip(str1.bytes()).rev()) {
            let mut bottom_right = i;
            let mut right = i + 1;

            for ((target, &bottom), rhs) in targets.iter_mut().zip(bottom_row).zip(str2.bytes()).rev() {
                *target = if lhs == rhs { bottom_right } else { right.min(bottom) } + 1;
                bottom_right = bottom;
                right = *target;
            }

            bottom_row = targets;
        }

        let str1 = str1.into_bytes();
        let str2 = str2.into_bytes();
        let length = cache[0];
        let mut result = String::with_capacity(usize::from(length));
        let mut i = 0;
        let mut j = 0;

        loop {
            let choice = match (str1.get(i), str2.get(j)) {
                (None, None) => break,
                (None, Some(&rhs)) => {
                    j += 1;

                    rhs
                }
                (Some(&lhs), None) => {
                    i += 1;

                    lhs
                }
                (Some(&lhs), Some(&rhs)) => {
                    if lhs == rhs {
                        i += 1;
                        j += 1;

                        lhs
                    } else if cache[columns * i + j + 1] < cache[columns * (i + 1) + j] {
                        j += 1;

                        rhs
                    } else {
                        i += 1;

                        lhs
                    }
                }
            };

            result.push(char::from(choice));
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn shortest_common_supersequence(str1: String, str2: String) -> String {
        Self::shortest_common_supersequence(str1, str2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
