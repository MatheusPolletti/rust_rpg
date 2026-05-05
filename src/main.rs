use std::io::{self, Write};

mod combat;
mod enemy;
mod market;
mod meditation;
mod player;
mod utils;

fn main() {
    let name_user = utils::greetings();
    let mut player = utils::choose_attributes(name_user);

    println!("\nSua jornada começa agora, {}!", player.name);

    loop {
        if !player.is_alive() {
            break;
        }

        println!("\n========= MENU PRINCIPAL =========");
        println!("[1] Ver Estatísticas");
        println!("[2] Se aventurar no bosque maldito");
        println!("[3] Ir no Mercado");
        println!("[4] Ir ao Templo da Clareza");
        println!("[5] Sair do Jogo");
        print!("O que você quer fazer? ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Erro");

        match choice.trim() {
            "1" => player.show_status(),
            "2" => combat::start_campaign(&mut player),
            "3" => market::visit(&mut player),
            "4" => meditation::go_to_temple(&mut player),
            "5" => {
                println!("\nAté a próxima aventura!");
                break;
            }
            _ => println!("\nOpção inválida. Tente novamente."),
        }
    }

    println!("\nObrigado por jogar o RPG SB Tecnologia!");
}
