pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn minimize_string_value(s: String) -> String {
        let mut counts = [0_u32; 26];
        let mut extra = 0;

        for c in s.bytes() {
            *counts
                .get_mut(usize::from(c).wrapping_sub(usize::from(b'a')))
                .unwrap_or(&mut extra) += 1;
        }

        let mut counts_sorted = counts.each_mut();

        counts_sorted.sort_unstable();

        let mut sum = 0;
        let mut i = 0;

        for &&mut height in &counts_sorted {
            if extra < height * i - sum {
                break;
            }

            sum += height;
            i += 1;
        }

        counts_sorted[..i as usize].sort_unstable_by_key(|x| &raw const **x);

        let available = sum + extra;
        let height_2 = available / i;
        let width_1 = available % i;
        let height_1 = height_2 + 1;

        counts_sorted[..width_1 as usize]
            .iter_mut()
            .for_each(|x| **x = height_1 - **x);

        counts_sorted[width_1 as usize..i as usize]
            .iter_mut()
            .for_each(|x| **x = height_2 - **x);

        counts_sorted[i as usize..].iter_mut().for_each(|x| **x = 0);

        let mut result = s.into_bytes();
        let mut iter = 0;

        for c in &mut result {
            if *c == b'?' {
                loop {
                    let count = &mut counts[iter];

                    if *count == 0 {
                        iter += 1;
                    } else {
                        *count -= 1;

                        break;
                    }
                }

                *c = b'a' + iter as u8;
            }
        }

        String::from_utf8(result).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimize_string_value(s: String) -> String {
        Self::minimize_string_value(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
