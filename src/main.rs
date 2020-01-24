extern crate ftp;

use ftp::FtpStream;
use std::fs::File;
use std::io::Write;

fn main() {
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
    println!("Download realizado. Transferindo dados para máquina local.");
    let mut buffer = File::create("/home/camilo/Documentos/rp/erp/teste/erp-Alfa.rar").unwrap();
    // Transfere os bytes da memória para a máquina local
    buffer.write_all(&remote_file.into_inner()).unwrap();
    // Encerra conexão com ftp server
    let _ = ftp_stream.quit();
    
    println!("Procedimento finalizado!");
}
