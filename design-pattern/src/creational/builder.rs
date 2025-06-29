use std::fmt::Display;

#[derive(Debug)]
pub enum Profession {
    Archer,
    SwordMaster,
    AxeMaster,
}

#[derive(Debug)]
pub enum HairType {
    Short,
    Medium,
    Long,
}

#[derive(Debug)]
pub enum Weapon {
    Arrow,
    Axes,
    Sword,
}

#[derive(Debug)]
pub struct Hero {
    pub name: String,
    pub profession: Profession,
    pub hair_type: HairType,
    pub weapon: Weapon,
    pub damage: f32,
}

impl Hero {
    pub fn builder() -> HeroBuilder {
        HeroBuilder::default()
    }
}

pub struct HeroBuilder {
    profession: Profession,
    hair_type: HairType,
    weapon: Weapon,
}

impl HeroBuilder {
    pub fn profession(mut self, profession: Profession) -> Self {
        self.profession = profession;
        self
    }

    pub fn hair(mut self, hair_type: HairType) -> Self {
        self.hair_type = hair_type;
        self
    }
    pub fn weapon(mut self, weapon: Weapon) -> Self {
        self.weapon = weapon;
        self
    }

    pub fn build(self, name: String) -> Hero {
        let calculated_damage: f32 = match self.profession {
            Profession::Archer => {
                let mut damage: f32 = 10.0;

                if let Weapon::Arrow = self.weapon {
                    damage += 10.0;
                }
                damage
            }
            Profession::SwordMaster => {
                let mut damage: f32 = 10.0;

                if let Weapon::Sword = self.weapon {
                    damage += 10.0;
                }
                damage
            }
            Profession::AxeMaster => {
                let mut damage: f32 = 10.0;

                if let Weapon::Axes = self.weapon {
                    damage += 10.0;
                }
                damage
            }
        };
        Hero {
            name,
            profession: self.profession,
            hair_type: self.hair_type,
            weapon: self.weapon,
            damage: calculated_damage,
        }
    }
}

impl Default for HeroBuilder {
    fn default() -> Self {
        Self {
            profession: Profession::SwordMaster,
            hair_type: HairType::Short,
            weapon: Weapon::Arrow,
        }
    }
}

impl Display for Hero {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{} have a {:?} with damage {}",
            self.name, self.weapon, self.damage
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn creational_builder_test() {
        let axes_hero = Hero::builder()
            .profession(Profession::AxeMaster)
            .hair(HairType::Long)
            .weapon(Weapon::Axes)
            .build("Older Axes Master".into());

        println!("{}", axes_hero);
    }
}
