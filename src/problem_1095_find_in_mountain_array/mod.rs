pub mod binary_search;

use std::cell::Cell;

pub struct MountainArray<'a> {
    data: &'a [i32],
    remaining_get_calls: Cell<u8>,
}

impl MountainArray<'_> {
    fn get(&self, index: i32) -> i32 {
        assert_ne!(self.remaining_get_calls.get(), 0);

        self.remaining_get_calls.set(self.remaining_get_calls.get());

        self.data[index as usize]
    }

    fn length(&self) -> i32 {
        self.data.len() as _
    }
}

pub trait Solution {
    fn find_in_mountain_array(target: i32, mountain_arr: &MountainArray) -> i32;
}

#[cfg(test)]
mod tests {
    use super::{MountainArray, Solution};
    use std::cell::Cell;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 2, 3, 4, 5, 3, 1] as &[_], 3), 2),
            ((&[0, 1, 2, 4, 2, 1], 3), -1),
            ((&[1, 5, 2], 2), 2),
        ];

        for ((mountain_arr, target), expected) in test_cases {
            assert_eq!(
                S::find_in_mountain_array(
                    target,
                    &MountainArray {
                        data: mountain_arr,
                        remaining_get_calls: Cell::new(100)
                    },
                ),
                expected,
            );
        }
    }
}
