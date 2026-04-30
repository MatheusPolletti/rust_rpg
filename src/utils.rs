use crate::player::Player;
use std::io;

pub fn choose_attributes(name: String) -> Player {
    let mut points: u32 = 10;
    let strength: u32;
    let mana: u32;

    println!("\nOlá {name}, você tem {points} pontos para distribuir.");

    loop {
        println!("Diga de 1 a {} quanto de Força você quer:", points);

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Erro ao ler");

        match input.trim().parse::<u32>() {
            Ok(0) => {
                println!("Você precisa investir pelo menos 1 ponto!");
            }
            Ok(valor) if valor <= points => {
                strength = valor;
                points -= valor;
                break;
            }
            Ok(_) => {
                println!(
                    "Você não tem pontos suficientes para isso! Máximo: {}",
                    points
                );
            }
            Err(_) => {
                println!("Por favor, digite um número válido!");
            }
        }
    }

    loop {
        println!(
            "\nDiga de 0 a {} quanto de Mana você quer (o resto virará vida extra):",
            points
        );

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Erro ao ler");

        match input.trim().parse::<u32>() {
            Ok(valor) if valor <= points => {
                mana = valor;
                points -= valor;
                break;
            }
            Ok(_) => {
                println!("Você não tem pontos suficientes para isso!");
            }
            Err(_) => {
                println!("Por favor, digite um número válido!");
            }
        }
    }

    let extra_health = points;

    if extra_health > 0 {
        println!(
            "\nVocê guardou {} pontos! Eles foram convertidos em Vida Extra.",
            extra_health
        );
    } else {
        println!("\nVocê gastou todos os seus pontos em Força e Mana.");
    }

    Player::new(name, strength, mana, extra_health)
}

pub fn greetings() -> String {
    println!("Bem-vindo ao RPG SB Tecnologia.");

    let mut name = String::new();
    let stdin = io::stdin();

    loop {
        println!("Adicione o seu nome:");

        name.clear();

        stdin.read_line(&mut name).expect("Erro ao ler a linha");

        let name = name.trim();

        let total_letters_name = name.chars().count();

        match total_letters_name {
            n if n < 2 => {
                println!("Nome muito curto, precisa de pelo menos 2 dígitos");
            }
            2..=100 => {
                return name.to_string();
            }
            _ => {
                println!("Nome muito longo, no máximo 100 dígitos",);
            }
        }
    }
}
