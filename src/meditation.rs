use crate::combat;
use crate::player::Player;
use rand::Rng;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

pub fn go_to_temple(player: &mut Player) {
    println!("\n✨ Você inicia uma peregrinação rumo ao Templo da Clareza...");
    println!("🌿 O ar se torna pesado e o Bosque Maldito sussurra segredos em seus ouvidos.");

    thread::sleep(Duration::from_millis(1500));

    for _ in 0..5 {
        print!("🚶 ");
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(600));
    }

    let mut rng = rand::thread_rng();
    let evento = rng.gen_range(1..=10);

    if evento <= 4 {
        println!(
            "\nO silêncio é interrompido pelo estalar de galhos! Uma criatura bloqueia seu caminho!"
        );
        combat::start_campaign(player);
        println!(
            "🛡️ Exausto da batalha, você decide que é melhor voltar para a segurança do acampamento."
        );
    } else {
        println!(
            "\n🏛️ As árvores se abrem, revelando as ruínas de um templo ancestral banhado por luz etérea."
        );
        println!("O que você busca neste lugar sagrado?");
        println!("[1] Canalizar energia (Recuperar Mana)");
        println!("[2] Transcender limites (Expandir Mana Máxima - Difícil)");

        loop {
            print!("\nSua escolha: ");
            io::stdout().flush().unwrap();
            let mut choice = String::new();
            io::stdin().read_line(&mut choice).expect("Erro");

            match choice.trim() {
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
    }
}

fn meditate_logic(player: &mut Player) {
    println!("\n🧘 Você se senta em meio às runas e esvazia sua mente...");
    thread::sleep(Duration::from_millis(2000));

    let mut rng = rand::thread_rng();
    let mana_recuperada = rng.gen_range(2..=5);

    player.mana = std::cmp::min(player.mana + mana_recuperada, player.max_mana);

    println!("🌀 Uma aura azulada envolve seu corpo. Você sentiu o fluxo de mana retornar.");
    println!("✨ Mana restaurada: {}/{}", player.mana, player.max_mana);
}

fn achive_more_mana(player: &mut Player) {
    println!("\n🌌 Você foca toda a sua vontade em romper as barreiras da sua alma...");
    println!("(Isso pode levar algum tempo...)");

    for _ in 0..10 {
        print!("🔥");
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(400));
    }

    let mut rng = rand::thread_rng();
    let chance_achieving_mana = rng.gen_range(1..=90);

    if chance_achieving_mana == 1 {
        player.max_mana += 1;

        if player.max_mana == 1 {
            println!(
                "\n🌟 AS BARREIRAS CAÍRAM! Pela primeira vez, você sente o pulso do universo em suas veias."
            );
            println!("🔮 Você despertou o dom da Magia!");
        } else {
            println!(
                "\n🌠 Seu receptáculo espiritual se expande! Você agora suporta mais energia arcana."
            );
            println!("📈 Nova Mana Máxima: {}", player.max_mana);
        }
    } else {
        println!("\n🍃 Suas energias vacilaram... O cosmos permanece em silêncio desta vez.");
        println!("Você sente que está mais perto, mas ainda não foi o suficiente.");
    }
}
