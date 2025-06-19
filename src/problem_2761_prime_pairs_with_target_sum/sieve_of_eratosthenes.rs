pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn find_prime_pairs(n: i32) -> Vec<Vec<i32>> {
        let n = n as u32 as usize;
        let mut result = Vec::new();

        if n >= 4 {
            let mut is_composites = vec![false; n - 3].into_boxed_slice();

            for step in 2..=n.isqrt() {
                if !is_composites[step - 2] {
                    let mut i = step * step - 2;

                    while let Some(is_composite) = is_composites.get_mut(i) {
                        *is_composite = true;
                        i += step;
                    }
                }
            }

            result.extend(
                (2..)
                    .zip(is_composites[..n / 2 - 1].iter().zip(is_composites.iter().rev()))
                    .filter(|&(_, (&lhs_is_composite, &rhs_is_composite))| !lhs_is_composite && !rhs_is_composite)
                    .map(|(lhs, _)| vec![lhs, n as i32 - lhs]),
            );
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_prime_pairs(n: i32) -> Vec<Vec<i32>> {
        Self::find_prime_pairs(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
