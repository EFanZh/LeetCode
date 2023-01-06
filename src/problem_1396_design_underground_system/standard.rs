// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;

struct State {
    total_time: u64,
    total_count: u32,
}

pub struct UndergroundSystem {
    records: HashMap<([u8; 10], [u8; 10]), State>,
    ongoing: HashMap<i32, ([u8; 10], u32)>,
}

impl UndergroundSystem {
    fn new() -> Self {
        Self {
            records: HashMap::new(),
            ongoing: HashMap::new(),
        }
    }

    fn get_station_name(s: String) -> [u8; 10] {
        let mut result = [0; 10];

        for (target, c) in result.iter_mut().zip(s.into_bytes()) {
            *target = c;
        }

        result
    }

    fn check_in(&mut self, id: i32, station_name: String, t: i32) {
        let station_name = Self::get_station_name(station_name);
        let t = t as _;

        self.ongoing.insert(id, (station_name, t));
    }

    fn check_out(&mut self, id: i32, station_name: String, t: i32) {
        let end_station = Self::get_station_name(station_name);
        let t = t as u32;
        let (start_station, start_time) = self.ongoing.remove(&id).unwrap();
        let duration = u64::from(t - start_time);

        self.records
            .entry((start_station, end_station))
            .and_modify(|state| {
                state.total_time += duration;
                state.total_count += 1;
            })
            .or_insert_with(|| State {
                total_time: duration,
                total_count: 1,
            });
    }

    #[allow(clippy::cast_precision_loss)]
    fn get_average_time(&self, start_station: String, end_station: String) -> f64 {
        let start_station = Self::get_station_name(start_station);
        let end_station = Self::get_station_name(end_station);
        let state = &self.records[&(start_station, end_station)];

        (state.total_time as f64) / f64::from(state.total_count)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::UndergroundSystem for UndergroundSystem {
    fn new() -> Self {
        Self::new()
    }

    fn check_in(&mut self, id: i32, station_name: String, t: i32) {
        self.check_in(id, station_name, t);
    }

    fn check_out(&mut self, id: i32, station_name: String, t: i32) {
        self.check_out(id, station_name, t);
    }

    fn get_average_time(&self, start_station: String, end_station: String) -> f64 {
        self.get_average_time(start_station, end_station)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::UndergroundSystem>();
    }
}
