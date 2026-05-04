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

        let desc_pocao = std::cmp::min(player.charisma, 8);
        let preco_pocao = 10 - desc_pocao;

        let preco_espada1 = 30u32.saturating_sub(player.charisma);
        let preco_espada2 = 40u32.saturating_sub(player.charisma);
        let preco_espada3 = 60u32.saturating_sub(player.charisma);
        let preco_espada4 = 80u32.saturating_sub(player.charisma);
        let preco_espada5 = 160u32.saturating_sub(player.charisma);

        println!(
            "\n[1] 🧪 Poção de Vida (Recupera 10 de Vida) - 💰 {} moedas",
            preco_pocao
        );
        println!(
            "[2] 🗡️ Espada de Pedra (Dano: 7) - 💰 {} moedas",
            preco_espada1
        );
        println!(
            "[3] 🗡️ Espada de Ferro (Dano: 11) - 💰 {} moedas",
            preco_espada2
        );
        println!(
            "[4] ⚔️ Espada de Aço Escuro (Dano: 16) - 💰 {} moedas",
            preco_espada3
        );
        println!(
            "[5] ⚔️ Espada de Diamante (Dano: 22) - 💰 {} moedas",
            preco_espada4
        );
        println!(
            "[6] ⚔️ Espada do Diabo (Dano: 30) - 💰 {} moedas",
            preco_espada5
        );

        println!("[7] Sair do Mercado");

        print!("O que deseja comprar? ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Erro ao ler");

        match choice.trim() {
            "1" => comprar_item(player, preco_pocao, "Poção de Vida"),
            "2" => comprar_espada(player, preco_espada1, "Espada de Pedra", 7),
            "3" => comprar_espada(player, preco_espada2, "Espada de Ferro", 11),
            "4" => comprar_espada(player, preco_espada3, "Espada de Aço Escuro", 16),
            "5" => comprar_espada(player, preco_espada4, "Espada de Diamante", 22),
            "6" => comprar_espada(player, preco_espada5, "Espada do Diabo", 30),
            "7" => {
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
            println!("\n✅ Você comprou uma {nome}! (Total: {})", player.potion);
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
