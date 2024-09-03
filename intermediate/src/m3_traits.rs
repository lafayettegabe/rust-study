trait Attacker {
    //fn choose_style(&self) -> String;
    fn choose_weapon(&self) -> String;
}

#[derive(Debug)]
enum Character {
    Warrior,
    Archer,
    Wizard,
}

impl Attacker for Character {
    fn choose_weapon(&self) -> String {
        match self {
            Character::Warrior => "Sword".to_string(),
            Character::Archer => "Bow".to_string(),
            Character::Wizard => "Staff".to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tests_traits() {
        let my_character: Character = Character::Archer;
        let choosen_weapon = my_character.choose_weapon();
        dbg!(choosen_weapon);
    }
}
