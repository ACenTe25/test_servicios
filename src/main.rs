use std::process::{Command, Output};
use std::io::Read;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    
    println!("[I] ARRANQUE");

    abrir_twinkle();

}

fn abrir_twinkle() {

    println!(" [I] ABRIENDO TWINKLE...");

    let mut proc_twinkle = Command::new("twinkle")
    .args(["-c", "-f", "/home/nsm/.twinkle/twinkle.cfg"])
    .spawn()
    .expect("[E] FALLA AL ABRIR TWINKLE");

    sleep(Duration::from_secs(10));

    let mut stdout_twinkle = String::new();

    let stdout_child = match proc_twinkle.stdout.take() {
        Some(mut stdout) => stdout.read_to_string(&mut stdout_twinkle).expect("[E] FALLA AL LEER STDOUT DE ABRIR TWINKLE"),
        None => {
            stdout_twinkle = "(stdout vacío)".to_string();
            0
        }
    };

    let mut stderr_twinkle = String::new();

    let stderr_child = match proc_twinkle.stderr.take() {
        Some(mut stderr) => stderr.read_to_string(&mut stderr_twinkle).expect("[E] FALLA AL LEER STDERR DE ABRIR TWINKLE"),
        None => {
            stderr_twinkle = "(stderr vacío)".to_string();
            0
        }
    };

    println!(" [I] CERRANDO TWINKLE...");

    Command::new("twinkle")
    .args(["-c", "--cmd", "quit"])
    .spawn()
    .expect("[E] FALLA AL TRATAR DE CERRAR TWINKLE")
    .wait()
    .expect("[E] FALLA AL TRATAR DE CERRAR TWINKLE");

    println!("  [I] STDOUT ABRIR TWINKLE: {}", stdout_twinkle);

    println!("  [I] STDERR ABRIR TWINKLE: {}", stderr_twinkle);

}
