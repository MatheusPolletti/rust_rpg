use rand::Rng;

pub struct Enemy {
    pub name: String,
    pub health: u32,
    pub damage: u32,
}

impl Enemy {
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

    pub fn spawn_random(player_level: u32) -> Self {
        let mut rng = rand::thread_rng();

        let enemy_types = [
            ("Rato Mutante", 8, 1),
            ("Goblin Ladrão", 12, 2),
            ("Lobo Feroz", 15, 3),
            ("Esqueleto Arqueiro", 14, 3),
            ("Orc Guerreiro", 20, 4),
            ("Troll das Floresta", 30, 5),
            ("Lorde Demônio", 40, 7),
        ];

        let index = rng.gen_range(0..enemy_types.len());
        let (base_name, base_health, base_damage) = enemy_types[index];

        let variance = rng.gen_range(80..=120);
        let randomized_health = (base_health * variance) / 100;

        let level_multiplier = player_level - 1;
        let final_health = randomized_health + (randomized_health * level_multiplier * 30 / 100);
        let final_damage = base_damage + level_multiplier;

        let final_name = format!("{} Nv.{}", base_name, player_level);

        Self {
            name: final_name,
            health: final_health,
            damage: final_damage,
        }
    }
}
