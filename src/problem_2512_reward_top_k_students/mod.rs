pub mod hash_set;

pub trait Solution {
    fn top_students(
        positive_feedback: Vec<String>,
        negative_feedback: Vec<String>,
        report: Vec<String>,
        student_id: Vec<i32>,
        k: i32,
    ) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (
                    &["smart", "brilliant", "studious"] as &[_],
                    &["not"] as &[_],
                    &["this student is studious", "the student is smart"] as &[_],
                    &[1, 2] as &[_],
                    2,
                ),
                &[1, 2] as &[_],
            ),
            (
                (
                    &["smart", "brilliant", "studious"],
                    &["not"],
                    &["this student is not studious", "the student is smart"],
                    &[1, 2],
                    2,
                ),
                &[2, 1],
            ),
        ];

        for ((positive_feedback, negative_feedback, report, student_id, k), expected) in test_cases {
            assert_eq!(
                S::top_students(
                    positive_feedback.iter().copied().map(str::to_string).collect(),
                    negative_feedback.iter().copied().map(str::to_string).collect(),
                    report.iter().copied().map(str::to_string).collect(),
                    student_id.to_vec(),
                    k
                ),
                expected,
            );
        }
    }
}
