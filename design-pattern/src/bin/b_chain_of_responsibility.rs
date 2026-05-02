use design_pattern::behavioural::chain_of_responsibility::*;

fn main() {
    let patient = Object { name: "john" };
    let doctor = medical_handler::Doctor;
    let nurse = medical_handler::Nurse;
    let casier = medical_handler::Casier;

    let executor = HandlerBuilder::new()
        .add_handler(doctor)
        .add_handler(nurse)
        .add_handler(casier)
        .build();

    executor.handle(&patient);
}
