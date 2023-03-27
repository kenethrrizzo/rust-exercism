pub fn production_rate_per_hour(speed: u8) -> f64 {
    let rate = (221 * speed as u16) as f64;

    match speed {
        (5..=8) => rate * 0.9,
        (9..=10) => rate * 0.77,
        _ => rate,
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.0) as u32
}
