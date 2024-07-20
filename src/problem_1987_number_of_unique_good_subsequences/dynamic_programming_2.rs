pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn add(lhs: u32, rhs: u32) -> u32 {
        let result = lhs + rhs;

        result.checked_sub(1_000_000_007).unwrap_or(result)
    }

    pub fn number_of_unique_good_subsequences(binary: String) -> i32 {
        let n = binary.len() as u32;
        let mut iter = binary.into_bytes().into_iter();

        iter.position(|c| c == b'0').map_or(n, |i| {
            let mut counts = [i as u32; 2];

            for c in iter {
                let i = usize::from(c & 1);

                counts[i] = Self::add(counts[i], counts[1 - i] + i as u32);
            }

            let [zero, one] = counts;

            Self::add(zero + 1, one)
        }) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn number_of_unique_good_subsequences(binary: String) -> i32 {
        Self::number_of_unique_good_subsequences(binary)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
