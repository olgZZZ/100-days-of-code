// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
// #![allow(unused)]

const CARS_EACH_HOUR: u32 = 221;

pub fn production_rate_per_hour(speed: u8) -> f64 {

    let rate = (CARS_EACH_HOUR as f64) * (speed as f64);

    if speed >= 1 && speed <= 4 {
        return rate;
    } else if speed >= 5 && speed <= 8 {
        return rate * 0.9;
    } else if speed >= 9 && speed <= 10 {
        return rate * 0.77;
    } else {
        return 0.0;
    }

}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60f64) as u32
}
