mod combat;
mod enemy;
mod market;
mod meditation;
mod player;
mod utils;

fn main() {
    let name_user = utils::greetings();
    let mut player = utils::choose_attributes(name_user);

    loop {
        if !player.is_alive() {
            utils::clear_console();
            println!("\n💀 FIM DE JOGO 💀");
            println!("Obrigado por jogar o RPG SB Tecnologia!");
            break;
        }

        utils::clear_console();
        println!("========= 🏕️  ACAMPAMENTO =========");
        println!("[1] Ver Estatísticas do Herói");
        println!("[2] Ver Conquistas do Herói");
        println!("[3] Se aventurar no Bosque Maldito");
        println!("[4] Visitar o Mercado Local");
        println!("[5] Ir ao Templo da Clareza");
        println!("[6] Sair do Jogo");
        println!("=====================================");

        let choice = utils::read_input("Onde você deseja ir? ");

        match choice.as_str() {
            "1" => {
                utils::clear_console();
                player.show_status();
                utils::press_enter_to_continue();
            }
            "2" => {
                utils::clear_console();
                player.show_achievements();
            }
            "3" => combat::start_campaign(&mut player),
            "4" => market::visit(&mut player),
            "5" => meditation::go_to_temple(&mut player),
            "6" => {
                println!("\nAté a próxima aventura, herói!");
                break;
            }
            _ => {
                println!("\n❌ Opção inválida! Escolha um destino válido.");
                utils::press_enter_to_continue();
            }
        }
    }
}
