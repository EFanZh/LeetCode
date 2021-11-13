pub mod bfs;

pub trait Solution {
    fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[&[1] as &[_], &[2], &[3], &[]] as &[&[_]], true),
            (&[&[1, 3], &[3, 0, 1], &[2], &[0]], false),
        ];

        for (rooms, expected) in test_cases {
            assert_eq!(
                S::can_visit_all_rooms(rooms.iter().copied().map(<[_]>::to_vec).collect()),
                expected
            );
        }
    }
}
