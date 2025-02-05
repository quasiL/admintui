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
    fn set_timezone(&mut self, timezone_str: &str) -> Result<(), String> {
        match Tz::from_str(timezone_str) {
            Ok(tz) => {
                self.timezone = tz;
                Ok(())
            }
            Err(_) => Err(format!("Invalid timezone: {}", timezone_str)),
        }
    }

    pub fn get_timezone(&self) -> Tz {
        self.timezone.clone()
    }

    pub fn change_timezone(new_timezone: &str) -> Result<(), String> {
        let mut settings = CONFIG.write().unwrap();
        settings.set_timezone(new_timezone)
    }
}
