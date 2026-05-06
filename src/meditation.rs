use crate::combat;
use crate::player::Player;
use crate::utils;
use rand::Rng;
use std::thread;
use std::time::Duration;

pub fn go_to_temple(player: &mut Player) {
    utils::clear_console();
    println!("✨ Você inicia uma peregrinação rumo ao Templo da Clareza...");
    println!("🌿 O ar se torna pesado e o Bosque Maldito sussurra segredos...");

    for _ in 0..5 {
        print!("🚶 ");
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        thread::sleep(Duration::from_millis(500));
    }

    let mut rng = rand::thread_rng();
    let evento = rng.gen_range(1..=10);

    if evento <= 4 {
        println!("\n\n⚠️ O silêncio é rompido pelo estalar de galhos! Uma emboscada!");
        utils::press_enter_to_continue();
        combat::start_campaign(player);

        if player.is_alive() {
            println!("\n🛡️ Exausto, você decide que é melhor voltar para o acampamento.");
            utils::press_enter_to_continue();
        }
    } else {
        utils::clear_console();
        println!(
            "🏛️ As árvores se abrem, revelando as ruínas de um templo banhado por luz etérea.\n"
        );
        println!("O que você busca neste lugar sagrado?");
        println!("[1] 🧘 Canalizar energia (Recuperar Mana atual)");
        println!("[2] 🌌 Transcender limites (Expandir Mana Máxima - Arriscado)");

        loop {
            let choice = utils::read_input("\nSua escolha: ");

            match choice.as_str() {
                "1" => {
                    meditate_logic(player);
                    break;
                }
                "2" => {
                    achive_more_mana(player);
                    break;
                }
                _ => println!("🙏 O silêncio do templo aguarda uma escolha válida."),
            }
        }
        utils::press_enter_to_continue();
    }
}

fn meditate_logic(player: &mut Player) {
    utils::clear_console();
    println!("🧘 Você se senta em meio às runas e esvazia sua mente...");
    thread::sleep(Duration::from_secs(2));

    let mut rng = rand::thread_rng();
    let mana_recuperada = rng.gen_range(2..=5);

    player.mana = std::cmp::min(player.mana + mana_recuperada, player.max_mana);

    println!("\n🌀 Uma aura azulada envolve seu corpo. Você sente o fluxo arcano retornar.");
    println!(
        "✨ Mana restaurada para: {}/{}",
        player.mana, player.max_mana
    );
}

fn achive_more_mana(player: &mut Player) {
    utils::clear_console();
    println!("🌌 Você foca toda a sua vontade em romper as barreiras da sua alma...");
    println!("(Concentrando poder mágico...)");

    for _ in 0..8 {
        print!("🔥 ");
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        thread::sleep(Duration::from_millis(400));
    }

    let mut rng = rand::thread_rng();
    let chance_achieving_mana = rng.gen_range(1..=100);

    if chance_achieving_mana <= 12 {
        player.max_mana += 1;

        if player.max_mana == 1 {
            println!(
                "\n\n🌟 AS BARREIRAS CAÍRAM! Pela primeira vez, você sente o pulso do universo."
            );
            println!("🔮 Você despertou o dom da Magia!");
        } else {
            println!(
                "\n\n🌠 Seu receptáculo espiritual se expandiu! Você suporta mais energia arcana."
            );
            println!("📈 Nova Mana Máxima: {}", player.max_mana);
        }
    } else {
        println!("\n\n🍃 Suas energias vacilaram... O cosmos permanece em silêncio.");
        println!("Você sente que estava perto, mas sua mente não suportou a pressão.");
    }
}
