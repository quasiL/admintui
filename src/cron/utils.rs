use chrono::{Duration, Utc};
use cron::Schedule;
use std::str::FromStr;

pub fn get_next_execution(cron_expr: &str) -> String {
    let formatted_expr = format!("* {}", cron_expr);
    let schedule = Schedule::from_str(&formatted_expr).ok();

    if let Some(schedule) = schedule {
        let now = Utc::now() + Duration::seconds(1);
        if let Some(next_time) = schedule.upcoming(Utc).find(|&t| t > now) {
            return next_time.to_string();
        }
    }

    "N/A".to_string()
}
