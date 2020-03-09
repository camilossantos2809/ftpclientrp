extern crate ftp;
extern crate unrar;

use ftp::FtpStream;
use std::fs::File;
use std::io::{Cursor, Write};
use std::vec::Vec;
use unrar::Archive;

fn download_ftp(dir: &str, file: &str) -> Cursor<Vec<u8>> {
    // Conecta ao ftp server
    let mut ftp_stream = FtpStream::connect("ftp.rpinfo.com.br:21").unwrap();
    // Realiza login
    let _ = ftp_stream.login("supermercado", "rpsupermercado10");
    // Acessa o diretório
    let _ = ftp_stream.cwd(dir);
    // Retorna o diretório atual
    println!(
        "Realizando download do arquivo {}/{}...",
        ftp_stream.pwd().unwrap(),
        file
    );
    // Retorna cursor com bytes do arquivo existente no ftp server
    let remote_file = ftp_stream.simple_retr(file).unwrap();
    // Encerra conexão com ftp server
    let _ = ftp_stream.quit();
    remote_file
}

fn write_file(file: &str, remote_file: Cursor<Vec<u8>>) {
    println!("Transferindo dados para máquina local...");
    // Diretório/arquivo local que será escrito com os bytes do arquivo do ftp server
    let mut buffer = File::create(file).unwrap();
    // Transfere os bytes da memória para a máquina local
    buffer.write_all(&remote_file.into_inner()).unwrap();
    println!("Arquivo gravado: {}", file);
}

fn extract_file(file: String, dir: &str) {
    println!("Descompactando arquivo...");
    Archive::new(file)
        .extract_to(dir.to_string())
        .unwrap()
        .process()
        .unwrap();
    println!("Arquivo descompactado");
}

fn process_upgrade(dir_file: &str, name_file: &str, remote_dir: &str, remote_name_file: &str) {
    let remote_file = download_ftp(remote_dir, remote_name_file);
    write_file(&name_file, remote_file);
    extract_file(name_file.to_string(), dir_file);
}

fn download_flex() {
    println!("Atualização do ERP Flex iniciada...");
    let dir_file = "/home/camilo/Documentos/rp/erp/";
    let name_file = format!("{}erp-Alfa.rar", dir_file);
    let remote_dir = "install/erp/alpha_builds";
    let remote_name_file = "erp-Alfa.rar";
    process_upgrade(dir_file, &name_file, remote_dir, remote_name_file);
    println!("Atualização do ERP Flex concluída");
}

fn main() {
    println!("Iniciando procedimento...");
    download_flex();
    println!("Procedimento finalizado!");
}
