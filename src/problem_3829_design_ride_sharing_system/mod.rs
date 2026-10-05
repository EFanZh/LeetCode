pub mod deque_and_hashmap;

pub trait RideSharingSystem {
    fn new() -> Self;
    fn add_rider(&mut self, rider_id: i32);
    fn add_driver(&mut self, driver_id: i32);
    fn match_driver_with_rider(&mut self) -> Vec<i32>;
    fn cancel_rider(&mut self, rider_id: i32);
}

#[cfg(test)]
mod tests {
    use super::RideSharingSystem;

    enum Operation {
        AddRider(i32),
        AddDriver(i32),
        MatchDriverWithRider([i32; 2]),
        CancelRider(i32),
    }

    pub fn run<R: RideSharingSystem>() {
        let test_cases = [
            &[
                Operation::AddRider(3),
                Operation::AddDriver(2),
                Operation::AddRider(1),
                Operation::MatchDriverWithRider([2, 3]),
                Operation::AddDriver(5),
                Operation::CancelRider(3),
                Operation::MatchDriverWithRider([5, 1]),
                Operation::MatchDriverWithRider([-1, -1]),
            ] as &[_],
            &[
                Operation::AddRider(8),
                Operation::AddDriver(8),
                Operation::AddDriver(6),
                Operation::MatchDriverWithRider([8, 8]),
                Operation::AddRider(2),
                Operation::CancelRider(2),
                Operation::MatchDriverWithRider([-1, -1]),
            ],
            &[
                Operation::AddDriver(423),
                Operation::MatchDriverWithRider([-1, -1]),
                Operation::AddRider(310),
                Operation::AddRider(336),
                Operation::MatchDriverWithRider([423, 310]),
                Operation::CancelRider(336),
                Operation::AddDriver(970),
                Operation::AddDriver(821),
            ],
            &[
                Operation::AddDriver(846),
                Operation::AddRider(186),
                Operation::AddRider(253),
                Operation::CancelRider(253),
                Operation::AddRider(532),
                Operation::AddDriver(249),
                Operation::MatchDriverWithRider([846, 186]),
                Operation::AddDriver(506),
                Operation::CancelRider(532),
                Operation::CancelRider(186),
                Operation::AddDriver(692),
                Operation::MatchDriverWithRider([-1, -1]),
                Operation::AddDriver(211),
                Operation::MatchDriverWithRider([-1, -1]),
                Operation::CancelRider(431),
                Operation::AddDriver(333),
                Operation::AddRider(21),
                Operation::CancelRider(21),
            ],
        ];

        for operations in test_cases {
            let mut ride_sharing_system = R::new();

            for operation in operations {
                match *operation {
                    Operation::AddRider(rider_id) => ride_sharing_system.add_rider(rider_id),
                    Operation::AddDriver(driver_id) => ride_sharing_system.add_driver(driver_id),
                    Operation::MatchDriverWithRider(expected) => {
                        assert_eq!(ride_sharing_system.match_driver_with_rider(), expected);
                    }
                    Operation::CancelRider(rider_id) => ride_sharing_system.cancel_rider(rider_id),
                }
            }
        }
    }
}
