use std::cell::Cell;
use std::cmp::Ordering;

pub mod binary_search;

pub trait Solution {
    fn guess_number(n: i32) -> i32;
}

// static NUM: AtomicI32 = AtomicI32::new(0);

thread_local! {
    static NUM: Cell<i32> = Cell::new(0);
}

fn internal_guess(num: i32) -> i32 {
    match NUM.with(Cell::get).cmp(&num) {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    }
}

/// # Safety
///
/// The `unsafe` here is used to match official function signature, the function is actually safe to use.
pub unsafe fn guess(num: i32) -> i32 {
    internal_guess(num)
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (1, 1),
            (2, 1),
            (2, 2),
            (3, 1),
            (3, 2),
            (3, 3),
            (4, 1),
            (4, 2),
            (4, 3),
            (4, 4),
            (5, 1),
            (5, 2),
            (5, 3),
            (5, 4),
            (5, 5),
            (6, 1),
            (6, 2),
            (6, 3),
            (6, 4),
            (6, 5),
            (6, 6),
        ];

        for (n, num) in test_cases {
            super::NUM.with(|num_cell| {
                num_cell.set(num);

                assert_eq!(S::guess_number(n), num);
            });
        }
    }
}
