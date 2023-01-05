use std::process::{Command, Output};
use std::io::{Read, Write};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    
    println!("[I] ARRANQUE");

    abre_twinkle_con_sh_inyectando_stdin();

    println!("[I] FIN");

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

    proc_twinkle.wait();

    println!("  [I] STDOUT ABRIR TWINKLE: {}", stdout_twinkle);

    println!("  [I] STDERR ABRIR TWINKLE: {}", stderr_twinkle);

}

fn abrir_twinkle_con_sh() {
    println!(" [I] ABRIENDO TWINKLE CON sh...");

    let mut proc_twinkle = Command::new("sh")
    .args(["-c", "twinkle -c -f /home/nsm/.twinkle/twinkle.cfg"])
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

    Command::new("sh")
    .args(["-c", "twinkle -c --cmd quit"])
    .spawn()
    .expect("[E] FALLA AL TRATAR DE CERRAR TWINKLE")
    .wait()
    .expect("[E] FALLA AL TRATAR DE CERRAR TWINKLE");

    proc_twinkle.wait();

    println!("  [I] STDOUT ABRIR TWINKLE: {}", stdout_twinkle);

    println!("  [I] STDERR ABRIR TWINKLE: {}", stderr_twinkle);
}

fn hacer_ls_con_sh() {
    println!(" [I] HACIENDO ls -alh CON sh...");

    let mut proc_twinkle = Command::new("bash")
    .args(["-c", "ls -alh"])
    .spawn()
    .expect("[E] FALLA AL HACER ls");

    sleep(Duration::from_secs(10));

    let mut stdout_twinkle = String::new();

    let stdout_child = match proc_twinkle.stdout.take() {
        Some(mut stdout) => stdout.read_to_string(&mut stdout_twinkle).expect("[E] FALLA AL LEER STDOUT DE sh"),
        None => {
            stdout_twinkle = "(stdout vacío)".to_string();
            0
        }
    };

    let mut stderr_twinkle = String::new();

    let stderr_child = match proc_twinkle.stderr.take() {
        Some(mut stderr) => stderr.read_to_string(&mut stderr_twinkle).expect("[E] FALLA AL LEER STDERR DE sh"),
        None => {
            stderr_twinkle = "(stderr vacío)".to_string();
            0
        }
    };

    proc_twinkle.wait();

    println!("  [I] STDOUT ls: {}", stdout_twinkle);

    println!("  [I] STDERR ls: {}", stderr_twinkle);
}

fn prueba_inyeccion_stdin() {

    let mut mi_shell = Command::new("sh")
    .arg("-m")
    .stdin(std::process::Stdio::piped())
    .stdout(std::process::Stdio::piped())
    .spawn()
    .expect("No abrí shell");

    let shell_stdin = mi_shell.stdin.as_mut().unwrap();

    shell_stdin.write(b"ls -alh\n").ok();

    sleep(Duration::from_secs(2));

    shell_stdin.write(b"exit\n").ok();

    let output = mi_shell.wait_with_output().expect("no se pudo leer stdout");

    let salida = String::from_utf8_lossy(&output.stdout);

    println!("Salida: {}", salida);

}

fn abre_twinkle_con_sh_inyectando_stdin() {

    println!("[I] Abriendo shell...");

    let mut mi_shell_1 = Command::new("sh")
    .arg("-m")
    .stdin(std::process::Stdio::piped())
    .stdout(std::process::Stdio::piped())
    .stderr(std::process::Stdio::piped())
    .spawn()
    .expect("No abrí shell");

    let shell_stdin_1 = mi_shell_1.stdin.as_mut().unwrap();

    println!(" [I] ABRIENDO TWINKLE...");

    shell_stdin_1.write(b"twinkle -c -f /home/nsm/twinkle.cfg >> /twinkle_stdout_test.txt\n").ok();

    println!(" [I] ESPERANDO A TWINKLE...");
    sleep(Duration::from_secs(30));

    println!(" [I] DANDO TWINKLE QUIT");
    shell_stdin_1.write(b"quit\n\n\n").ok();

    println!(" [I] ESPERANDO TWINKLE QUIT");
    sleep(Duration::from_secs(30));

    println!(" [I] DANDO sh exit");
    shell_stdin_1.write(b"exit\n").ok();

    println!(" [I] Esperando al child...");
    let out = mi_shell_1.wait_with_output().expect("no capturó stdout");

    let salida_string = String::from_utf8_lossy(&out.stdout);
    let err_string = String::from_utf8_lossy(&out.stderr);

    println!("SALIDA: {}\nERROR: {}", salida_string, err_string);

}