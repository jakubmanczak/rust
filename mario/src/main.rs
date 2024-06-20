mod mario;
use mario::*;

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
