use crate::player::Player;
use std::io::{self, Write};

pub fn visit(player: &mut Player) {
    loop {
        println!("\n--- BEM-VINDO AO MERCADO ---");

        println!(
            "Seu Carisma: {} | Suas Moedas: 💰 {}",
            player.charisma, player.coins
        );

        println!("Vendedor: 'Dê uma olhada nas minhas mercadorias!'");

        let preco_pocao_vida = 10u32.saturating_sub(player.charisma);
        let price_potion_strength = 60u32.saturating_sub(player.charisma);
        let preco_espada1 = 30u32.saturating_sub(player.charisma);
        let preco_espada2 = 40u32.saturating_sub(player.charisma);
        let preco_espada3 = 60u32.saturating_sub(player.charisma);
        let preco_espada4 = 80u32.saturating_sub(player.charisma);
        let preco_espada5 = 160u32.saturating_sub(player.charisma);

        println!(
            "\n[1] Poção de Vida (Recupera 10 de Vida) - 💰 {} moedas",
            preco_pocao_vida
        );
        println!(
            "\n[2] Poção de Força (Aumenta 5 de força) - 💰 {} moedas",
            price_potion_strength
        );
        println!(
            "[3] Espada de Pedra (Dano: 5) - 💰 {} moedas",
            preco_espada1
        );
        println!(
            "[4] Espada de Ferro (Dano: 9) - 💰 {} moedas",
            preco_espada2
        );
        println!(
            "[5] Espada de Aço Escuro (Dano: 14) - 💰 {} moedas",
            preco_espada3
        );
        println!(
            "[6] Espada de Diamante (Dano: 20) - 💰 {} moedas",
            preco_espada4
        );
        println!(
            "[7] Espada do Diabo (Dano: 28) - 💰 {} moedas",
            preco_espada5
        );

        println!("[8] Sair do Mercado");

        print!("O que deseja comprar? ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Erro ao ler");

        match choice.trim() {
            "1" => comprar_item(player, preco_pocao_vida, "Poção de Vida"),
            "2" => {
                if player.coins >= price_potion_strength {
                    player.coins -= price_potion_strength;
                    player.strength = std::cmp::min(player.strength + 5, player.max_strength);
                    println!(
                        "\n💪 Você usou a Poção de Força! Suas veias ardem com energia pura.\n\
                        [BÔNUS: +5 de Força! Sua força total agora é: {}]",
                        player.strength
                    );
                }
            }
            "3" => comprar_espada(player, preco_espada1, "Espada de Pedra", 5),
            "4" => comprar_espada(player, preco_espada2, "Espada de Ferro", 9),
            "5" => comprar_espada(player, preco_espada3, "Espada de Aço Escuro", 14),
            "6" => comprar_espada(player, preco_espada4, "Espada de Diamante", 20),
            "7" => comprar_espada(player, preco_espada5, "Espada do Diabo", 28),
            "8" => {
                println!("\nVendedor: 'Volte sempre, aventureiro!'");
                break;
            }
            _ => println!("\nOpção inválida."),
        }
    }

    fn comprar_item(player: &mut Player, preco: u32, nome: &str) {
        if player.coins >= preco {
            player.coins -= preco;
            player.potion += 1;
            println!("\nVocê comprou uma {nome}! (Total: {})", player.potion);
        } else {
            println!("\nVendedor: 'Moedas insuficientes para {nome}!'");
        }
    }

    fn comprar_espada(player: &mut Player, preco: u32, nome: &str, dano: u32) {
        if player.coins >= preco {
            player.coins -= preco;
            player.equip_sword(nome.to_string(), dano);
        } else {
            println!("\nVendedor: 'Você não tem moedas suficientes para comprar a {nome}!'");
        }
    }
}
