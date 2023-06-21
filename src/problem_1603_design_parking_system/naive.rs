// ------------------------------------------------------ snip ------------------------------------------------------ //

pub struct ParkingSystem {
    slots: [u32; 3],
}

impl ParkingSystem {
    fn new(big: i32, medium: i32, small: i32) -> Self {
        Self {
            slots: [big as _, medium as _, small as _],
        }
    }

    fn add_car(&mut self, car_type: i32) -> bool {
        let slot = &mut self.slots[car_type as u32 as usize - 1];
        let is_available = *slot != 0;

        *slot -= u32::from(is_available);

        is_available
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::ParkingSystem for ParkingSystem {
    fn new(big: i32, medium: i32, small: i32) -> Self {
        Self::new(big, medium, small)
    }

    fn add_car(&mut self, car_type: i32) -> bool {
        self.add_car(car_type)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::ParkingSystem>();
    }
}
