pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::iter;

impl Solution {
    pub fn min_number_of_hours(
        initial_energy: i32,
        initial_experience: i32,
        energy: Vec<i32>,
        experience: Vec<i32>,
    ) -> i32 {
        let mut current_energy = initial_energy as u32;
        let mut current_experience = initial_experience as u32;
        let mut result = 0;

        iter::zip(&energy, &experience).for_each(|(&required_energy, &required_experience)| {
            let (required_energy, required_experience) = (required_energy as u32, required_experience as u32);

            if current_energy <= required_energy {
                result += required_energy + 1 - current_energy;
                current_energy = 1;
            } else {
                current_energy -= required_energy;
            }

            if current_experience <= required_experience {
                result += required_experience + 1 - current_experience;
                current_experience = required_experience + 1;
            }

            current_experience += required_experience;
        });

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_number_of_hours(
        initial_energy: i32,
        initial_experience: i32,
        energy: Vec<i32>,
        experience: Vec<i32>,
    ) -> i32 {
        Self::min_number_of_hours(initial_energy, initial_experience, energy, experience)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
