// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    // unimplemented!("calculate hourly production rate at speed: {}", speed)

    let theoretical_value:f64 = speed as f64 * 221.0;

    match speed {
        0 => 0.0,
        1..=4 => 1.0 * theoretical_value,
        5..=8 => 0.9 * theoretical_value,
        9..=10 => 0.77 * theoretical_value,
        _ => 0.0,
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    // unimplemented!("calculate the amount of working items at speed: {}", speed)

    return production_rate_per_hour(speed) as u32 / 60
}


