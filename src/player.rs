use crate::utils;

pub struct Player {
    pub name: String,
    pub achievements: Vec<String>,
    pub sword_name: String,
    pub sword_damage: u32,
    pub armour_name: String,
    pub armour_protection: u32,
    pub strength: u32,
    pub max_strength: u32,
    pub mana: u32,
    pub max_mana: u32,
    pub mana_damage: u32,
    pub charisma: u32,
    pub max_charisma: u32,
    pub health: u32,
    pub max_health: u32,
    pub potion: u32,
    pub coins: u32,
    pub level: u32,
    pub xp: u32,
}

impl Player {
    pub fn new(name: String, str: u32, mn: u32, cha: u32) -> Self {
        Self {
            name,
            achievements: Vec::new(),
            sword_name: "Espada de Madeira".to_string(),
            sword_damage: 2,
            armour_name: "Roupas de Camponês".to_string(),
            armour_protection: 0,
            strength: str,
            max_strength: 100,
            mana: mn,
            max_mana: mn,
            mana_damage: 5,
            charisma: cha,
            max_charisma: 20,
            health: 20,
            max_health: 20,
            potion: 3,
            coins: 5,
            level: 1,
            xp: 0,
        }
    }

    pub fn show_status(&self) {
        println!("=== 📜 FICHA DO HERÓI ===");
        println!(
            "👤 Nome: {} | 🌟 Nível: {} (XP: {}/100)",
            self.name, self.level, self.xp
        );
        println!(
            "🗡️ Arma: {} (Dano Base: {})",
            self.sword_name, self.sword_damage
        );
        println!(
            "🛡️ Armadura: {} (Defesa: {})",
            self.armour_name, self.armour_protection
        );
        println!("-------------------------");
        println!("💪 Força:   {}/{}", self.strength, self.max_strength);
        println!(
            "✨ Mana:    {}/{} (Poder Magia: {})",
            self.mana, self.max_mana, self.mana_damage
        );
        println!("🗣️ Carisma: {}/{}", self.charisma, self.max_charisma);
        println!("❤️ Vida:    {}/{}", self.health, self.max_health);
        println!("-------------------------");
        println!(
            "🎒 Inventário: 🧪 {} Poções | 💰 {} Moedas",
            self.potion, self.coins
        );
    }

    pub fn show_achievements(&self) {
        println!("=== 📜 Conquistas do Herói {} ===", self.name);

        if !self.achievements.is_empty() {
            println!("Conquistas: {}", self.achievements.join(","));
        } else {
            println!("Nenhuma conquista. Vá à luta!");
        };
    }

    pub fn add_achievement(&mut self, achievement: String) {
        if !self.achievements.contains(&achievement) {
            self.achievements.push(achievement)
        }
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0
    }

    pub fn take_damage(&mut self, protection: u32, damage: u32) {
        let final_damage = damage.saturating_sub(protection);

        self.health = self.health.saturating_sub(final_damage);

        println!(
            "💥 {} levou {} de dano! (A armadura bloqueou {})",
            self.name, final_damage, protection
        );
    }

    pub fn equip_sword(&mut self, name: String, damage: u32) {
        println!("✅ Você sacou a {} (Dano: {})!", name, damage);

        self.sword_name = name;
        self.sword_damage = damage;
    }

    pub fn equip_armour(&mut self, name: String, protection: u32) {
        println!("✅ Você vestiu a {} (Defesa: {})!", name, protection);

        self.armour_name = name;
        self.armour_protection = protection;
    }

    pub fn gain_xp(&mut self, xp_gained: u32) {
        self.xp += xp_gained;
        println!("✨ +{} XP", xp_gained);

        while self.xp >= 100 {
            self.level += 1;
            self.xp -= 100;
            self.max_health += 5;
            self.mana = self.max_mana;
            self.health = self.max_health;

            match self.level {
                5 => {
                    self.add_achievement("👣 Desbravador".to_string());
                    println!("🏆 Conquista: Você não é mais um novato!");
                }
                10 => {
                    self.add_achievement("⚔️ Veterano".to_string());
                    println!("🏆 Conquista: Suas cicatrizes contam histórias de vitória.");
                }
                15 => {
                    self.add_achievement("👑 Lenda".to_string());
                    println!("🏆 Conquista: O Bosque Maldito treme sob seus pés.");
                }
                _ => {}
            }

            utils::clear_console();
            println!("🎉 LEVEL UP! Você subiu para o Nível {}! 🎉", self.level);
            println!("Sua vida foi totalmente curada e o limite de HP aumentou!");

            let points_to_distribute = self.level.saturating_div(2).max(1); // Garante pelo menos 1 ponto
            self.distribute_points(points_to_distribute);
        }
    }

    fn distribute_points(&mut self, mut points: u32) {
        while points > 0 {
            println!("\n🌟 Você tem {} Ponto(s) de Atributo livre.", points);
            println!("[1] 💪 +1 Força (Atual: {})", self.strength);
            println!("[2] ✨ +1 Mana Máxima (Atual: {})", self.max_mana);
            println!(
                "[3] 🔥 +1 Dano Mágico (Custo: 2 Pontos | Atual: {})",
                self.mana_damage
            );
            println!("[4] 🗣️ +1 Carisma (Atual: {})", self.charisma);

            let input = utils::read_input("Onde deseja focar seu treinamento? ");

            match input.as_str() {
                "1" => {
                    self.strength += 1;
                    points -= 1;
                    println!("✅ Força aumentada!");
                }
                "2" => {
                    self.max_mana += 1;
                    self.mana += 1;
                    points -= 1;
                    println!("✅ Mana máxima aumentada!");
                }
                "3" => {
                    if points >= 2 {
                        points -= 2;
                        self.mana_damage += 2;
                        println!("✅ Dano Mágico amplificado!");
                    } else {
                        println!("❌ Você precisa de 2 pontos para upar o Dano Mágico.");
                    }
                }
                "4" => {
                    self.charisma += 1;
                    points -= 1;
                    println!("✅ Carisma aumentado!");
                }
                _ => println!("❌ Opção inválida."),
            }
        }
        utils::press_enter_to_continue();
    }
}
