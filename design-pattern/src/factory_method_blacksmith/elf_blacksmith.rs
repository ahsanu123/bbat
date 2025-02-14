use super::blacksmith::{Blacksmith, Weapon, WeaponType};

#[derive(Default)]
pub struct ElfBlacksmith {}

impl Blacksmith for ElfBlacksmith {
    fn manufacture_weapon(&self, weapon_type: WeaponType) -> Weapon {
        match weapon_type {
            WeaponType::Spear => Weapon {
                name: "Elf Spear".into(),
                damage: 100,
            },
            WeaponType::Axe => Weapon {
                name: "Elf Axe".into(),
                damage: 110,
            },
            WeaponType::Arrow => Weapon {
                name: "Elf Arrow".into(),
                damage: 80,
            },
        }
    }
}
