pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::collections::BinaryHeap;

impl Solution {
    fn compare_unit_wage((lhs_quality, lhs_wage): (u32, u32), (rhs_quality, rhs_wage): (u32, u32)) -> Ordering {
        (lhs_wage * rhs_quality).cmp(&(rhs_wage * lhs_quality))
    }

    pub fn mincost_to_hire_workers(quality: Vec<i32>, wage: Vec<i32>, k: i32) -> f64 {
        let k = k as usize;

        let mut workers = quality
            .iter()
            .zip(&wage)
            .map(|(&quality, &wage)| ((quality as u32), (wage as u32)))
            .collect::<Vec<_>>();

        workers.sort_unstable_by(|&lhs, &rhs| Self::compare_unit_wage(lhs, rhs));

        let (left, right) = workers.split_at(k);
        let mut heap = left.iter().map(|&(quality, _)| quality).collect::<BinaryHeap<_>>();
        let mut total_quality = heap.iter().sum::<u32>();

        let unit_wage = left
            .iter()
            .copied()
            .max_by(|&lhs, &rhs| Self::compare_unit_wage(lhs, rhs))
            .unwrap();

        let mut result = f64::from(total_quality) * f64::from(unit_wage.1) / f64::from(unit_wage.0);

        for &(quality, wage) in right {
            let mut top = heap.peek_mut().unwrap();

            if quality < *top {
                total_quality -= *top - quality;
                *top = quality;

                let new_total_wage = f64::from(total_quality) * f64::from(wage) / f64::from(quality);

                if new_total_wage < result {
                    result = new_total_wage;
                }
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn mincost_to_hire_workers(quality: Vec<i32>, wage: Vec<i32>, k: i32) -> f64 {
        Self::mincost_to_hire_workers(quality, wage, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
