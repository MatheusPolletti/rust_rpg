use crate::player::Player;
use std::io::{self, Write};

pub fn clear_console() {
    print!("\x1B[2J\x1B[1;1H");

    io::stdout().flush().unwrap();
}

pub fn press_enter_to_continue() {
    print!("\n[Pressione Enter para continuar...]");

    io::stdout().flush().unwrap();

    let mut _temp = String::new();

    io::stdin().read_line(&mut _temp).unwrap_or(0);
}

pub fn read_input(prompt: &str) -> String {
    print!("{}", prompt);

    io::stdout().flush().unwrap();

    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap_or(0);

    input.trim().to_string()
}

pub fn greetings() -> String {
    clear_console();

    println!("⚔️  BEM-VINDO AO RPG SB TECNOLOGIA ⚔️");
    println!("Uma jornada de glória e perigos aguarda você.\n");

    loop {
        let name = read_input("Qual o nome do seu herói? ");
        let total_letters_name = name.chars().count();

        match total_letters_name {
            n if n < 2 => println!("❌ O nome é muito curto. Precisa ter pelo menos 2 letras!"),
            2..=100 => return name,
            _ => println!("❌ O nome é muito longo. Máximo de 100 caracteres!"),
        }
    }
}

pub fn choose_attributes(name: String) -> Player {
    let mut points: u32 = 10;
    let strength: u32;
    let mut mana: u32 = 0;
    let mut charisma: u32 = 0;

    clear_console();
    println!(
        "Saudações, {}! Os deuses lhe concederam {} pontos para moldar seu destino.",
        name, points
    );

    loop {
        println!("\n💪 Pontos restantes: {}", points);
        let input = read_input("Quantos pontos deseja investir em FORÇA? ");

        match input.parse::<u32>() {
            Ok(0) => println!(
                "⚠️ Um herói sem força não levanta uma espada! Invista pelo menos 1 ponto."
            ),
            Ok(valor) if valor <= points => {
                strength = valor;
                points -= valor;
                break;
            }
            Ok(_) => println!("❌ Você não tem pontos suficientes para isso!"),
            Err(_) => println!("❌ Por favor, digite um número válido!"),
        }
    }

    // Loop de Mana
    if points > 0 {
        loop {
            println!("\n✨ Pontos restantes: {}", points);
            let input = read_input("Quantos pontos deseja investir em MANA? ");

            match input.parse::<u32>() {
                Ok(valor) if valor <= points => {
                    mana = valor;
                    points -= valor;
                    break;
                }
                Ok(_) => println!("❌ Você não tem pontos suficientes!"),
                Err(_) => println!("❌ Por favor, digite um número válido!"),
            }
        }
    }

    // Loop de Carisma
    if points > 0 {
        loop {
            println!("\n🗣️ Pontos restantes: {}", points);
            let input = read_input("Quantos pontos deseja investir em CARISMA? ");

            match input.parse::<u32>() {
                Ok(valor) if valor <= points => {
                    charisma = valor;
                    points -= valor;
                    break;
                }
                Ok(_) => println!("❌ Você não tem pontos suficientes!"),
                Err(_) => println!("❌ Por favor, digite um número válido!"),
            }
        }
    }

    clear_console();
    if points > 0 {
        println!(
            "⚠️ Você deixou {} pontos sem gastar! Eles se perderam no éter...",
            points
        );
    } else {
        println!("✅ Atributos definidos com sucesso!");
    }
    press_enter_to_continue();

    Player::new(name, strength, mana, charisma)
}
