pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Reverse;
use std::collections::HashSet;

impl Solution {
    pub fn top_students(
        positive_feedback: Vec<String>,
        negative_feedback: Vec<String>,
        report: Vec<String>,
        student_id: Vec<i32>,
        k: i32,
    ) -> Vec<i32> {
        let k = k as u32 as usize;

        let positive_feedback = positive_feedback
            .into_iter()
            .map(String::into_bytes)
            .collect::<HashSet<_>>();

        let negative_feedback = negative_feedback
            .into_iter()
            .map(String::into_bytes)
            .collect::<HashSet<_>>();

        let mut students = report
            .into_iter()
            .zip(student_id)
            .map(|(report, student_id)| {
                let points = report
                    .into_bytes()
                    .split(|&c| c == b' ')
                    .map(|word| {
                        if positive_feedback.contains(word) {
                            3
                        } else if negative_feedback.contains(word) {
                            -1
                        } else {
                            0
                        }
                    })
                    .sum::<i32>();

                (Reverse(points), student_id)
            })
            .collect::<Vec<_>>();

        students.select_nth_unstable(k - 1);
        students.truncate(k);
        students.sort_unstable();

        students.into_iter().map(|(_, id)| id).collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn top_students(
        positive_feedback: Vec<String>,
        negative_feedback: Vec<String>,
        report: Vec<String>,
        student_id: Vec<i32>,
        k: i32,
    ) -> Vec<i32> {
        Self::top_students(positive_feedback, negative_feedback, report, student_id, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
