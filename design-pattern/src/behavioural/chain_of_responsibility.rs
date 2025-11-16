use std::fmt::Error;

pub struct Object<'a> {
    pub name: &'a str,
}

pub trait HandlerTrait {
    fn handle(&self, object: &Object) -> Result<(), Error>;
    fn name(&self) -> &str;
}

pub mod medical_handler {

    use super::*;

    pub struct Doctor;
    pub struct Nurse;
    pub struct Casier;

    impl HandlerTrait for Doctor {
        fn handle(&self, object: &Object) -> Result<(), Error> {
            println!(
                "{} is handled By {}, it seem object in flu",
                &object.name,
                self.name()
            );
            Ok(())
        }

        fn name(&self) -> &str {
            "Doctor"
        }
    }

    impl HandlerTrait for Nurse {
        fn handle(&self, object: &Object) -> Result<(), Error> {
            println!(
                "{} is handled By {}, move patient to room 101",
                &object.name,
                self.name()
            );
            Ok(())
        }

        fn name(&self) -> &str {
            "Nurse"
        }
    }

    impl HandlerTrait for Casier {
        fn handle(&self, object: &Object) -> Result<(), Error> {
            println!(
                "{} is handled By {}, patient charged for 1000 usd",
                &object.name,
                self.name()
            );
            Ok(())
        }

        fn name(&self) -> &str {
            "Casier"
        }
    }
}

pub struct HandlerExecutor {
    handlers: Vec<Box<dyn HandlerTrait>>,
}

impl HandlerExecutor {
    pub fn new(handlers: Vec<Box<dyn HandlerTrait>>) -> Self {
        Self { handlers }
    }

    pub fn handle(&self, object: &Object) {
        for h in &self.handlers {
            h.handle(object).expect(&format!(
                "error when handling Object in {} handler",
                h.name()
            ))
        }
    }
}

pub struct HandlerBuilder {
    handlers: Vec<Box<dyn HandlerTrait>>,
}

impl HandlerBuilder {
    pub fn new() -> Self {
        Self {
            handlers: Vec::new(),
        }
    }

    pub fn add_handler(mut self, handler: impl HandlerTrait + Sized + 'static) -> Self {
        self.handlers.push(Box::new(handler));
        self
    }

    pub fn build(self) -> HandlerExecutor {
        HandlerExecutor::new(self.handlers)
    }
}

