use std::io::{self, Write};

pub struct Player {
    pub name: String,
    pub sword_name: String,
    pub sword_damage: u32,
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
            sword_name: "Espada de Madeira".to_string(),
            sword_damage: 2,
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
        println!("\n--- Ficha do Herói ---");
        println!(
            "Nome: {} | Nível: {} (XP: {}/100)",
            self.name, self.level, self.xp
        );
        println!(
            "Espada {} com {} de dano",
            self.sword_name, self.sword_damage
        );
        println!("Força: {} / {} ", self.strength, self.max_strength);
        println!(
            "Mana: {} / {} com dano de {}",
            self.mana, self.max_mana, self.mana_damage
        );
        println!("Carisma: {} / {}", self.charisma, self.max_charisma);
        println!("Vida: {} / {}", self.health, self.max_health);
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

    pub fn equip_sword(&mut self, name: String, damage: u32) {
        println!("Você equipou a {} com {} de dano!", name, damage);

        self.sword_name = name;
        self.sword_damage = damage;
    }

    pub fn gain_xp(&mut self, xp_gained: u32) {
        self.xp += xp_gained;
        println!("Você ganhou {} de XP!", xp_gained);

        while self.xp >= 100 {
            self.level += 1;
            self.xp -= 100;
            self.max_health += 5;

            self.mana = self.max_mana;
            self.health = self.max_health;

            println!("\n🎉 LEVEL UP! Você alcançou o nível {}!", self.level);

            self.distribute_points(self.level.saturating_div(2));
        }
    }

    fn distribute_points(&mut self, mut points: u32) {
        println!("{}", points);
        println!("Sua vida máxima aumentou e você foi totalmente curado!");

        while points > 0 {
            println!("\nVocê tem {} ponto(s) para distribuir.", points);
            println!(
                "[1] +1 Força (Atual: {}/{})",
                self.strength, self.max_strength
            );
            println!("[2] +1 Mana Máxima (Atual: {})", self.max_mana);
            println!(
                "[3] +1 Dano de mana (Requer 2 Pontos) (Dano Atual: {})",
                self.mana_damage
            );
            println!(
                "[4] +1 Carisma (Atual: {}/{})",
                self.charisma, self.max_charisma
            );
            print!("Onde deseja investir? ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Erro ao ler");

            match input.trim() {
                "1" => {
                    if self.strength < self.max_strength {
                        self.strength += 1;
                        points -= 1;
                        println!("Força aumentada!");
                    } else {
                        println!("Sua Força já está no máximo!");
                    }
                }
                "2" => {
                    self.max_mana += 1;
                    self.mana += 1;
                    points -= 1;
                    println!("Mana Máxima aumentada!");
                }
                "3" => {
                    if points >= 2 {
                        points -= 2;
                        self.mana_damage += 2;
                        println!("Dano aumentado para {}", self.mana_damage);
                    } else {
                        println!("Precisa de 2 pontos de xp para aumentar o dano");
                        continue;
                    }
                }
                "4" => {
                    if self.charisma < self.max_charisma {
                        self.charisma += 1;
                        points -= 1;
                        println!("Carisma aumentado!");
                    } else {
                        println!("Seu Carisma já está no máximo!");
                    }
                }
                _ => println!("Opção inválida. Escolha 1, 2 ou 3."),
            }
        }
    }
}
