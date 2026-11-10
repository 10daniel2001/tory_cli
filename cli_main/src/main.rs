// Permite ao compilador não reclamar de imports ainda não usados.
// Remova esta linha quando o código estiver usando tudo.
#![allow(unused_imports)]

// ---------- clap: leitura dos argumentos do terminal ----------

// `Parser` é uma trait do clap. Com `#[derive(Parser)]` em uma struct,
// ela passa a saber ler o que o usuário digitou (ex.: `tory --help`).
use clap::Parser;

// `Subcommand` serve para criar subcomandos em um enum
// (ex.: `tory add`, `tory list`, `tory remove`).
use clap::Subcommand;

// ---------- anyhow: tratamento de erros simples ----------

// `Result` do anyhow é um atalho para `Result<T, anyhow::Error>`:
// qualquer tipo de erro cabe nele, então o operador `?` funciona em tudo.
// `Context` adiciona o método `.context("mensagem")`, que explica
// o que estava sendo feito quando o erro aconteceu.
use anyhow::{Context, Result};

// ---------- serde: converter dados de/para texto (JSON) ----------

// `Serialize` transforma uma struct em texto (para salvar em arquivo).
// `Deserialize` faz o caminho contrário (ler o arquivo e montar a struct).
use serde::{Deserialize, Serialize};

// ---------- biblioteca padrão do Rust (std), não precisa de cargo add ----------

// `PathBuf` guarda um caminho de arquivo ou pasta que pode ser montado
// e modificado (ex.: juntar a pasta de config com o nome do arquivo).
use std::path::PathBuf;

// `fs` (file system) tem as funções para ler, criar e escrever arquivos
// (ex.: `fs::read_to_string`, `fs::write`, `fs::create_dir_all`).
use std::fs;

#[derive(Parser)]
// `derive(Parser)` faz o clap gerar sozinho o código que lê os argumentos.
#[command(name = "tory", version, about = "Minha cli em rust")]
struct Cli {
    #[command(Subcommand)]
    comando: comandos, 
}


#[derive(Subcommand)]
enum comandos {
    
    add {

        texto: String,
    }
}