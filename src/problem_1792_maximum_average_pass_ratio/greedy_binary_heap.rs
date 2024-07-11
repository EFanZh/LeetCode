pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::collections::BinaryHeap;

struct Class {
    not_pass: u32,
    total: u32,
}

impl PartialEq for Class {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Class {}

impl PartialOrd for Class {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Class {
    fn cmp(&self, other: &Self) -> Ordering {
        let lhs = u64::from(self.not_pass) * u64::from(other.total) * u64::from(other.total + 1);
        let rhs = u64::from(other.not_pass) * u64::from(self.total) * u64::from(self.total + 1);

        lhs.cmp(&rhs)
    }
}

impl Solution {
    pub fn max_average_ratio(classes: Vec<Vec<i32>>, extra_students: i32) -> f64 {
        let mut classes = classes
            .into_iter()
            .map(|class| {
                let [pass, total] = class.try_into().ok().unwrap();
                let pass = pass as u32;
                let total = total as u32;

                Class {
                    not_pass: total - pass,
                    total,
                }
            })
            .collect::<BinaryHeap<_>>();

        for _ in 0..extra_students {
            classes.peek_mut().unwrap().total += 1;
        }

        let mut numerator = 0.0;

        for &Class { not_pass, total } in &classes {
            numerator += f64::from(total - not_pass) / f64::from(total);
        }

        numerator / f64::from(classes.len() as u32)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_average_ratio(classes: Vec<Vec<i32>>, extra_students: i32) -> f64 {
        Self::max_average_ratio(classes, extra_students)
    }
}

#[cfg(test)]
mod tests {
    use super::Class;

    #[test]
    fn test_class_partial_eq() {
        assert!(Class { not_pass: 2, total: 3 } == Class { not_pass: 2, total: 3 });
    }

    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
