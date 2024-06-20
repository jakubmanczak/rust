#[derive(Debug, PartialEq)]
enum MarioForm {
    Mario,
    SuperMario,
    FireMario,
    CapeMario,
}

#[derive(Debug)]
enum Powerup {
    Mushroom,
    FireFlower,
    Feather,
}

#[derive(Debug)]
struct Player {
    form: MarioForm,
    lives: u32,
}

impl Player {
    fn new() -> Player {
        Player {
            form: MarioForm::Mario,
            lives: 3,
        }
    }

    fn collect_power_up(&mut self, powerup: Powerup) {
        use MarioForm::*;
        use Powerup::*;
        match (&self.form, powerup) {
            (Mario, Mushroom) => self.form = SuperMario,
            (_, Feather) => self.form = CapeMario,
            (_, FireFlower) => self.form = FireMario,
            (_, Mushroom) => self.lives += 1,
        }
    }

    fn take_damage(&mut self) {
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

fn main() {
    let mut p = Player::new();
    p.collect_power_up(Powerup::Mushroom);
    p.collect_power_up(Powerup::Mushroom);
    p.collect_power_up(Powerup::Feather);
    p.collect_power_up(Powerup::Mushroom);
    p.collect_power_up(Powerup::Mushroom);
    p.take_damage();
    p.take_damage();
    p.take_damage();
    p.collect_power_up(Powerup::Mushroom);
    p.collect_power_up(Powerup::FireFlower);

    assert!(p.form == MarioForm::FireMario);
    assert!(p.lives == 4);

    loop {
        p.take_damage();
        if p.lives == 0 {
            p.lives = u32::MAX;
            break;
        }
    }
    println!("Hello, mario!");
    println!("Mario has {} lives.", p.lives);
}
