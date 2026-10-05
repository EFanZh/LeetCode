// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::{HashMap, VecDeque};

pub struct RideSharingSystem {
    drivers: VecDeque<i32>,
    riders: VecDeque<i32>,
    riders_offset: u32,
    rider_id_to_index: HashMap<i32, u32>,
}

impl RideSharingSystem {
    fn new() -> Self {
        Self {
            drivers: VecDeque::new(),
            riders: VecDeque::new(),
            riders_offset: 0,
            rider_id_to_index: HashMap::new(),
        }
    }

    fn add_rider(&mut self, rider_id: i32) {
        let index = self.riders_offset + self.riders.len() as u32;

        self.riders.push_back(rider_id);
        self.rider_id_to_index.insert(rider_id, index);
    }

    fn add_driver(&mut self, driver_id: i32) {
        self.drivers.push_back(driver_id);
    }

    fn clean_removed_riders(&mut self) {
        while self.riders.front().is_some_and(|&rider_id| rider_id == 0) {
            self.riders.pop_front();
            self.riders_offset += 1;
        }
    }

    fn match_driver_with_rider(&mut self) -> Vec<i32> {
        if !self.drivers.is_empty()
            && let Some(rider_id) = self.riders.pop_front()
        {
            let driver_id = self.drivers.pop_front().unwrap();

            self.riders_offset += 1;
            self.rider_id_to_index.remove(&rider_id);
            self.clean_removed_riders();

            [driver_id, rider_id]
        } else {
            [-1, -1]
        }
        .into()
    }

    fn cancel_rider(&mut self, rider_id: i32) {
        if let Some(index) = self.rider_id_to_index.remove(&rider_id) {
            self.riders[(index - self.riders_offset) as usize] = 0;
            self.clean_removed_riders();
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::RideSharingSystem for RideSharingSystem {
    fn new() -> Self {
        Self::new()
    }

    fn add_rider(&mut self, rider_id: i32) {
        self.add_rider(rider_id);
    }

    fn add_driver(&mut self, driver_id: i32) {
        self.add_driver(driver_id);
    }

    fn match_driver_with_rider(&mut self) -> Vec<i32> {
        self.match_driver_with_rider()
    }

    fn cancel_rider(&mut self, rider_id: i32) {
        self.cancel_rider(rider_id);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::RideSharingSystem>();
    }
}
