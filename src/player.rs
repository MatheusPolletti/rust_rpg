pub struct Player {
    pub name: String,
    pub strength: u32,
    pub mana: u32,
    pub health: u32,
    pub potion: u32,
    pub coins: u32,
    pub level: u32,
    pub xp: u32,
}

impl Player {
    pub fn new(name: String, str: u32, mn: u32, htl: u32) -> Self {
        Self {
            name,
            strength: str,
            mana: mn,
            health: 10 + htl,
            potion: 3,
            coins: 10,
            level: 1,
            xp: 0,
        }
    }

    pub fn show_status(&self) {
        println!("\n--- Ficha do Herói ---");
        println!(
            "Nome: {} | Nível: {} (XP: {}/100)",
            self.name, self.level, self.xp
        );
        println!("Força: {}", self.strength);
        println!("Mana: {}", self.mana);
        println!("Vida: {}", self.health);
        println!(
            "Inventário: 🧪 {} Poções | 💰 {} Moedas",
            self.potion, self.coins
        );
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0
    }

    pub fn take_damage(&mut self, damage: u32) {
        self.health = self.health.saturating_sub(damage);
        println!(
            "💥 {} recebeu {} de dano! (Vida restante: {})",
            self.name, damage, self.health
        );
    }

    pub fn gain_xp(&mut self, xp_gained: u32) {
        self.xp += xp_gained;
        println!("✨ Você ganhou {} de XP!", xp_gained);

        if self.xp >= 100 {
            self.level += 1;
            self.xp -= 100;
            self.health += 5;
            self.strength += 1;
            println!("LEVEL UP! Você alcançou o nível {}!", self.level);
        }
    }
}
