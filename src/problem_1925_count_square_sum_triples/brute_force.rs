pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn count_triples(n: i32) -> i32 {
        let n = u16::from(n as u8);
        let mut result = 0;

        for c in 5..=n {
            let c_squared = c * c;
            let half_c_squared = c_squared / 2;

            for a in 1_u16.. {
                let a_squared = a * a;

                if a_squared <= half_c_squared {
                    let b_squared = c_squared - a_squared;

                    if (f32::from(b_squared).sqrt() as u16).pow(2) == b_squared {
                        result += 2;
                    }
                } else {
                    break;
                }
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_triples(n: i32) -> i32 {
        Self::count_triples(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
