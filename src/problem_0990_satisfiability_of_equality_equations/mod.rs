pub mod bfs;

pub trait Solution {
    fn equations_possible(equations: Vec<String>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&["a==b", "b!=a"] as &[_], false),
            (&["b==a", "a==b"], true),
            (&["a!=a"], false),
            (&["c==c", "b==d", "x!=z"], true),
        ];

        for (equations, expected) in test_cases {
            assert_eq!(
                S::equations_possible(equations.iter().copied().map(str::to_string).collect()),
                expected
            );
        }
    }
}
