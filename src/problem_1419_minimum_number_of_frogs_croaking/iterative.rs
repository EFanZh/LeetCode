pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_number_of_frogs(croak_of_frogs: String) -> i32 {
        let mut state_c = 0;
        let mut state_cr = 0;
        let mut state_cro = 0;
        let mut state_croa = 0;
        let mut in_progress = 0_u32;
        let mut max_in_progress = 0_u32;

        for c in croak_of_frogs.into_bytes() {
            match c {
                b'c' => {
                    state_c += 1;
                    in_progress += 1;
                    max_in_progress = max_in_progress.max(in_progress);
                }
                b'r' => {
                    if state_c == 0 {
                        return -1;
                    }

                    state_c -= 1;
                    state_cr += 1;
                }
                b'o' => {
                    if state_cr == 0 {
                        return -1;
                    }

                    state_cr -= 1;
                    state_cro += 1;
                }
                b'a' => {
                    if state_cro == 0 {
                        return -1;
                    }

                    state_cro -= 1;
                    state_croa += 1;
                }
                _ => {
                    if state_croa == 0 {
                        return -1;
                    }

                    state_croa -= 1;
                    in_progress -= 1;
                }
            }
        }

        if in_progress == 0 { max_in_progress as _ } else { -1 }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_number_of_frogs(croak_of_frogs: String) -> i32 {
        Self::min_number_of_frogs(croak_of_frogs)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
