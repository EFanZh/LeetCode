pub mod dfs;

pub trait ThroneInheritance {
    fn new(king_name: String) -> Self;
    fn birth(&mut self, parent_name: String, child_name: String);
    fn death(&mut self, name: String);
    fn get_inheritance_order(&self) -> Vec<String>;
}

#[cfg(test)]
mod tests {
    use super::ThroneInheritance;

    enum Operation {
        Birth(&'static str, &'static str),
        Death(&'static str),
        GetInheritanceOrder(&'static [&'static str]),
    }

    pub fn run<T: ThroneInheritance>() {
        let test_cases = [(
            "king",
            &[
                Operation::Birth("king", "andy"),
                Operation::Birth("king", "bob"),
                Operation::Birth("king", "catherine"),
                Operation::Birth("andy", "matthew"),
                Operation::Birth("bob", "alex"),
                Operation::Birth("bob", "asha"),
                Operation::GetInheritanceOrder(&["king", "andy", "matthew", "bob", "alex", "asha", "catherine"]),
                Operation::Death("bob"),
                Operation::GetInheritanceOrder(&["king", "andy", "matthew", "alex", "asha", "catherine"]),
            ],
        )];

        for (king_name, operations) in test_cases {
            let mut throne_inheritance = T::new(king_name.to_string());

            for operation in operations {
                match *operation {
                    Operation::Birth(parent_name, child_name) => {
                        throne_inheritance.birth(parent_name.to_string(), child_name.to_string())
                    }
                    Operation::Death(name) => throne_inheritance.death(name.to_string()),
                    Operation::GetInheritanceOrder(expected) => {
                        assert_eq!(throne_inheritance.get_inheritance_order(), expected);
                    }
                }
            }
        }
    }
}
