use anyhow::{Context, Result, bail};
use std::io::{self, Write};
use std::process::Command;

fn main() -> Result<()> {
    let stdin = io::stdin();
    let mut line = String::new();

    loop {
        let current_dir = std::env::current_dir().context("não foi possível ler a pasta atual")?;
        print!("tory:{}$ ", current_dir.display());
        io::stdout()
            .flush()
            .context("não foi possível exibir o prompt")?;

        line.clear();
        if stdin
            .read_line(&mut line)
            .context("não foi possível ler o comando")?
            == 0
        {
            break;
        }

        match execute_line(&line) {
            Ok(true) => {}
            Ok(false) => break,
            Err(error) => eprintln!("tory: {error:#}"),
        }
    }

    Ok(())
}

fn execute_line(line: &str) -> Result<bool> {
    let command_line = line.trim();
    if command_line.is_empty() {
        return Ok(true);
    }

    let command_end = command_line
        .find(char::is_whitespace)
        .unwrap_or(command_line.len());
    let command = &command_line[..command_end];
    let arguments = command_line[command_end..].trim();

    match command {
        "exit" if arguments.is_empty() => return Ok(false),
        "exit" => bail!("o comando 'exit' não aceita argumentos"),
        "cd" => change_directory(arguments)?,
        _ => {
            Command::new("/bin/sh")
                .arg("-c")
                .arg(command_line)
                .status()
                .context("não foi possível iniciar /bin/sh")?;
        }
    }

    Ok(true)
}

fn change_directory(arguments: &str) -> Result<()> {
    let path = match parse_single_argument(arguments)? {
        Some(path) => expand_home(path),
        None => std::path::PathBuf::from(
            std::env::var_os("HOME").context("a variável HOME não está definida")?,
        ),
    };

    std::env::set_current_dir(&path)
        .with_context(|| format!("não foi possível acessar '{}'", path.display()))
}

fn parse_single_argument(input: &str) -> Result<Option<String>> {
    let mut chars = input.chars().peekable();
    while chars
        .peek()
        .is_some_and(|character| character.is_whitespace())
    {
        chars.next();
    }
    if chars.peek().is_none() {
        return Ok(None);
    }

    let mut argument = String::new();
    let mut quote = None;
    let mut escaped = false;
    let mut started = false;

    while let Some(character) = chars.next() {
        started = true;
        if escaped {
            argument.push(character);
            escaped = false;
            continue;
        }

        match quote {
            Some('\'') if character == '\'' => quote = None,
            Some('\'') => argument.push(character),
            Some('"') if character == '"' => quote = None,
            Some('"') if character == '\\' => escaped = true,
            Some('"') => argument.push(character),
            _ if character == '\\' => escaped = true,
            _ if character == '\'' || character == '"' => quote = Some(character),
            _ if character.is_whitespace() => break,
            _ => argument.push(character),
        }
    }

    if escaped || quote.is_some() {
        bail!("aspas ou escape incompletos no caminho");
    }
    if chars.any(|character| !character.is_whitespace()) {
        bail!("use um único caminho com o comando 'cd'");
    }
    if !started {
        return Ok(None);
    }

    Ok(Some(argument))
}

fn expand_home(path: String) -> std::path::PathBuf {
    if path == "~" || path.starts_with("~/") {
        if let Some(home) = std::env::var_os("HOME") {
            let suffix = path.strip_prefix('~').unwrap_or_default();
            return std::path::PathBuf::from(home).join(suffix.trim_start_matches('/'));
        }
    }

    path.into()
}

#[cfg(test)]
mod tests {
    use super::parse_single_argument;

    #[test]
    fn parses_quoted_paths_with_spaces() {
        assert_eq!(
            parse_single_argument("\"pasta com espaços\"").unwrap(),
            Some("pasta com espaços".to_owned())
        );
    }

    #[test]
    fn rejects_multiple_paths() {
        assert!(parse_single_argument("pasta outra").is_err());
    }
}
