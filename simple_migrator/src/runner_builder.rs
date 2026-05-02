use crate::migrations::MigrationTrait;
use crate::{executor::ExecutorTrait, runner::Runner};
use anyhow::{Ok, Result, anyhow};

pub struct RunnerBuilder<EX>
where
    EX: ExecutorTrait,
{
    executor: Option<EX>,
    migrations: Vec<Box<dyn MigrationTrait>>,
}

impl<EX> RunnerBuilder<EX>
where
    EX: ExecutorTrait,
{
    pub fn create() -> Self {
        Self {
            executor: None,
            migrations: Vec::new(),
        }
    }

    pub fn with_executor(mut self, executor: EX) -> Self {
        self.executor = Some(executor);
        self
    }

    pub fn add_migration<M: MigrationTrait + 'static>(mut self, migration: M) -> Self {
        self.migrations.push(Box::new(migration));
        self
    }

    pub fn add_migrations<M: MigrationTrait + 'static>(mut self, migrations: Vec<M>) -> Self {
        let list_box_migration = migrations
            .into_iter()
            .map(|mig| Box::new(mig) as Box<dyn MigrationTrait + 'static>);

        self.migrations.extend(list_box_migration);
        self
    }

    pub fn build(self) -> Result<Runner<EX>> {
        let executor = self
            .executor
            .ok_or_else(|| anyhow!("executor cant be empty!!"))?;

        if self.migrations.is_empty() {
            return Err(anyhow!("please add at least one migration"));
        }

        Ok(Runner::create(executor, self.migrations))
    }
}
