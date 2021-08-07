pub mod right_to_left_recursive_descent;

pub trait Solution {
    fn count_of_atoms(formula: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("H2O", "H2O"),
            ("Mg(OH)2", "H2MgO2"),
            ("K4(ON(SO3)2)2", "K4N2O14S4"),
            ("Be32", "Be32"),
        ];

        for (formula, expected) in test_cases {
            assert_eq!(S::count_of_atoms(formula.to_string()), expected);
        }
    }
}
