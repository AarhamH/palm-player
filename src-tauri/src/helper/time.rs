pub fn seconds_to_minutes(seconds: u64) -> String {
    let minutes: u64 = seconds / 60;
    let seconds: u64 = seconds % 60;
    let res: String = format!("{:02}:{:02}", minutes, seconds);
    res
}

pub fn meta_duration_to_minutes(meta_dur: String) -> String {
    let mut minutes: i32 = 0;
    let mut seconds: i32 = 0;

    let re: regex::Regex = regex::Regex::new(r"PT(?:(\d+)M)?(?:(\d+)S)?").unwrap();
    if let Some(caps) = re.captures(meta_dur.as_str()) {
        // Capture minutes
        if let Some(min) = caps.get(1) {
            minutes = min.as_str().parse().unwrap();
        }
        // Capture seconds
        if let Some(sec) = caps.get(2) {
            seconds = sec.as_str().parse().unwrap();
        }
    }

    format!("{:02}:{:02}", minutes, seconds)
}