extern crate ftp;
extern crate unrar;

use ftp::{FtpError, FtpStream};
use std::fs::{create_dir_all, File};
use std::io;
use std::io::{Cursor, Write};
use std::vec::Vec;
use unrar::Archive;
use zip::ZipArchive;

pub enum TypeCompressFile {
    Rar,
    Zip,
}

fn write_file(file: &str, remote_file: Cursor<Vec<u8>>) {
    println!("Transferindo dados para máquina local...");
    // Diretório/arquivo local que será escrito com os bytes do arquivo do ftp server
    let mut buffer = File::create(file).unwrap();
    // Transfere os bytes da memória para a máquina local
    buffer.write_all(&remote_file.into_inner()).unwrap();
    println!("Arquivo gravado: {}", file);
}

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

pub fn list_ftp_files(dir: &str) -> Result<Vec<String>, FtpError> {
    // Conecta ao ftp server
    let mut ftp_stream = FtpStream::connect("ftp.rpinfo.com.br:21").unwrap();
    // Realiza login
    let _ = ftp_stream.login("supermercado", "rpsupermercado10");
    let list = ftp_stream.list(Some(dir));
    list
}

fn extract_zip_file(file_path: String, dir: &str) {
    println!("Descompactando arquivo...");
    let file = File::open(&file_path).unwrap();
    let mut zip_file = ZipArchive::new(file).unwrap();
    for i in 0..zip_file.len() {
        let mut loop_file = zip_file.by_index(i).unwrap();
        if loop_file.size() > 0 {
            let file = format!("{}{}", &dir, loop_file.name());
            let mut buffer = File::create(file).unwrap();
            io::copy(&mut loop_file, &mut buffer).unwrap();
        } else {
            create_dir_all(format!(
                "{}{}",
                dir,
                loop_file.sanitized_name().as_path().display()
            ))
            .unwrap();
        }
    }
    println!("Arquivo descompactado");
}

fn extract_rar_file(file: String, dir: &str) {
    println!("Descompactando arquivo...");
    Archive::new(file)
        .extract_to(dir.to_string())
        .unwrap()
        .process()
        .unwrap();
    println!("Arquivo descompactado");
}

pub fn process_upgrade(
    dir_file: &str,
    name_file: &str,
    remote_dir: &str,
    remote_name_file: &str,
    type_compress: TypeCompressFile,
) {
    let remote_file = download_ftp(remote_dir, remote_name_file);
    write_file(&name_file, remote_file);
    match type_compress {
        TypeCompressFile::Rar => extract_rar_file(name_file.to_string(), dir_file),
        TypeCompressFile::Zip => extract_zip_file(name_file.to_string(), dir_file),
    }
}
