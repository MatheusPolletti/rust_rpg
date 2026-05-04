use crate::enemy::Enemy;
use crate::player::Player;
use rand::Rng;
use std::io::{self, Write};

pub fn start_campaign(player: &mut Player) {
    let mut enemy = Enemy::spawn_random(player.level);

    println!("\nUM NOVO DESAFIO! {} apareceu!", enemy.name);

    loop {
        let sword_damage = player.sword_damage;
        let sword_name = player.sword_name.clone();
        let player_strength = player.strength;
        let player_mana_damage = player.mana_damage;

        println!("\n==============================");
        println!(
            "Sua Vida: {} | Vida do Inimigo: {}",
            player.health, enemy.health
        );
        println!("O que você quer fazer?");
        println!("[1] Atacar com a {sword_name} (Dano = {sword_damage} + {player_strength})");
        println!("[2] Usar Magia (Gasta 1 Mana, Causa {player_mana_damage} de Dano)");
        println!(
            "[3] Usar Poção (Restam: {}, Recupera 15 de Vida)",
            player.potion
        );

        print!("Sua ação: ");
        io::stdout().flush().unwrap();

        let mut action = String::new();
        io::stdin().read_line(&mut action).expect("Erro");

        match action.trim() {
            "1" => {
                println!("\nVocê ataca!");
                let damage = sword_damage + player_strength;
                enemy.take_damage(damage);
            }
            "2" => {
                if player.mana > 0 {
                    println!("\nVocê conjura uma bola de fogo!");
                    player.mana -= 1;
                    enemy.take_damage(player_mana_damage);
                } else {
                    println!("\nSem mana! Escolha outro item para usar.");
                    continue;
                }
            }
            "3" => {
                if player.potion > 0 {
                    player.potion -= 1;

                    player.health += 10;

                    if player.health > player.max_health {
                        player.health = player.max_health;
                    }
                    println!(
                        "\nVocê bebeu uma poção e recuperou vida! (Vida atual: {}/{})",
                        player.health, player.max_health
                    );
                } else {
                    println!("\nVocê revirou a mochila, mas não encontrou nenhuma poção!");
                    continue;
                }
            }
            _ => {
                println!("\nVocê se confundiu e perdeu a vez!");
            }
        }

        if !enemy.is_alive() {
            println!("\nVOCÊ VENCEU! O {} caiu!", enemy.name);
            generate_loot(player);
            break;
        }

        println!("\nTurno do {}!", enemy.name);
        player.take_damage(enemy.damage);

        if !player.is_alive() {
            println!("\nVOCÊ MORREU! Sua jornada termina aqui...");
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
        let potions = rng.gen_range(1..=2);
        player.potion += potions;
        println!("🧪 Você encontrou {potions} Poção de Vida!");
    }
    println!("-------------------");
}
