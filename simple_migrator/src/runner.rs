use crate::migrations::MigrationTrait;
use crate::{executor::ExecutorTrait, models::MigrationStatus};
use anyhow::{Ok, Result};
use std::cmp::Reverse;

pub trait RunnerTrait {
    fn up(&mut self) -> Result<()>;
    fn down(&mut self) -> Result<()>;
    fn get_applied(&mut self) -> Result<Vec<MigrationStatus>>;
    fn get_unapplied(&mut self) -> Result<Vec<MigrationStatus>>;

    fn up_to(&mut self, migration_id: String) -> Result<()>;
    fn down_to(&mut self, migration_id: String) -> Result<()>;

    fn ensure_migration_table(&mut self) -> Result<()>;
}

pub struct Runner<EX>
where
    EX: ExecutorTrait,
{
    executor: EX,
    migrations: Vec<Box<dyn MigrationTrait>>,
}

impl<EX> Runner<EX>
where
    EX: ExecutorTrait,
{
    pub fn create(executor: EX, migrations: Vec<Box<dyn MigrationTrait>>) -> Self {
        Self {
            executor,
            migrations,
        }
    }
}

impl<EX> RunnerTrait for Runner<EX>
where
    EX: ExecutorTrait,
{
    fn up(&mut self) -> Result<()> {
        self.ensure_migration_table()?;
        let unapplied_migrations: Vec<String> = self
            .get_unapplied()?
            .iter()
            .map(|status| status.migration_id.clone())
            .collect();

        self.migrations
            .sort_by_key(|m| m.get_date().expect("fail to parse date time"));

        for migration in &self.migrations {
            if unapplied_migrations.contains(&migration.get_id()) {
                migration.up()?;
            }
        }

        Ok(())
    }

    fn down(&mut self) -> Result<()> {
        self.ensure_migration_table()?;
        let appliead_migrations: Vec<String> = self
            .get_applied()?
            .iter()
            .map(|status| status.migration_id.clone())
            .collect();

        self.migrations
            .sort_by_key(|m| Reverse(m.get_date().expect("fail to parse date time")));

        for migration in &self.migrations {
            if appliead_migrations.contains(&migration.get_id()) {
                migration.down()?;
            }
        }

        Ok(())
    }

    fn get_applied(&mut self) -> Result<Vec<MigrationStatus>> {
        self.executor.get_applied()
    }

    fn get_unapplied(&mut self) -> Result<Vec<MigrationStatus>> {
        let applied = self.executor.get_applied()?;
        let migration_ids: Vec<String> = self.migrations.iter().map(|mig| mig.get_id()).collect();

        let unapplied_ids: Vec<String> = applied
            .iter()
            .filter(|mig| !migration_ids.contains(&mig.migration_id))
            .map(|mig| mig.migration_id.clone())
            .collect();

        let unapplied_migration: Vec<MigrationStatus> = self
            .migrations
            .iter()
            .filter(|mig| unapplied_ids.contains(&mig.get_id()))
            .map(|mig| MigrationStatus {
                migration_id: mig.get_id(),
                name: Some(mig.get_name()),
                description: Some(mig.get_description()),
                is_applied: false,
                applied_date: None,
            })
            .collect();

        Ok(unapplied_migration)
    }

    fn up_to(&mut self, migration_id: String) -> Result<()> {
        self.ensure_migration_table()?;
        let applied_migrations: Vec<String> = self
            .get_applied()?
            .iter()
            .map(|status| status.migration_id.clone())
            .collect();

        self.migrations
            .sort_by_key(|m| m.get_date().expect("fail to parse date time"));

        for migration in &self.migrations {
            if applied_migrations.contains(&migration.get_id())
                && migration.get_id() != migration_id
            {
                continue;
            }

            if migration.get_id() == migration_id {
                break;
            }
            migration.up()?;
        }

        Ok(())
    }

    fn down_to(&mut self, migration_id: String) -> Result<()> {
        self.ensure_migration_table()?;
        let unapplied_migrations: Vec<String> = self
            .get_unapplied()?
            .iter()
            .map(|status| status.migration_id.clone())
            .collect();

        self.migrations
            .sort_by_key(|m| Reverse(m.get_date().expect("fail to parse date time")));

        for migration in &self.migrations {
            if unapplied_migrations.contains(&migration.get_id())
                && migration.get_id() != migration_id
            {
                continue;
            }

            if migration.get_id() == migration_id {
                break;
            }
            migration.down()?;
        }

        Ok(())
    }

    fn ensure_migration_table(&mut self) -> Result<()> {
        self.executor.ensure_migration_table()
    }
}
