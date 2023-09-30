pub mod lazy;
pub mod mod_inverse;

pub trait Fancy {
    fn new() -> Self;
    fn append(&mut self, val: i32);
    fn add_all(&mut self, inc: i32);
    fn mult_all(&mut self, m: i32);
    fn get_index(&mut self, idx: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Fancy;

    enum Operation {
        Append(i32),
        AddAll(i32),
        MultAll(i32),
        GetIndex(i32, i32),
    }

    pub fn run<F: Fancy>() {
        let test_cases = [
            &[
                Operation::Append(2),
                Operation::AddAll(3),
                Operation::Append(7),
                Operation::MultAll(2),
                Operation::GetIndex(0, 10),
                Operation::AddAll(3),
                Operation::Append(10),
                Operation::MultAll(2),
                Operation::GetIndex(0, 26),
                Operation::GetIndex(1, 34),
                Operation::GetIndex(2, 20),
            ] as &[_],
            &[Operation::AddAll(1), Operation::GetIndex(0, -1)],
            &[
                Operation::Append(2),
                Operation::AddAll(100),
                Operation::MultAll(100),
                Operation::MultAll(100),
                Operation::MultAll(100),
                Operation::MultAll(10),
                Operation::AddAll(9),
                Operation::GetIndex(0, 20_000_002),
            ],
        ];

        for operations in test_cases {
            let mut fancy = F::new();

            for operation in operations {
                match *operation {
                    Operation::Append(val) => fancy.append(val),
                    Operation::AddAll(inc) => fancy.add_all(inc),
                    Operation::MultAll(m) => fancy.mult_all(m),
                    Operation::GetIndex(idx, expected) => assert_eq!(fancy.get_index(idx), expected),
                }
            }
        }
    }
}
