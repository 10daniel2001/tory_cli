# Tory CLI

> Um shell moderno para Linux, escrito em Rust, com interface evoluída, atalhos intuitivos e IA integrada.

![status](https://img.shields.io/badge/status-em%20desenvolvimento-orange)
![linguagem](https://img.shields.io/badge/linguagem-Rust-b7410e)
![plataforma](https://img.shields.io/badge/plataforma-Linux-blue)

---

## Sobre o projeto

O **Tory CLI** nasce de uma ideia simples: o terminal do Linux é poderoso, mas a experiência de uso quase não mudou em décadas. A proposta é criar um shell que seja **bonito, simples e inteligente**, sem perder a força da linha de comando.

Este é um projeto pessoal, feito com foco em aprendizado e em construir algo útil de verdade. A **IA é a principal ferramenta de desenvolvimento** do projeto, e também faz parte do que o Tory CLI quer oferecer ao usuário.

## Objetivos

- **Interface moderna**: cores, destaque de sintaxe e visual limpo, com identidade própria.
- **Informação sempre à vista**: hora e data visíveis no prompt ou na barra do terminal.
- **Atalhos que fazem sentido**: `Ctrl + C` copia e `Ctrl + V` cola, como em qualquer outro programa.
- **Comandos simples**: nomes fáceis de lembrar e mensagens de erro claras, em vez de saídas enigmáticas.
- **IA integrada**: ajuda para explicar comandos, corrigir erros e sugerir o que fazer.
- **Rápido e seguro**: construído em Rust, com bom desempenho e segurança de memória.

## Funcionalidades planejadas

| Funcionalidade | Status |
|---|---|
| Prompt colorido com hora e data | Planejado |
| `Ctrl + C` copia quando há texto selecionado | Planejado |
| `Ctrl + V` cola | Planejado |
| Destaque de sintaxe ao digitar | Planejado |
| Autocompletar inteligente | Planejado |
| Histórico de comandos com busca | Planejado |
| Mensagens de erro amigáveis | Planejado |
| Assistente de IA (explicar e sugerir comandos) | Planejado |
| Temas e personalização | Planejado |
| Pacote instalável (`.deb` e loja do Linux) | Planejado |

## Decisões de design

### Sobre o `Ctrl + C`

No terminal tradicional, `Ctrl + C` **interrompe** o programa em execução (sinal `SIGINT`). Para não perder essa função, o Tory CLI seguirá a abordagem usada por terminais modernos:

- **Com texto selecionado** → `Ctrl + C` copia.
- **Sem texto selecionado** → `Ctrl + C` interrompe o processo, como sempre.

Assim você ganha o atalho que já conhece de outros programas sem perder o controle dos processos.

### Sobre a fonte

A fonte de um terminal é definida pelo emulador de terminal (GNOME Terminal, Kitty, Alacritty etc.), e não pelo shell. O Tory CLI vai recomendar fontes modernas (como as famílias *Nerd Fonts*) e usar ícones e símbolos compatíveis com elas.

## Tecnologias

- **[Rust](https://www.rust-lang.org/)**: linguagem principal.
- **IA**: ferramenta central de desenvolvimento e recurso do produto.
- **Git e GitHub**: versionamento e documentação.

Bibliotecas específicas (interface de terminal, leitura de linha, clipboard) serão definidas ao longo do desenvolvimento.

## Como instalar

> O projeto ainda está no início e não há versão estável.

Para rodar a versão de desenvolvimento, você precisa do [Rust](https://rustup.rs/) instalado:

```bash
# Clonar o repositório
git clone git@github.com:10daniel2001/tory_cli.git
cd tory_cli

# Compilar e executar
cargo run
```

## Estrutura do projeto

```
tory_cli/
├── src/            # código-fonte em Rust
├── Cargo.toml      # configuração do projeto e dependências
├── README.md       # este arquivo
└── LICENSE         # licença do projeto
```

## Roadmap

- [ ] **v0.1**: shell básico que executa comandos e mostra o prompt
- [ ] **v0.2**: prompt com cores, hora e data
- [ ] **v0.3**: atalhos de copiar e colar
- [ ] **v0.4**: histórico e autocompletar
- [ ] **v0.5**: integração com IA
- [ ] **v1.0**: primeira versão estável e empacotada para Linux

## Como contribuir

Contribuições, ideias e sugestões são bem-vindas. Para participar:

1. Faça um fork do projeto.
2. Crie uma branch: `git checkout -b minha-feature`.
3. Faça o commit: `git commit -m "Adiciona minha feature"`.
4. Envie para o GitHub: `git push origin minha-feature`.
5. Abra um Pull Request.

## Licença

Licença ainda a definir (sugestão: MIT).

## Autor

Feito por **[10daniel2001](https://github.com/10daniel2001)**.

---

*Tory CLI: o terminal que você sempre quis usar.*