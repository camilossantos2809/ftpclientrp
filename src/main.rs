mod core;

fn download_flex() {
    println!("Atualização do ERP Flex iniciada...");
    let dir_file = "/home/camilo/Documentos/rp/erp/";
    let name_file = format!("{}erp-Alfa.rar", dir_file);
    let remote_dir = "install/erp/alpha_builds";
    let remote_name_file = "erp-Alfa.rar";
    core::process_upgrade(
        dir_file,
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
    let dir_file = "/home/camilo/Documentos/rp/";
    let name_file = format!("{}rpservices-2.0.0.67.zip", dir_file);
    let remote_dir = "install/erp";
    let remote_name_file = "rpservices-2.0.0.67.zip";
    core::process_upgrade(
        dir_file,
        &name_file,
        remote_dir,
        remote_name_file,
        core::TypeCompressFile::Zip,
    );
    println!("Atualização do RPServices concluída");
}

fn main() {
    println!("Iniciando procedimento...");
    download_flex();
    // download_rpservices();
    println!("Procedimento finalizado!");
}
