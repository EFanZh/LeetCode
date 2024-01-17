pub mod modular_arithmetic;

pub trait Robot {
    fn new(width: i32, height: i32) -> Self;
    fn step(&mut self, num: i32);
    fn get_pos(&self) -> Vec<i32>;
    fn get_dir(&self) -> String;
}

#[cfg(test)]
mod tests {
    use super::Robot;

    enum Operation {
        Step(i32),
        GetPos([i32; 2]),
        GetDir(&'static str),
    }

    const LONG_TEST_CASE: ((i32, i32), &[Operation]) = (
        (20, 14),
        &[
            Operation::Step(32),
            Operation::Step(18),
            Operation::Step(18),
            Operation::GetPos([4, 0]),
            Operation::GetDir("East"),
            Operation::Step(18),
            Operation::GetPos([19, 3]),
            Operation::GetDir("North"),
            Operation::Step(45),
            Operation::Step(37),
            Operation::Step(39),
            Operation::GetPos([15, 0]),
            Operation::GetDir("East"),
            Operation::Step(8),
            Operation::Step(11),
            Operation::Step(18),
            Operation::GetPos([0, 12]),
            Operation::GetDir("South"),
            Operation::Step(3),
            Operation::Step(39),
            Operation::Step(7),
            Operation::Step(31),
            Operation::Step(42),
            Operation::GetPos([5, 13]),
            Operation::GetDir("West"),
            Operation::Step(35),
            Operation::Step(11),
            Operation::Step(36),
            Operation::Step(29),
            Operation::Step(10),
            Operation::GetPos([12, 13]),
            Operation::GetDir("West"),
            Operation::Step(49),
            Operation::Step(31),
            Operation::GetPos([0, 9]),
            Operation::GetDir("South"),
            Operation::Step(31),
            Operation::Step(47),
            Operation::GetPos([5, 0]),
            Operation::GetDir("East"),
            Operation::Step(29),
            Operation::Step(1),
            Operation::GetPos([16, 13]),
            Operation::GetDir("West"),
            Operation::Step(5),
            Operation::Step(44),
        ],
    );

    pub fn run<R: Robot>() {
        let test_cases = [
            (
                (6, 3),
                &[
                    Operation::Step(2),
                    Operation::Step(2),
                    Operation::GetPos([4, 0]),
                    Operation::GetDir("East"),
                    Operation::Step(2),
                    Operation::Step(1),
                    Operation::Step(4),
                    Operation::GetPos([1, 2]),
                    Operation::GetDir("West"),
                ] as &[_],
            ),
            (
                (8, 2),
                &[
                    Operation::Step(17),
                    Operation::GetPos([1, 0]),
                    Operation::GetDir("East"),
                    Operation::Step(21),
                    Operation::GetPos([6, 0]),
                    Operation::GetDir("East"),
                    Operation::Step(22),
                    Operation::Step(34),
                    Operation::GetPos([1, 1]),
                    Operation::GetDir("West"),
                    Operation::Step(1),
                    Operation::Step(46),
                    Operation::Step(35),
                    Operation::GetPos([0, 0]),
                    Operation::GetDir("South"),
                    Operation::Step(44),
                    Operation::Step(14),
                    Operation::Step(31),
                    Operation::GetPos([6, 1]),
                    Operation::GetDir("West"),
                    Operation::Step(50),
                ],
            ),
            ((97, 98), &[Operation::GetPos([0, 0]), Operation::GetDir("East")]),
            LONG_TEST_CASE,
        ];

        for ((width, height), operations) in test_cases {
            let mut robot = R::new(width, height);

            for operation in operations {
                match *operation {
                    Operation::Step(num) => robot.step(num),
                    Operation::GetPos(expected) => assert_eq!(robot.get_pos(), expected),
                    Operation::GetDir(expected) => assert_eq!(robot.get_dir(), expected),
                }
            }
        }
    }
}
