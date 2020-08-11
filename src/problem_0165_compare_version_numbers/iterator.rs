pub struct Solution {}

use std::cmp::Ordering;

impl Solution {
    fn get_version_components<'a>(version: &'a str) -> impl Iterator<Item = u32> + 'a {
        version.split('.').map(|x| x.parse::<u32>().unwrap())
    }

    pub fn compare_version(version1: String, version2: String) -> i32 {
        let mut iter_1 = Self::get_version_components(&version1);
        let mut iter_2 = Self::get_version_components(&version2);

        loop {
            return match (iter_1.next(), iter_2.next()) {
                (None, None) => 0,
                (None, Some(rhs)) => {
                    if rhs == 0 && iter_2.all(|c| c == 0) {
                        0
                    } else {
                        -1
                    }
                }
                (Some(lhs), None) => {
                    if lhs == 0 && iter_1.all(|c| c == 0) {
                        0
                    } else {
                        1
                    }
                }
                (Some(lhs), Some(rhs)) => match lhs.cmp(&rhs) {
                    Ordering::Less => -1,
                    Ordering::Equal => continue,
                    Ordering::Greater => 1,
                },
            };
        }
    }
}

impl super::Solution for Solution {
    fn compare_version(version1: String, version2: String) -> i32 {
        Self::compare_version(version1, version2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
