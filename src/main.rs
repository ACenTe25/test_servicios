use std::process::{Command, Stdio};
use std::io::{Read, Write};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    
    println!("[I] ARRANQUE");

    llama_con_baresip();

    println!("[I] FIN");

}

fn llama_con_baresip() {

    println!(" [I] ABRIENDO BARESIP...");

    let mut baresip = Command::new("baresip")
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()
    .expect("[E] ERROR AL LLAMAR BARESIP");

    println!("  [I] BARESIP ABIERTO. LLAMANDO...");
    let stdin_baresip = baresip.stdin.as_mut().unwrap();

    stdin_baresip.write(b"9184\n").expect("[E] ERROR AL ENVIAR POR STDIN BARESIP");

    println!("   [I] LLAMADA COLOCADA. ESPERANDO...");
    sleep(Duration::from_secs(10));

    println!("   [I] ESPERA TERMINADA. COLGANDO...");
    stdin_baresip.write(b"b\n").expect("[E] ERROR AL ENVIAR POR STDIN BARESIP");
    sleep(Duration::from_secs(1));

    println!("   [I] COLGADO. CERRANDO BARESIP...");
    stdin_baresip.write(b"q\n").expect("[E] ERROR AL ENVIAR POR STDIN BARESIP");

    let salida_baresip = baresip.wait_with_output().expect("[E] ERROR AL TERMINAR BARESIP");
    let stdout_baresip = String::from_utf8_lossy(&salida_baresip.stdout);
    let stderr_baresip = String::from_utf8_lossy(&salida_baresip.stderr);

    println!("[I] STDOUT BARESIP:\n{}\n\n[I] STDERR BARESIP:\n{}\n", stdout_baresip, stderr_baresip);

}