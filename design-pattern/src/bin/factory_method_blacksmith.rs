use design_pattern::factory_method_blacksmith::{
    blacksmith::{Blacksmith, WeaponType},
    *,
};

fn main() {
    let elf_blacksmith = elf_blacksmith::ElfBlacksmith::default();
    let orch_blacksmith = orch_blacksmith::OrchBlacksmith::default();

    let orch_axes = orch_blacksmith.manufacture_weapon(WeaponType::Axe);
    let elf_axes = elf_blacksmith.manufacture_weapon(WeaponType::Spear);

    orch_axes.info();
    elf_axes.info();
}
