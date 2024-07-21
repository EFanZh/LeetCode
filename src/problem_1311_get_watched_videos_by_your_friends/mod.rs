pub mod bfs;

pub trait Solution {
    fn watched_videos_by_friends(
        watched_videos: Vec<Vec<String>>,
        friends: Vec<Vec<i32>>,
        id: i32,
        level: i32,
    ) -> Vec<String>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (
                    &[&["A", "B"] as &[_], &["C"], &["B", "C"], &["D"]] as &[&[_]],
                    &[&[1, 2] as &[_], &[0, 3], &[0, 3], &[1, 2]] as &[&[_]],
                    0,
                    1,
                ),
                &["B", "C"] as &[_],
            ),
            (
                (
                    &[&["A", "B"], &["C"], &["B", "C"], &["D"]],
                    &[&[1, 2], &[0, 3], &[0, 3], &[1, 2]],
                    0,
                    2,
                ),
                &["D"],
            ),
            (
                (
                    &[&["A", "B"], &["C"], &["B", "C"], &["D"]],
                    &[&[1, 2], &[0, 3], &[0, 3], &[1, 2]],
                    0,
                    5,
                ),
                &[],
            ),
        ];

        for ((watched_videos, friends, id, level), expected) in test_cases {
            assert_eq!(
                S::watched_videos_by_friends(
                    watched_videos
                        .iter()
                        .map(|watched_videos| watched_videos.iter().copied().map(str::to_string).collect())
                        .collect(),
                    friends.iter().copied().map(<[_]>::to_vec).collect(),
                    id,
                    level,
                ),
                expected,
            );
        }
    }
}
