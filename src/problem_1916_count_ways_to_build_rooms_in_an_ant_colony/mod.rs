pub mod recursive_with_mod_inverse;

pub trait Solution {
    fn ways_to_build_rooms(prev_room: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[-1, 0, 1] as &[_], 1), (&[-1, 0, 0, 1, 2], 6)];

        for (prev_room, expected) in test_cases {
            assert_eq!(S::ways_to_build_rooms(prev_room.to_vec()), expected);
        }
    }
}
