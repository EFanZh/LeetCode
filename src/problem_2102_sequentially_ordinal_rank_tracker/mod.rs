pub mod binary_heap;

pub trait SORTracker {
    fn new() -> Self;
    fn add(&mut self, name: String, score: i32);
    fn get(&mut self) -> String;
}

#[cfg(test)]
mod tests {
    use super::SORTracker;

    enum Operation {
        Add(&'static str, i32),
        Get(&'static str),
    }

    pub fn run<T: SORTracker>() {
        let test_cases = [&[
            Operation::Add("bradford", 2),
            Operation::Add("branford", 3),
            Operation::Get("branford"),
            Operation::Add("alps", 2),
            Operation::Get("alps"),
            Operation::Add("orland", 2),
            Operation::Get("bradford"),
            Operation::Add("orlando", 3),
            Operation::Get("bradford"),
            Operation::Add("alpine", 2),
            Operation::Get("bradford"),
            Operation::Get("orland"),
        ] as &[_]];

        for operations in test_cases {
            let mut sor_tracker = T::new();

            for operation in operations {
                match *operation {
                    Operation::Add(name, score) => sor_tracker.add(name.to_string(), score),
                    Operation::Get(expected) => assert_eq!(sor_tracker.get(), expected),
                }
            }
        }
    }
}
