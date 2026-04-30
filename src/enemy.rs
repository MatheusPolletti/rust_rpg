pub struct Enemy {
    pub name: String,
    pub health: u32,
    pub damage: u32,
}

impl Enemy {
    pub fn new(name: &str, health: u32, damage: u32) -> Self {
        Self {
            name: name.to_string(),
            health,
            damage,
        }
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0
    }

    pub fn take_damage(&mut self, amount: u32) {
        self.health = self.health.saturating_sub(amount);
        println!(
            "🗡️ {} recebeu {} de dano! (Vida: {})",
            self.name, amount, self.health
        );
    }

    pub fn spawn_horde() -> Vec<Enemy> {
        vec![
            Enemy::new("Goblin Ladrão", 10, 2),
            Enemy::new("Orc Guerreiro", 20, 4),
            Enemy::new("Lorde Demônio", 20, 7),
        ]
    }
}
