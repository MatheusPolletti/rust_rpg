use crate::player::Player;
use crate::utils;

pub fn visit(player: &mut Player) {
    loop {
        utils::clear_console();
        println!("--- 🏪 BEM-VINDO AO MERCADO DE TARTARUS ---");
        println!(
            "Seu Carisma: {} | Suas Moedas: 💰 {}",
            player.charisma, player.coins
        );
        println!("Vendedor: 'Dê uma olhada nas minhas mercadorias, aventureiro!'\n");

        let espadas: [(&str, u32, u32); 5] = [
            ("Espada de Pedra", 30, 5),
            ("Espada de Ferro", 40, 9),
            ("Espada de Aço Escuro", 60, 14),
            ("Espada de Diamante", 80, 20),
            ("Espada do Diabo", 160, 28),
        ];

        let armaduras: [(&str, u32, u32); 5] = [
            ("Manto de Aprendiz", 20, 1),
            ("Gibão de Couro", 45, 3),
            ("Cota de Malha", 90, 5),
            ("Armadura de Placas", 180, 9),
            ("Couraça do Dragão", 350, 14),
        ];

        let preco_pocao_vida = 10u32.saturating_sub(player.charisma);
        let preco_pocao_forca = 60u32.saturating_sub(player.charisma);

        println!(
            "[1] Poção de Vida (Recupera 10 HP) - 💰 {} moedas",
            preco_pocao_vida
        );
        println!(
            "[2] Poção de Força (+5 Força Fixa) - 💰 {} moedas\n",
            preco_pocao_forca
        );

        println!("⚔️  ARMAS:");
        for (i, (nome, preco_base, dano)) in espadas.iter().enumerate() {
            let preco_final = preco_base.saturating_sub(player.charisma);
            println!(
                "[{}] {} (Dano: {}) - 💰 {} moedas",
                i + 3,
                nome,
                dano,
                preco_final
            );
        }

        println!("\n🛡️  ARMADURAS:");
        for (i, (nome, preco_base, def)) in armaduras.iter().enumerate() {
            let preco_final = preco_base.saturating_sub(player.charisma);
            // offset de +8 porque as espadas vão do 3 ao 7
            println!(
                "[{}] {} (Def: {}) - 💰 {} moedas",
                i + 8,
                nome,
                def,
                preco_final
            );
        }

        println!("\n[13] Sair do Mercado");

        let choice = utils::read_input("\nO que deseja comprar? ");

        match choice.as_str() {
            "1" => comprar_item(player, preco_pocao_vida, "Poção de Vida"),
            "2" => {
                if player.coins >= preco_pocao_forca {
                    player.coins -= preco_pocao_forca;
                    player.strength = std::cmp::min(player.strength + 5, player.max_strength);
                    println!(
                        "\n💪 Você bebeu a Poção de Força na hora! Suas veias ardem com energia."
                    );
                    println!("Sua força subiu para: {}", player.strength);
                } else {
                    println!("\n❌ Moedas insuficientes!");
                }
            }
            // Mapeando do 3 ao 7 para as Espadas
            n if let Ok(num) = n.parse::<usize>() => {
                if num >= 3 && num <= 7 {
                    let (nome, preco_base, dano) = espadas[num - 3];
                    let preco_final = preco_base.saturating_sub(player.charisma);
                    comprar_equipamento(player, preco_final, nome, dano, true);
                } else if num >= 8 && num <= 12 {
                    let (nome, preco_base, def) = armaduras[num - 8];
                    let preco_final = preco_base.saturating_sub(player.charisma);
                    comprar_equipamento(player, preco_final, nome, def, false);
                } else if num == 13 {
                    println!("\nVendedor: 'Que os deuses protejam seu ouro!'");
                    utils::press_enter_to_continue();
                    break;
                } else {
                    println!("\n❌ Opção inválida.");
                }
            }
            _ => println!("\n❌ Opção inválida."),
        }
        utils::press_enter_to_continue();
    }

    fn comprar_item(player: &mut Player, preco: u32, nome: &str) {
        if player.coins >= preco {
            player.coins -= preco;
            player.potion += 1;
            println!("✅ Você comprou uma {}! (Total: {})", nome, player.potion);
        } else {
            println!("❌ Vendedor: 'Ouro insuficiente, forasteiro!'");
        }
    }

    fn comprar_equipamento(
        player: &mut Player,
        preco: u32,
        nome: &str,
        status: u32,
        is_sword: bool,
    ) {
        if player.coins >= preco {
            player.coins -= preco;
            if is_sword {
                player.equip_sword(nome.to_string(), status);
            } else {
                player.equip_armour(nome.to_string(), status);
            }
        } else {
            println!(
                "❌ Vendedor: 'Você precisa de mais moedas para levar o(a) {}!'",
                nome
            );
        }
    }
}
