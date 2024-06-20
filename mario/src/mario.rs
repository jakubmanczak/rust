#[derive(Debug, PartialEq)]
pub enum MarioForm {
    Mario,
    SuperMario,
    FireMario,
    CapeMario,
}

#[derive(Debug)]
pub enum Powerup {
    Mushroom,
    FireFlower,
    Feather,
}

#[derive(Debug)]
pub struct Player {
    pub form: MarioForm,
    pub lives: u32,
}

impl Player {
    pub fn new() -> Player {
        Player {
            form: MarioForm::Mario,
            lives: 3,
        }
    }

    pub fn collect_power_up(&mut self, powerup: Powerup) {
        use MarioForm::*;
        use Powerup::*;
        match (&self.form, powerup) {
            (Mario, Mushroom) => self.form = SuperMario,
            (_, Feather) => self.form = CapeMario,
            (_, FireFlower) => self.form = FireMario,
            (_, Mushroom) => self.lives += 1,
        }
    }

    pub fn take_damage(&mut self) {
        use MarioForm::*;
        if self.lives == 0 {
            println!("Mario has 0 lives already...");
        } else {
            match &self.form {
                Mario => self.lives -= 1,
                _ => self.form = Mario,
            }
        }
    }
}
