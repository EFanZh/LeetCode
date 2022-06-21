pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn add_negabinary(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let (arr1, mut arr2) = if arr2.len() < arr1.len() {
            (arr2, arr1)
        } else {
            (arr1, arr2)
        };

        let mut iter = arr2.iter_mut().rev();
        let mut carry = 0;

        for (value, target) in arr1.into_iter().rev().zip(&mut iter) {
            *target += value + carry;

            match target {
                -1 => {
                    *target = 1;
                    carry = 1;
                }
                2 => {
                    *target = 0;
                    carry = -1;
                }
                3 => {
                    *target = 1;
                    carry = -1;
                }
                _ => carry = 0,
            }
        }

        if carry != 0 {
            loop {
                if let Some(target) = iter.next() {
                    *target += carry;

                    match target {
                        -1 => {
                            *target = 1;
                            carry = 1;
                        }
                        0 => {
                            if let Some(i) = arr2.iter().position(|&d| d != 0) {
                                arr2.splice(0..i, []);
                            } else {
                                arr2.truncate(1);
                            }

                            break;
                        }
                        1 => break,
                        _ => {
                            *target = 0;
                            carry = -1;
                        }
                    }
                } else {
                    arr2.splice(0..0, [1, 1]);

                    break;
                }
            }
        }

        arr2
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn add_negabinary(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        Self::add_negabinary(arr1, arr2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
