#[derive(Debug)]
struct Arguments {
    target: String,
    replacement: String,
    filename: String,
    output: String,
}

use text_colorizer::*;

fn print_usage() {
    eprintln!(
        "{} - alterar ocorrências de uma string para outra",
        "quickreplace".green()
    );

    eprintln!("Use: quickreplace <target> <replacement> <INPUT> <OUTPUT>");
}

use std::env;

fn parse_args() -> Arguments {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 4 {
        print_usage();
        eprintln!(
            "{} Numero de argumentos errado: esperado 4, veio {} ",
            "Error".red().bold(),
            args.len()
        );
        std::process::exit(1);
    }
    Arguments {
        target: args[0].clone(),
        replacement: args[1].clone(),
        filename: args[2].clone(),
        output: args[3].clone(),
    }
}

use std::fs;
use std::os::windows::process;

fn main() {
    let args = parse_args();

    copy_file(&args)

    // println!("{:?}", args);
}

fn copy_file(args: &Arguments) {
    let data = match fs::read_to_string(&args.filename) {
        Ok(v) => v,
        Err(e) => {
            eprint!(
                "{} falha ao ler arquivo de '{}' : {:?}",
                "Error".red().bold(),
                args.filename,
                e
            );
            std::process::exit(1);
        }
    };

    match fs::write(&args.output, &data) {
        Ok(_) => {}
        Err(e) => {
            eprintln!(
                "{} falha ao gravar o arquivo {}: {:?}",
                "Error".red().bold(),
                &args.filename,
                e
            );
            std::process::exit(1);
        }
    }
}