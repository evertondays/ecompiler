// use std::fs::File;
// use std::io::{BufReader};
// use std::process;
use clap::Parser;
use inkwell::context::Context;
// use inkwell::values::BasicValue;

mod lexical;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: Option<String>,
}

fn main() {
    // let path = get_source_code_path();
    // let mut source_code_reader = open_source_code_file(&path);

    // println!("Iniciando análise léxica . . .");
    // lexical::lexer(&mut source_code_reader);

    // 1. O Contexto é o "dono" de todas as estruturas de dados do LLVM (tipos, consts)
    let context = Context::create();
    
    // 2. O Módulo é a unidade de compilação (equivalente a um arquivo .c ou .go)
    let module = context.create_module("meu_programa");
    
    // 3. O Builder é o cursor que escreve as instruções
    let builder = context.create_builder();

    // --- Definindo a função main() ---
    // Tipo: fn() -> i32
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    
    // Adiciona a função ao módulo
    let function = module.add_function("main", fn_type, None);
    
    // Cria um Bloco Básico (entry) e move o builder para lá
    let basic_block = context.append_basic_block(function, "entry");
    builder.position_at_end(basic_block);

    // --- Gerando o corpo da função (AST -> IR) ---
    // Vamos simular: return 10 + 5
    
    let x = i32_type.const_int(10, false);
    let y = i32_type.const_int(5, false);
    
    // O nome "tmp_soma" aparece no IR para facilitar debug
    let soma = builder.build_int_add(x, y, "tmp_soma").unwrap(); 
    
    // Retorna o resultado
    builder.build_return(Some(&soma)).unwrap();

    // --- Saída ---
    // Imprime o IR gerado no console (stderr)
    module.print_to_stderr();
    
    // Ou salva em arquivo para ser compilado depois pelo clang
    // module.print_to_file("output.ll").unwrap();
}

// fn get_source_code_path() -> String {
//     let args = Args::parse();

//     match args.path {
//         Some(p) => return p,
//         None => {
//             eprintln!("É necessário passar o diretório do arquivo do código fonte");
//             eprintln!("Exemplo: --path \"./teste.por\"");
//             process::exit(1);
//         },
//     }
// }

// fn open_source_code_file(path: &str) -> BufReader<File> {

//     let file = File::open(path);

//     match file {
//         Ok (content) => {
//             return BufReader::new(content);
//         }
//         Err(err) => {
//             eprintln!("Erro ao ler arquivo!\nArquivo não encontrado em {}\nA extensão do arquivo esperado é '.por'\nMais detalhes: {}", path, err);
//             process::exit(1);
//         }
//     }
// }