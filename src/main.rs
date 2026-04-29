use std::io;

fn main() {
    let name_user: String = greetings();
}

fn greetings() -> String {
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
