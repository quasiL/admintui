use chrono::{Duration, Utc};
use cron::Schedule;
use regex::Regex;
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

pub fn convert_to_human_readable(input: &str) -> String {
    let cron_regex = Regex::new(r"^(\S+)\s+(\S+)\s+(\S+)\s+(\S+)\s+(\S+)$").unwrap();

    if let Some(captures) = cron_regex.captures(input.trim()) {
        let minute = &captures[1];
        let hour = &captures[2];
        let day_of_month = &captures[3];
        let month = &captures[4];
        let day_of_week = &captures[5];

        let minute_str = parse_minute(minute);
        let hour_str = parse_hour(hour);
        let day_of_month_str = parse_day_of_month(day_of_month);
        let month_str = parse_month(month);
        let day_of_week_str = parse_day_of_week(day_of_week);

        return format!(
            "At {} past {} on {} {} {}",
            minute_str, hour_str, day_of_month_str, month_str, day_of_week_str
        );
    }

    "Unable to parse cron expression into human-readable format.".to_string()
}

fn parse_minute(minute: &str) -> String {
    if minute == "*" {
        "every minute".to_string()
    } else if minute.contains("/") {
        format!(
            "every {} minute(s) starting at {}",
            &minute[2..],
            &minute[0..1]
        )
    } else {
        format!("minute {}", minute)
    }
}

fn parse_hour(hour: &str) -> String {
    if hour == "*" {
        "every hour".to_string()
    } else if hour.contains("/") {
        format!("every {} hour(s) starting at {}", &hour[2..], &hour[0..1])
    } else {
        format!("hour {}", hour)
    }
}

fn parse_day_of_month(day: &str) -> String {
    if day == "*" {
        "every day".to_string()
    } else if day.contains("/") {
        format!("every {} day(s) starting at day {}", &day[2..], &day[0..1])
    } else {
        format!("day-of-month {}", day)
    }
}

fn parse_month(month: &str) -> String {
    if month == "*" {
        "every month".to_string()
    } else if month.contains("/") {
        format!(
            "every {} month(s) starting at month {}",
            &month[2..],
            &month[0..1]
        )
    } else {
        format!("month {}", month)
    }
}

fn parse_day_of_week(day: &str) -> String {
    match day {
        "*" => "every day of the week".to_string(),
        "0" | "7" => "Sunday".to_string(),
        "1" => "Monday".to_string(),
        "2" => "Tuesday".to_string(),
        "3" => "Wednesday".to_string(),
        "4" => "Thursday".to_string(),
        "5" => "Friday".to_string(),
        "6" => "Saturday".to_string(),
        _ if day.contains("-") => {
            let parts: Vec<&str> = day.split('-').collect();
            format!(
                "every day-of-week from {} through {}",
                day_name(parts[0]),
                day_name(parts[1])
            )
        }
        _ if day.contains(",") => {
            let days: Vec<String> = day.split(',').map(day_name).collect();
            format!("{}", days.join(", "))
        }
        _ => format!("day-of-week {}", day),
    }
}

fn day_name(day: &str) -> String {
    match day {
        "0" | "7" => "Sunday".to_string(),
        "1" => "Monday".to_string(),
        "2" => "Tuesday".to_string(),
        "3" => "Wednesday".to_string(),
        "4" => "Thursday".to_string(),
        "5" => "Friday".to_string(),
        "6" => "Saturday".to_string(),
        _ => day.to_string(),
    }
}
