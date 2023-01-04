struct HasHealth {
    health: u32,
}

pub fn get_updated_health(mut health: u32, damage: u32) -> u32 {
    if health < damage {
        health = 0;
    } else {
        health -= damage;
    }

    health
}

pub trait Health {
    fn get_health(&self) -> u32;

    fn apply_damage(&mut self, damage: u32) -> &mut Self;
}
