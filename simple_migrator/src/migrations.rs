use anyhow::Result;
use chrono::NaiveDateTime;

pub trait MigrationTrait {
    fn get_date(&self) -> Result<NaiveDateTime>;
    fn get_id(&self) -> String;
    fn get_name(&self) -> String;
    fn get_description(&self) -> String;
    fn up(&self) -> Result<()>;
    fn down(&self) -> Result<()>;
}
