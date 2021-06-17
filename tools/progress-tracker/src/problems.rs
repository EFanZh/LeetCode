use serde::Deserialize;

#[derive(Deserialize)]
pub struct Stat {
    #[serde(rename = "question__title")]
    pub title: String,
    #[serde(rename = "question__title_slug")]
    pub title_slug: String,
    pub frontend_question_id: u16,
}

#[derive(Deserialize)]
pub struct Difficulty {
    pub level: u8,
}

#[derive(Deserialize)]
pub struct Problem {
    pub stat: Stat,
    pub difficulty: Difficulty,
    pub paid_only: bool,
}

impl Problem {
    pub fn get_id(&self) -> String {
        format!("{:04}-{}", self.stat.frontend_question_id, self.stat.title_slug)
    }
}

#[derive(Deserialize)]
pub struct Problems {
    #[serde(rename = "stat_status_pairs")]
    pub problems: Vec<Problem>,
}

impl Problems {
    pub fn retain_free(&mut self) {
        self.problems.retain(|problem| !problem.paid_only);
    }
}

#[cfg(test)]
mod tests {
    use super::{Problem, Stat};

    #[test]
    fn test_deserialize_stat() {
        let source = r#"{
            "question_id": 1,
            "question__article__live": true,
            "question__article__slug": "two-sum",
            "question__title": "Two Sum",
            "question__title_slug": "two-sum",
            "question__hide": false,
            "total_acs": 2713958,
            "total_submitted": 6004485,
            "frontend_question_id": 1,
            "is_new_question": false
        }"#;

        let result = serde_json::from_str::<Stat>(source).unwrap();

        assert_eq!(result.title, "Two Sum");
        assert_eq!(result.title_slug, "two-sum");
        assert_eq!(result.frontend_question_id, 1);
    }

    #[test]
    fn test_deserialize_problem() {
        let source = r#"{
            "stat": {
                "question_id": 1,
                "question__article__live": true,
                "question__article__slug": "two-sum",
                "question__title": "Two Sum",
                "question__title_slug": "two-sum",
                "question__hide": false,
                "total_acs": 2713958,
                "total_submitted": 6004485,
                "frontend_question_id": 1,
                "is_new_question": false
            },
            "status": null,
            "difficulty": {
                "level": 1
            },
            "paid_only": false,
            "is_favor": false,
            "frequency": 0,
            "progress": 0
        }"#;

        let result = serde_json::from_str::<Problem>(source).unwrap();

        assert_eq!(result.stat.title, "Two Sum");
        assert_eq!(result.stat.title_slug, "two-sum");
        assert_eq!(result.stat.frontend_question_id, 1);
        assert!(!result.paid_only);
    }
}
