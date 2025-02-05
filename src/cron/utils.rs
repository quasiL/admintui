use crate::config::CONFIG;
use chrono::Utc;
use cron_descriptor::cronparser::cron_expression_descriptor;
use cron_parser::parse;

pub fn get_next_execution(cron_expr: &str) -> String {
    let settings = CONFIG.read().unwrap();
    let timezone = settings.get_timezone();

    let now = Utc::now().with_timezone(&timezone);

    match parse(cron_expr, &now) {
        Ok(next) => format!("{}", next),
        Err(_) => "Invalid cron expression".to_string(),
    }
}

pub fn get_human_readable_cron(cron_expr: &str) -> Result<String, String> {
    let now = Utc::now();

    if parse(cron_expr.trim(), &now).is_err() {
        return Ok("Unable to parse cron expression into human-readable format.".to_string());
    }

    let description = cron_expression_descriptor::get_description_cron(cron_expr.trim());
    Ok(description)
}
