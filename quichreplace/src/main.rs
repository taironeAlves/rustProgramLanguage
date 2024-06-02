use regex::Regex;
use std::env;
use std::fs;
use text_colorizer::*;

#[derive(Debug)]
struct Arguments {
    target: String,
    replacement: String,
    filename: String,
    output: String,
}

fn main() {
    let args: Arguments = parse_args();

    read_file(&args)

    // println!("{:?}", args);
}

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

fn print_usage() {
    eprintln!(
        "{} - alterar ocorrências de uma string para outra",
        "quickreplace".green()
    );

    eprintln!("Use: quickreplace <target> <replacement> <INPUT> <OUTPUT>");
}

fn read_file(args: &Arguments) {
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

    write_file(&args, &replace_file(&args, &data));
}

fn replace_file(args: &Arguments, data: &String) -> String {
    let resp = match replaces(&args.target, &args.replacement, &data) {
        Ok(v) => v,
        Err(e) => {
            eprintln!(
                "{} Falha ao sobrescrever o text: {:?}",
                "Error:".red().bold(),
                e
            );
            std::process::exit(1);
        }
    };

    resp
}

fn replaces(target: &str, replacement: &str, text: &str) -> Result<String, regex::Error> {
    let regex = Regex::new(target)?;
    Ok(regex.replace_all(text, replacement).to_string())
}

fn write_file(args: &Arguments, data: &String) {
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
