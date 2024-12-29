pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn helper(buses: &[u32], passengers: &[u32], capacity: u32) -> u32 {
        let mut buses_iter = buses.iter().copied();
        let mut passengers_iter = passengers.iter().copied();
        let mut prev_passenger = 0;
        let mut result = 0;

        'outer: while let Some(mut bus) = buses_iter.next() {
            let mut available = capacity;

            loop {
                if let Some(passenger) = passengers_iter.next() {
                    while bus < passenger {
                        if let Some(next_bus) = buses_iter.next() {
                            bus = next_bus;
                            available = capacity;
                        } else {
                            if prev_passenger != bus {
                                result = bus;
                            }

                            break 'outer;
                        }
                    }

                    available -= 1;

                    if prev_passenger != passenger - 1 {
                        result = passenger - 1;
                    }

                    prev_passenger = passenger;

                    if available == 0 {
                        continue 'outer;
                    }
                } else {
                    if let Some(last) = buses_iter.last() {
                        result = last;
                    } else if prev_passenger != bus {
                        result = bus;
                    }

                    break 'outer;
                }
            }
        }

        result
    }

    pub fn latest_time_catch_the_bus(buses: Vec<i32>, passengers: Vec<i32>, capacity: i32) -> i32 {
        let mut buses = buses.into_iter().map(|x| x as u32).collect::<Vec<_>>();
        let mut passengers = passengers.into_iter().map(|x| x as u32).collect::<Vec<_>>();

        buses.sort_unstable();
        passengers.sort_unstable();

        Self::helper(&buses, &passengers, capacity as _) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn latest_time_catch_the_bus(buses: Vec<i32>, passengers: Vec<i32>, capacity: i32) -> i32 {
        Self::latest_time_catch_the_bus(buses, passengers, capacity)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
