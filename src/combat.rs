use crate::enemy::Enemy;
use crate::player::Player;
use rand::Rng;
use std::io::{self, Write};

pub fn start_campaign(player: &mut Player) {
    let horda = Enemy::spawn_horde();

    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..horda.len());
    let mut enemy = horda.into_iter().nth(index).unwrap();

    println!("\nUM NOVO DESAFIO! {} apareceu!", enemy.name);

    loop {
        println!("\n==============================");
        println!(
            "Sua Vida: ❤️ {} | Vida do Inimigo: 👹 {}",
            player.health, enemy.health
        );
        println!("O que você quer fazer?");
        println!("[1] Atacar com a Espada (Dano = Força + 2)");
        println!("[2] Usar Magia (Gasta 1 Mana, Causa 5 de Dano)");
        println!(
            "[3] Usar Poção (Restam: 🧪 {}, Recupera 15 de Vida)",
            player.potion
        );

        print!("Sua ação: ");
        io::stdout().flush().unwrap();

        let mut action = String::new();
        io::stdin().read_line(&mut action).expect("Erro");

        match action.trim() {
            "1" => {
                println!("\nVocê ataca ferozmente!");
                let damage = 2 + player.strength;
                enemy.take_damage(damage);
            }
            "2" => {
                if player.mana > 0 {
                    println!("\nVocê conjura uma bola de fogo! 🔥");
                    player.mana -= 1;
                    enemy.take_damage(5);
                } else {
                    println!("\nSem mana! O feitiço falhou e você perdeu o turno!");
                }
            }
            "3" => {
                if player.potion > 0 {
                    println!("\n🧪 Glub glub... Você bebeu uma poção e recuperou 15 de vida!");
                    player.potion -= 1;
                    player.health += 15;
                } else {
                    println!(
                        "\n❌ Você revirou a mochila, mas não encontrou nenhuma poção! Perdeu a vez!"
                    );
                }
            }
            _ => {
                println!("\nVocê se confundiu e perdeu a vez!");
            }
        }

        if !enemy.is_alive() {
            println!("\n🏆 VOCÊ VENCEU! O {} caiu!", enemy.name);
            generate_loot(player);
            break;
        }

        println!("\nTurno do {}!", enemy.name);
        player.take_damage(enemy.damage);

        if !player.is_alive() {
            println!("\n💀 VOCÊ MORREU! Sua jornada termina aqui...");
            break;
        }
    }
}

fn generate_loot(player: &mut Player) {
    let mut rng = rand::thread_rng();
    println!("\n--- Recompensas ---");

    let xp_drop = rng.gen_range(20..=50);
    player.gain_xp(xp_drop);

    if rng.gen_bool(0.7) {
        let moedas = rng.gen_range(1..=5);
        player.coins += moedas;
        println!("💰 Você encontrou {} moedas!", moedas);
    }

    if rng.gen_bool(0.3) {
        player.potion += 1;
        println!("🧪 Você encontrou 1 Poção de Vida!");
    }
    println!("-------------------");
}
