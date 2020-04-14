#[macro_use]
extern crate serde_derive;

use clap::{App, Arg};

mod config;
mod core;

enum Programs {
    Erp,
    ErpAlpha,
    RPServices,
}

fn download_flex(program: Programs) {
    println!("Atualização do ERP Flex iniciada...");
    let cfg = match config::get() {
        Ok(res) => res,
        Err(_) => config::Config::default(),
    };
    let dir_file = match program {
        Programs::ErpAlpha => cfg.erp_alpha.dir,
        Programs::Erp => cfg.erp.dir,
        _ => "/tmp".to_string(),
    };
    let name_file = match program {
        Programs::ErpAlpha => format!("{}erp-Alfa.rar", dir_file),
        Programs::Erp => format!("{}ERP.rar", dir_file),
        _ => "file".to_string(),
    };
    let remote_dir = match program {
        Programs::ErpAlpha => "install/erp/alpha_builds",
        Programs::Erp => "install/erp",
        _ => "/",
    };
    let remote_name_file = match program {
        Programs::ErpAlpha => "erp-Alfa.rar",
        Programs::Erp => "ERP.rar",
        _ => "file",
    };
    core::process_upgrade(
        &dir_file,
        &name_file,
        remote_dir,
        remote_name_file,
        core::TypeCompressFile::Rar,
    );
    println!("Atualização do ERP Flex concluída");
}

fn download_rpservices() {
    println!("Atualização do RPServices iniciada...");
    // JAVA_HOME=/usr/lib/jvm/java-8-openjdk-amd64/jre
    let cfg = match config::get() {
        Ok(res) => res,
        Err(_) => config::Config::default(),
    };
    let dir_file = cfg.rp_services.dir;
    let remote_dir = "install/erp";
    // Lista de arquivos no ftp do diretório
    let list_files = core::list_ftp_files(remote_dir).unwrap();
    // Retorna elemento correspondente a rpservices
    let mut filtered_list = list_files
        .iter()
        .filter(|line| line.contains(&"rpservices".to_string()));
    // Filtra apenas o nome do arquivo na string
    let remote_name_file = filtered_list
        .next()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap();
    // Cria string contendo diretório e nome do arquivo
    let name_file = format!("{}{}", dir_file, remote_name_file);
    core::process_upgrade(
        &dir_file,
        &name_file,
        remote_dir,
        remote_name_file,
        core::TypeCompressFile::Zip,
    );
    println!("Atualização do RPServices concluída");
}

fn main() {
    println!("Iniciando procedimento...");

    let matches = App::new("ftpclientrp")
        .version("0.1.0")
        .bin_name("ftpclientrp")
        .arg(
            Arg::with_name("program")
                .long("program")
                .short("p")
                .required(true)
                .takes_value(true)
                .possible_values(&["erp", "erp_alpha", "rp_services"]),
        )
        .arg(Arg::with_name("config").short("c").takes_value(false))
        .get_matches();

    if let Some(program) = matches.value_of("program") {
        match program {
            "erp" => download_flex(Programs::Erp),
            "erp_alpha" => download_flex(Programs::ErpAlpha),
            "rp_services" => download_rpservices(),
            _ => println!("None"),
        }
    }

    println!("Procedimento finalizado!");
}
