pub mod hash_map;

#[expect(clippy::upper_case_acronyms, reason = "required")]
pub trait SQL {
    fn new(names: Vec<String>, columns: Vec<i32>) -> Self;
    fn ins(&mut self, name: String, row: Vec<String>) -> bool;
    fn rmv(&mut self, name: String, row_id: i32);
    fn sel(&self, name: String, row_id: i32, column_id: i32) -> String;
    fn exp(&self, name: String) -> Vec<String>;
}

#[cfg(test)]
mod tests {
    use super::SQL;

    enum Operation {
        Ins((&'static str, &'static [&'static str]), bool),
        Rmv(&'static str, i32),
        Sel((&'static str, i32, i32), &'static str),
        Exp(&'static str, &'static [&'static str]),
    }

    pub fn run<S: SQL>() {
        let test_cases = [
            (
                (&["one", "two", "three"] as &[_], &[2, 3, 1] as &[_]),
                &[
                    Operation::Ins(("two", &["first", "second", "third"]), true),
                    Operation::Sel(("two", 1, 3), "third"),
                    Operation::Ins(("two", &["fourth", "fifth", "sixth"]), true),
                    Operation::Exp("two", &["1,first,second,third", "2,fourth,fifth,sixth"]),
                    Operation::Rmv("two", 1),
                    Operation::Sel(("two", 2, 2), "fifth"),
                    Operation::Exp("two", &["2,fourth,fifth,sixth"]),
                ] as &[_],
            ),
            (
                (&["one", "two", "three"], &[2, 3, 1]),
                &[
                    Operation::Ins(("two", &["first", "second", "third"]), true),
                    Operation::Sel(("two", 1, 3), "third"),
                    Operation::Rmv("two", 1),
                    Operation::Sel(("two", 1, 2), "<null>"),
                    Operation::Ins(("two", &["fourth", "fifth"]), false),
                    Operation::Ins(("two", &["fourth", "fifth", "sixth"]), true),
                ],
            ),
        ];

        for ((names, columns), operations) in test_cases {
            let mut sql = S::new(names.iter().copied().map(str::to_string).collect(), columns.to_vec());

            for operation in operations {
                match *operation {
                    Operation::Ins((name, row), expected) => assert_eq!(
                        sql.ins(name.to_string(), row.iter().copied().map(str::to_string).collect()),
                        expected,
                    ),
                    Operation::Rmv(name, row_id) => sql.rmv(name.to_string(), row_id),
                    Operation::Sel((name, row_id, column_id), expected) => {
                        assert_eq!(sql.sel(name.to_string(), row_id, column_id), expected);
                    }
                    Operation::Exp(name, expected) => assert_eq!(sql.exp(name.to_string()), expected),
                }
            }
        }
    }
}
