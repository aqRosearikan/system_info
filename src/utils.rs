const SIZE_UNIT: f64 = 1024.0;
const SECONDS_OF_DAY: u64 = 86400;
const SECONDS_OF_HOUR: u64 = 3600;
const SECONDS_OF_MINUTE: u64 = 60;

pub fn get_size_str(mut val: f64) -> String {
    for suffix in ["KB", "MB", "GB"].iter() {
        if val < SIZE_UNIT {
            return format!("{:.2}{}", val, suffix);
        }
        val /= SIZE_UNIT;
    }
    format!("{:.2}TB", val)
}

pub fn get_time_show(val: u64) -> String {
    let day = val / SECONDS_OF_DAY;
    let hour = val % SECONDS_OF_DAY / SECONDS_OF_HOUR;
    let minute = val % SECONDS_OF_HOUR / SECONDS_OF_MINUTE;
    let second = val % SECONDS_OF_MINUTE;
    format!("{}d {}h {}m {}s", day, hour, minute, second)
}
