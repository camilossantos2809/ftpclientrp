extern crate ftp;
extern crate unrar;

use ftp::FtpStream;
use std::fs::File;
use std::io::Write;
use unrar::Archive;

fn main() {
    let dir_file = "/home/camilo/Documentos/rp/erp/";
    let name_file = "/home/camilo/Documentos/rp/erp/erp-Alfa.rar";

    // Conecta ao ftp server 
    let mut ftp_stream = FtpStream::connect("ftp.rpinfo.com.br:21").unwrap();
    // Realiza login
    let _ = ftp_stream.login("supermercado","rpsupermercado10").unwrap();
    // Acessa o diretório
    let _ = ftp_stream.cwd("install/erp/alpha_builds").unwrap();
    // Retorna o diretório atual
    println!("Realizando download do arquivo {}/erp-Alfa.rar...", ftp_stream.pwd().unwrap());
    // Retorna cursor com bytes do arquivo existente no ftp server
    let remote_file = ftp_stream.simple_retr("erp-Alfa.rar").unwrap();
    // Diretório/arquivo local que será escrito com os bytes do arquivo do ftp server
    println!("Download realizado. Transferindo dados para máquina local...");
    let mut buffer = File::create(name_file).unwrap();
    // Transfere os bytes da memória para a máquina local
    buffer.write_all(&remote_file.into_inner()).unwrap();
    // Encerra conexão com ftp server
    let _ = ftp_stream.quit();

    println!("Descompactando arquivo...");
    Archive::new(name_file.to_string()).extract_to(dir_file.to_string()).unwrap().process().unwrap();
    
    println!("Procedimento finalizado!");
}
