pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::Cell;

impl Solution {
    pub fn maximum_energy(energy: Vec<i32>, k: i32) -> i32 {
        let mut energy = energy;
        let slice = energy.as_mut_slice();
        let cell_slice = Cell::from_mut(slice).as_slice_of_cells();

        cell_slice
            .windows(k.cast_unsigned() as usize + 1)
            .rev()
            .for_each(|window| {
                let first = window.first().unwrap();
                let last = window.last().unwrap();

                first.set(first.get() + last.get());
            });

        slice.iter().copied().fold(i32::MIN, i32::max)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximum_energy(energy: Vec<i32>, k: i32) -> i32 {
        Self::maximum_energy(energy, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
