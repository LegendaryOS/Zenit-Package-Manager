use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::process::Command;
use std::thread;
use std::time::Duration;
use tokio::process::Command as TokioCommand;
use crate::config::Config;

pub async fn install(pkg: &str, config: &Config) {
    println!("{}", format!("Instalacja pakietu: {}", pkg).bold().cyan());

    let pb = create_progress_bar(&config.settings.progress_style, String::from("Instalacja"));

    for i in 0..=100 {
        pb.set_position(i);
        thread::sleep(Duration::from_millis(30));
    }

    let manager = &config.settings.default_manager;
    let confirm_flag = if config.settings.confirm { "" } else { "--noconfirm" };

    let status = match manager.as_str() {
        "pacman" => {
            let pacman_result = TokioCommand::new("sudo")
            .arg("pacman")
            .arg("-S")
            .arg(confirm_flag)
            .arg(pkg)
            .status()
            .await;

            if let Ok(pacman_status) = pacman_result {
                if pacman_status.success() {
                    Ok(pacman_status)
                } else {
                    println!("{}", "Próba instalacji przez yay...".yellow());
                    TokioCommand::new("yay")
                    .arg("-S")
                    .arg(confirm_flag)
                    .arg(pkg)
                    .status()
                    .await
                }
            } else {
                pacman_result
            }
        }
        "yay" => TokioCommand::new("yay")
        .arg("-S")
        .arg(confirm_flag)
        .arg(pkg)
        .status()
        .await,
        _ => {
            eprintln!("{}", "Nieobsługiwany menedżer pakietów!".red());
            return;
        }
    };

    match status {
        Ok(status) if status.success() => {
            pb.finish_with_message(format!("{}", "Zakończono!".green()));
            println!("{}", format!("Pakiet {} zainstalowany pomyślnie!", pkg).bold().green());
        }
        _ => {
            pb.abandon_with_message(format!("{}", "Błąd!".red()));
            eprintln!("{}", format!("Nie udało się zainstalować pakietu {}", pkg).bold().red());
        }
    }
}

pub async fn update(config: &Config) {
    println!("{}", "Aktualizacja systemu...".bold().cyan());

    let pb = create_progress_bar(&config.settings.progress_style, String::from("Aktualizacja pacman"));

    for i in 0..=100 {
        pb.set_position(i);
        thread::sleep(Duration::from_millis(50));
    }

    let confirm_flag = if config.settings.confirm { "" } else { "--noconfirm" };

    let status = TokioCommand::new("sudo")
    .arg("pacman")
    .arg("-Syu")
    .arg(confirm_flag)
    .status()
    .await;

    match status {
        Ok(status) if status.success() => {
            pb.finish_with_message(format!("{}", "Zakończono!".green()));
            println!("{}", "System zaktualizowany pomyślnie!".bold().green());
        }
        _ => {
            pb.abandon_with_message(format!("{}", "Błąd!".red()));
            eprintln!("{}", "Nie udało się zaktualizować systemu!".bold().red());
        }
    }

    flatpak_update(config).await;
}

pub async fn upgrade(config: &Config) {
    println!("{}", "Aktualizacja systemu i AUR...".bold().cyan());

    let pb = create_progress_bar(&config.settings.progress_style, String::from("Aktualizacja yay"));

    for i in 0..=100 {
        pb.set_position(i);
        thread::sleep(Duration::from_millis(60));
    }

    let confirm_flag = if config.settings.confirm { "" } else { "--noconfirm" };

    let status = TokioCommand::new("yay")
    .arg("-Syu")
    .arg("--devel")
    .arg("--timeupdate")
    .arg(confirm_flag)
    .status()
    .await;

    match status {
        Ok(status) if status.success() => {
            pb.finish_with_message(format!("{}", "Zakończono!".green()));
            println!("{}", "System i AUR zaktualizowane pomyślnie!".bold().green());
        }
        _ => {
            pb.abandon_with_message(format!("{}", "Błąd!".red()));
            eprintln!("{}", "Nie udało się zaktualizować systemu i AUR!".bold().red());
        }
    }
}

pub async fn autoclean(config: &Config) {
    println!("{}", "Czyszczenie cache...".bold().cyan());

    let pb = create_progress_bar(&config.settings.progress_style, String::from("Czyszczenie cache"));

    for i in 0..=100 {
        pb.set_position(i);
        thread::sleep(Duration::from_millis(40));
    }

    let confirm_flag = if config.settings.confirm { "" } else { "--noconfirm" };

    let pacman_status = TokioCommand::new("sudo")
    .arg("pacman")
    .arg("-Sc")
    .arg(confirm_flag)
    .status()
    .await;

    let flatpak_status = TokioCommand::new("flatpak")
    .arg("uninstall")
    .arg("--unused")
    .status()
    .await;

    if pacman_status.is_ok() && pacman_status.unwrap().success() && flatpak_status.is_ok() && flatpak_status.unwrap().success() {
        pb.finish_with_message(format!("{}", "Zakończono!".green()));
        println!("{}", "Cache wyczyszczony pomyślnie!".bold().green());
    } else {
        pb.abandon_with_message(format!("{}", "Błąd!".red()));
        eprintln!("{}", "Nie udało się wyczyścić cache!".bold().red());
    }
}

pub async fn autoremove(config: &Config) {
    println!("{}", "Usuwanie osieroconych pakietów...".bold().cyan());

    let pb = create_progress_bar(&config.settings.progress_style, String::from("Usuwanie osieroconych"));

    for i in 0..=100 {
        pb.set_position(i);
        thread::sleep(Duration::from_millis(40));
    }

    let confirm_flag = if config.settings.confirm { "" } else { "--noconfirm" };

    let output = Command::new("pacman")
    .arg("-Qdtq")
    .output()
    .expect("Błąd podczas wyszukiwania osieroconych pakietów");

    let packages = String::from_utf8_lossy(&output.stdout);
    if packages.is_empty() {
        pb.finish_with_message(format!("{}", "Brak osieroconych!".green()));
        println!("{}", "Nie znaleziono osieroconych pakietów!".bold().green());
        return;
    }

    let status = TokioCommand::new("sudo")
    .arg("pacman")
    .arg("-Rns")
    .arg(confirm_flag)
    .arg(packages.trim())
    .status()
    .await;

    match status {
        Ok(status) if status.success() => {
            pb.finish_with_message(format!("{}", "Zakończono!".green()));
            println!("{}", "Osierocone pakiety usunięte pomyślnie!".bold().green());
        }
        _ => {
            pb.abandon_with_message(format!("{}", "Błąd!".red()));
            eprintln!("{}", "Nie udało się usunąć osieroconych pakietów!".bold().red());
        }
    }
}

pub async fn flatpak_install(pkg: &str, config: &Config) {
    println!("{}", format!("Instalacja pakietu Flatpak: {}", pkg).bold().cyan());

    let pb = create_progress_bar(&config.settings.progress_style, String::from("Instalacja Flatpak"));

    for i in 0..=100 {
        pb.set_position(i);
        thread::sleep(Duration::from_millis(30));
    }

    let status = TokioCommand::new("flatpak")
    .arg("install")
    .arg(pkg)
    .status()
    .await;

    match status {
        Ok(status) if status.success() => {
            pb.finish_with_message(format!("{}", "Zakończono!".green()));
            println!("{}", format!("Pakiet Flatpak {} zainstalowany pomyślnie!", pkg).bold().green());
        }
        _ => {
            pb.abandon_with_message(format!("{}", "Błąd!".red()));
            eprintln!("{}", format!("Nie udało się zainstalować pakietu Flatpak {}", pkg).bold().red());
        }
    }
}

pub async fn flatpak_update(config: &Config) {
    println!("{}", "Aktualizacja Flatpak...".bold().cyan());

    let pb = create_progress_bar(&config.settings.progress_style, String::from("Aktualizacja Flatpak"));

    for i in 0..=100 {
        pb.set_position(i);
        thread::sleep(Duration::from_millis(50));
    }

    let status = TokioCommand::new("flatpak")
    .arg("update")
    .status()
    .await;

    match status {
        Ok(status) if status.success() => {
            pb.finish_with_message(format!("{}", "Zakończono!".green()));
            println!("{}", "Flatpak zaktualizowany pomyślnie!".bold().green());
        }
        _ => {
            pb.abandon_with_message(format!("{}", "Błąd!".red()));
            eprintln!("{}", "Nie udało się zaktualizować Flatpak!".bold().red());
        }
    }
}

pub fn help() {
    println!("{}", "Pomoc dla Zenit Package Manager".bold().yellow());
    println!("{}", r#"
    ╔════════════════════════════════════╗
    ║   Dostępne komendy                 ║
    ╚════════════════════════════════════╝
    "#.bold().cyan());
    println!("  {} install <pakiet>         - Instaluje pakiet (pacman/yay/flatpak)", "zenit".bold());
    println!("  {} update                 - Aktualizuje system (pacman i Flatpak)", "zenit".bold());
    println!("  {} upgrade                - Aktualizuje system i AUR (yay)", "zenit".bold());
    println!("  {} autoclean              - Czyści cache pacman i Flatpak", "zenit".bold());
    println!("  {} autoremove             - Usuwa osierocone paczki", "zenit".bold());
    println!("  {} flatpak install <pakiet> - Instaluje pakiet Flatpak", "zenit".bold());
    println!("  {} flatpak update         - Aktualizuje Flatpak", "zenit".bold());
    println!("  {} ?                      - Wyświetla tę pomoc", "zenit".bold());
    println!();
    println!("{}", "Konfiguracja: ~/.config/zenit/config.toml".italic());
}

fn create_progress_bar(style: &str, prefix: String) -> ProgressBar {
    let pb = ProgressBar::new(100);
    let template = match style {
        "simple" => {
            let simple_template = format!("{} [{{bar:20}}] {{percent}}% {{msg}}", prefix);
            simple_template
        }
        _ => {
            let fancy_template = format!("{} [{{bar:40.cyan/blue}}] {{percent}}% {{msg}}", prefix);
            fancy_template
        }
    };
    pb.set_style(
        ProgressStyle::default_bar()
        .template(&template)
        .unwrap()
        .progress_chars("=>-"),
    );
    pb.set_prefix(prefix);
    pb
}
