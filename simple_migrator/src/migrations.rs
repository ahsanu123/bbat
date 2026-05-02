use anyhow::Result;
use chrono::{DateTime, Utc};

pub trait MigrationTrait {
    fn get_date(&self) -> Result<DateTime<Utc>>;
    fn get_id(&self) -> String;
    fn get_name(&self) -> String;
    fn get_description(&self) -> String;
    fn up(&self) -> Result<()>;
    fn down(&self) -> Result<()>;
}
