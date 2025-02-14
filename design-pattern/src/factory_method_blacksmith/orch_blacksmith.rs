use super::blacksmith::{Blacksmith, Weapon, WeaponType};

#[derive(Default)]
pub struct OrchBlacksmith {}

impl Blacksmith for OrchBlacksmith {
    fn manufacture_weapon(&self, weapon_type: WeaponType) -> Weapon {
        match weapon_type {
            WeaponType::Spear => Weapon {
                name: "Orch Spear".into(),
                damage: 110,
            },
            WeaponType::Axe => Weapon {
                name: "Orch Axe".into(),
                damage: 140,
            },
            WeaponType::Arrow => Weapon {
                name: "Orch Arrow".into(),
                damage: 40,
            },
        }
    }
}
