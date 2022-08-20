mod book;
mod cli;
mod documents;
mod node;
mod tokens;
mod utils;
mod config;

use book::Book;
use cli::{Action, Cli};
use documents::Document;

fn main() {
    let cli = Cli::new();

    match cli.get_command() {
        Action::NewBook { name, force } => Book::new(&name, force),
    }
}
