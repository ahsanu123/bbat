pub enum WeaponType {
    Spear,
    Axe,
    Arrow,
}

pub struct Weapon {
    pub name: String,
    pub damage: u32,
}

impl Weapon {
    pub fn info(&self) {
        println!("{} has a damage {}", self.name, self.damage);
    }
}

pub trait Blacksmith {
    fn manufacture_weapon(&self, weapon_type: WeaponType) -> Weapon;
}
