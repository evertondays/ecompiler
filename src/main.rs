use std::fs::File;
use std::io::{BufReader};
use std::process;
use clap::Parser;

mod lexical;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: Option<String>,
}

fn main() {
    let path = get_source_code_path();
    let mut source_code_reader = open_source_code_file(&path);

    println!("Iniciando análise léxica . . .");
    lexical::get_tokens(&mut source_code_reader);
    println!("Análise léxica ✅");
}

fn get_source_code_path() -> String {
    let args = Args::parse();

    match args.path {
        Some(p) => return p,
        None => {
            eprintln!("É necessário passar o diretório do arquivo de código fonte");
            eprintln!("Exemplo: --path \"./teste.e\"");
            process::exit(1);
        },
    }
}

fn open_source_code_file(path: &str) -> BufReader<File> {
    let file = File::open(path);

    match file {
        Ok (content) => {
            return BufReader::new(content);
        }
        Err(err) => {
            eprintln!("Erro ao ler arquivo!\nArquivo não encontrado em {}\nA extensão do arquivo esperado é '.e'\nMais detalhes: {}", path, err);
            process::exit(1);
        }
    }
}