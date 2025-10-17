pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn minimum_deletions(word: String, k: i32) -> i32 {
        let n = word.len() as u32;
        let k = k.cast_unsigned();
        let mut counts = [0_u32; 26];

        for c in word.bytes() {
            counts[usize::from(c) - usize::from(b'a')] += 1;
        }

        drop(word);

        counts.sort_unstable();

        let split = counts.iter().position(|&count| count != 0).unwrap();
        let counts = &counts[split..];
        let mut left_sum = 0;
        let mut right = 0;
        let mut right_sum = n;
        let mut result = u32::MAX;

        'outer: for &count in counts {
            loop {
                if let Some(&right_count) = counts.get(right) {
                    let threshold = count + k;

                    if right_count <= threshold {
                        right += 1;
                        right_sum -= right_count;
                    } else {
                        result = result.min(left_sum + right_sum - threshold * (counts.len() as u32 - right as u32));

                        break;
                    }
                } else {
                    result = result.min(left_sum);

                    break 'outer;
                }
            }

            left_sum += count;
        }

        result.cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_deletions(word: String, k: i32) -> i32 {
        Self::minimum_deletions(word, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
