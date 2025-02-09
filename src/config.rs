use chrono_tz::{Europe::Prague, Tz};
use std::str::FromStr;
use std::sync::RwLock;

pub static CONFIG: RwLock<Config> = RwLock::new(Config { timezone: Prague });

#[derive(Clone, Debug)]
pub struct Config {
    timezone: Tz,
}

#[allow(dead_code)]
impl Config {
    pub fn get_timezone(&self) -> Tz {
        self.timezone.clone()
    }

    pub fn change_timezone(new_timezone: &str) -> Result<(), String> {
        let mut settings = CONFIG.write().unwrap();
        match Tz::from_str(new_timezone) {
            Ok(tz) => {
                settings.timezone = tz;
                Ok(())
            }
            Err(_) => Err(format!("Invalid timezone: {}", new_timezone)),
        }
    }
}
