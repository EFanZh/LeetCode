pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn can_change(start: String, target: String) -> bool {
        let top = start.as_bytes();
        let bottom = target.as_bytes();
        let mut i = 0;
        let mut j = 0;

        while let Some(&y) = bottom.get(j) {
            match y {
                b'L' => loop {
                    if let Some(&x) = top.get(i) {
                        match x {
                            b'L' => {
                                if i < j {
                                    return false;
                                }

                                i += 1;

                                break;
                            }
                            b'R' => return false,
                            _ => i += 1,
                        }
                    } else {
                        return false;
                    }
                },
                b'R' => {
                    let window = &top[..=j];

                    loop {
                        if let Some(&x) = window.get(i) {
                            match x {
                                b'L' => return false,
                                b'R' => {
                                    i += 1;

                                    break;
                                }
                                _ => i += 1,
                            }
                        } else {
                            return false;
                        }
                    }
                }
                _ => {}
            }

            j += 1;
        }

        while let Some(&c) = top.get(i) {
            if c != b'_' {
                return false;
            }

            i += 1;
        }

        true
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn can_change(start: String, target: String) -> bool {
        Self::can_change(start, target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
