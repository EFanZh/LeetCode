pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn add(lhs: u32, rhs: u32) -> u32 {
        let result = lhs + rhs;

        result.checked_sub(1_000_000_007).unwrap_or(result)
    }

    fn assign_add(target: &mut u32, value: u32) {
        *target = Self::add(*target, value);
    }

    pub fn number_of_unique_good_subsequences(binary: String) -> i32 {
        let n = binary.len() as u32;
        let mut iter = binary.into_bytes().into_iter();

        iter.position(|c| c == b'0').map_or(n, |i| {
            let mut zero = i as u32;
            let mut one = zero;

            for c in iter {
                if c == b'0' {
                    Self::assign_add(&mut zero, one);
                } else {
                    Self::assign_add(&mut one, zero + 1);
                }
            }

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
