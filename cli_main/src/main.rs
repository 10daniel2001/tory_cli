// Projeto CLI_Rust para linux, CLI mais moderna e segura 


use anyhow::Ok;
use clap::Parser;

#[derive(Parser)]
#[command(name = "tory", version, about = "Minha CLI")]
struct Cli {
    nome: String,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    println!("Opa, {}!", cli.nome);
    Ok(())

}