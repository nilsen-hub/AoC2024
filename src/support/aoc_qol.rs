use std::env;

pub fn clear_terminal() {
    if env::consts::OS == "windows" {
        std::process::Command::new("cmd")
            .args(&["/c", "cls"])
            .spawn()
            .expect("cls command failed to start")
            .wait()
            .expect("failed to wait");
    } else {
        std::process::Command::new("clear").status().unwrap();
    }
}
