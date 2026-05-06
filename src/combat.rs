use crate::enemy::Enemy;
use crate::player::Player;
use crate::utils;
use rand::Rng;
use std::thread;
use std::time::Duration;

pub fn start_campaign(player: &mut Player) {
    utils::clear_console();
    let mut enemy = Enemy::spawn_random(player.level);

    println!("🌳 Você adentra a escuridão do Bosque Maldito...");
    thread::sleep(Duration::from_secs(1));
    println!("⚠️ UM NOVO DESAFIO! {} pulou das sombras!", enemy.name);
    utils::press_enter_to_continue();

    loop {
        utils::clear_console();
        let sword_damage = player.sword_damage;
        let sword_name = player.sword_name.clone();
        let player_strength = player.strength;
        let player_mana_damage = player.mana_damage;
        let bonus_player_charisma = player.charisma >= 15;

        let text_run = if bonus_player_charisma {
            "🏃 Fuga sob o Véu das Palavras (Bônus de Carisma)"
        } else {
            "🏃 Tentar Fugir"
        };

        println!("================ BATALHA ================");
        println!(
            "🖤 Sua Vida: {}/{} | 👹 Vida do Inimigo: {}",
            player.health, player.max_health, enemy.health
        );
        println!(
            "✨ Mana: {}/{}      | 🧪 Poções: {}",
            player.mana, player.max_mana, player.potion
        );
        println!("=========================================");
        println!("Escolha sua ação:");
        println!(
            "[1] 🗡️ Atacar com {} (Dano: {} + {})",
            sword_name, sword_damage, player_strength
        );
        println!(
            "[2] 🔥 Usar Magia (Custo: 1 Mana | Dano: {})",
            player_mana_damage
        );
        println!("[3] 🧪 Usar Poção (Cura: 15 HP)");
        println!("[4] {}", text_run);

        let action = utils::read_input("\nAção: ");

        utils::clear_console();
        match action.as_str() {
            "1" => {
                println!("⚔️ Você avança e ataca ferozmente!");
                let damage = sword_damage + player_strength;
                enemy.take_damage(damage);
            }
            "2" => {
                if player.mana > 0 {
                    println!("🔥 Você recita palavras antigas e conjura uma bola de fogo!");
                    player.mana -= 1;
                    enemy.take_damage(player_mana_damage);
                } else {
                    println!("❌ Você tenta puxar energia mágica, mas sua Mana está vazia!");
                    utils::press_enter_to_continue();
                    continue;
                }
            }
            "3" => {
                if player.potion > 0 {
                    player.potion -= 1;
                    player.health = std::cmp::min(player.health + 15, player.max_health);
                    println!(
                        "🧪 Você bebe uma poção! Vida restaurada para {}/{}",
                        player.health, player.max_health
                    );
                } else {
                    println!("❌ Você revirou a mochila em pânico... Nenhuma poção restante!");
                    utils::press_enter_to_continue();
                    continue;
                }
            }
            "4" => {
                println!(
                    "\n{}",
                    if bonus_player_charisma {
                        "✨ No meio da batalha, você usa seus encantos naturais; um sussurro no vento e palavras \
            estudadas confundem o inimigo. Aproveitando o instante de desnorteio, você desaparece com elegância."
                    } else {
                        "🏃 Você tenta fugir desesperadamente, sem muito plano, apenas contando com a sorte e com as pernas."
                    }
                );
                for _ in 0..6 {
                    print!("💨 ");
                    std::io::Write::flush(&mut std::io::stdout()).unwrap();
                    thread::sleep(Duration::from_millis(200));
                }

                let mut rng = rand::thread_rng();

                let change_run = if bonus_player_charisma { 0.9 } else { 0.7 };

                if rng.gen_bool(change_run) {
                    let division_money_lost = if bonus_player_charisma { 4 } else { 2 };
                    let coins_lost = if player.coins > 0 {
                        rng.gen_range(1..=player.coins / division_money_lost + 1)
                    } else {
                        0
                    };

                    let chance_perder_pocao = if bonus_player_charisma { 0.2 } else { 0.5 };
                    let potions_lost = if player.potion > 0 && rng.gen_bool(chance_perder_pocao) {
                        1
                    } else {
                        0
                    };

                    player.coins = player.coins.saturating_sub(coins_lost);
                    player.potion = player.potion.saturating_sub(potions_lost);

                    println!("\n\n✅ Você conseguiu despistar o inimigo!");

                    if coins_lost > 0 || potions_lost > 0 {
                        print!("💔 No entanto, na pressa, você acabou perdendo: ");
                        if coins_lost > 0 {
                            print!("💰 {} moedas ", coins_lost);
                        }
                        if potions_lost > 0 {
                            print!("🧪 {} poção(ões) ", potions_lost);
                        }
                        println!();
                    } else {
                        println!("🍀 Incrível! Você saiu sem deixar cair nada.");
                    }
                    utils::press_enter_to_continue();
                    break;
                } else {
                    println!("\n\n❌ Você tropeçou em uma raiz! O inimigo está logo atrás!");
                }
            }
            _ => {
                println!("❓ Você hesitou, o medo te paralisou e você perdeu a vez!");
            }
        }

        if !enemy.is_alive() {
            println!("\n🏆 VITÓRIA! O {} caiu perante você!", enemy.name);
            generate_loot(player);
            utils::press_enter_to_continue();
            break;
        }

        println!("\n-- Turno Inimigo --");
        println!("👹 O {} contra-ataca!", enemy.name);
        player.take_damage(player.armour_protection, enemy.damage);
        utils::press_enter_to_continue();

        if !player.is_alive() {
            break;
        }
    }
}

fn generate_loot(player: &mut Player) {
    let mut rng = rand::thread_rng();
    println!("\n🎁 --- RECOMPENSAS ---");

    let xp_drop = rng.gen_range(20..=50);
    player.gain_xp(xp_drop);

    if rng.gen_bool(0.7) {
        let moedas = rng.gen_range(5..=15);
        player.coins += moedas;
        println!("💰 Ouro saqueado: {}", moedas);
    }

    if rng.gen_bool(0.3) {
        let potions = rng.gen_range(1..=2);
        player.potion += potions;
        println!("🧪 Poções encontradas: {}", potions);
    }
    println!("----------------------");
}
