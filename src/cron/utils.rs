use crate::config::CONFIG;
use crate::cron::CronJob;
use chrono::Utc;
use cron_descriptor::cronparser::cron_expression_descriptor;
use cron_parser::parse;
use std::io::{self, BufRead, Write};
use std::process::{Command, Stdio};

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

    if parse(cron_expr.trim(), &now).is_err() || cron_expr.contains(',') {
        return Ok("Unable to parse cron expression into human-readable format.".to_string());
    }

    let description = cron_expression_descriptor::get_description_cron(cron_expr.trim());
    Ok(description)
}

pub fn from_crontab() -> Result<Vec<CronJob>, io::Error> {
    let output = Command::new("crontab")
        .arg("-l")
        .stdout(Stdio::piped())
        .output()?;

    if !output.status.success() {
        let stderr_output = String::from_utf8_lossy(&output.stderr);

        if stderr_output.contains("no crontab for") {
            return Ok(vec![CronJob::new({
                CronJob {
                    cron_notation: "User has no crontab".to_string(),
                    job: String::new(),
                    job_description: String::new(),
                    next_execution: String::new(),
                }
            })]);
        }

        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Failed to read crontab",
        ));
    }

    let reader = io::BufReader::new(&output.stdout[..]);
    let mut cron_jobs = Vec::new();
    let mut comment: Option<String> = None;

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        if line.starts_with('#') {
            comment = Some(line.trim_start_matches('#').trim().to_string());
        } else {
            let parts: Vec<&str> = line.split_whitespace().collect();

            if parts.len() < 6 {
                continue;
            }

            let cron_notation = parts[..5].join(" ");
            let job = parts[5..].join(" ");
            let modified_next_execution = get_next_execution(&cron_notation);

            cron_jobs.push(CronJob {
                cron_notation,
                job,
                job_description: comment.take().unwrap_or_else(|| String::new()),
                next_execution: modified_next_execution,
            });
        }
    }

    Ok(cron_jobs)
}

pub fn save_to_crontab(cron_jobs: &[CronJob]) -> io::Result<()> {
    let mut new_crontab = String::new();

    for job in cron_jobs {
        if !job.job.is_empty() {
            if !new_crontab.is_empty() {
                new_crontab.push('\n');
            }
            if !job.job_description.is_empty() {
                new_crontab.push_str(&format!("# {}\n", job.job_description));
            }
            new_crontab.push_str(&format!("{} {}\n", job.cron_notation, job.job));
        }
    }

    let mut process = Command::new("crontab").stdin(Stdio::piped()).spawn()?;

    if let Some(stdin) = process.stdin.as_mut() {
        stdin.write_all(new_crontab.as_bytes())?;
    }

    process.wait()?;
    Ok(())
}
