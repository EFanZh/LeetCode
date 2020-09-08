pub struct Solution;

use std::cmp::Ordering;

impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut nums_string = nums.iter().map(i32::to_string).collect::<Vec<_>>();

        nums_string.sort_unstable_by(|lhs_str, rhs_str| {
            let mut lhs_iter = lhs_str.bytes();
            let mut lhs = lhs_iter.next().unwrap();
            let mut rhs_iter = rhs_str.bytes();
            let mut rhs = rhs_iter.next().unwrap();

            loop {
                let mut c_1 = lhs;
                let mut it_1 = lhs_iter.clone();
                let mut c_2 = rhs;
                let mut it_2 = rhs_iter.clone();

                loop {
                    match c_2.cmp(&c_1) {
                        Ordering::Less => return Ordering::Less,
                        Ordering::Equal => match (it_1.next(), it_2.next()) {
                            (None, None) => return Ordering::Equal,
                            (None, Some(new_c_2)) => {
                                rhs = new_c_2;
                                rhs_iter = it_2;

                                break;
                            }
                            (Some(new_c_1), None) => {
                                lhs = new_c_1;
                                lhs_iter = it_1;

                                break;
                            }
                            (Some(new_c_1), Some(new_c_2)) => {
                                c_1 = new_c_1;
                                c_2 = new_c_2;
                            }
                        },
                        Ordering::Greater => return Ordering::Greater,
                    }
                }
            }
        });

        if nums_string.first().map(String::as_str) == Some("0") {
            "0".to_string()
        } else {
            nums_string.join("")
        }
    }
}

impl super::Solution for Solution {
    fn largest_number(nums: Vec<i32>) -> String {
        Self::largest_number(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
