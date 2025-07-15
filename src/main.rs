use clap::{Parser, Subcommand};
use colored::*;
use commands::{install, update, upgrade, autoclean, autoremove, flatpak_install, flatpak_update, help};
use config::Config;
use utils::print_banner;
use std::path::Path;

mod commands;
mod config;
mod utils;

#[derive(Parser)]
#[command(name = "zenit", about = "Zenit Package Manager - Minimalistyczny wrapper dla Arch Linux", version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Instaluje pakiet przez pacman, yay lub Flatpak
    Install { package: String },
    /// Aktualizuje system (pacman i Flatpak)
    Update,
    /// Aktualizuje system i AUR (yay)
    Upgrade,
    /// Czyści cache pacman i Flatpak
    Autoclean,
    /// Usuwa osierocone paczki
    Autoremove,
    /// Instaluje pakiet Flatpak
    FlatpakInstall { package: String },
    /// Aktualizuje Flatpak
    FlatpakUpdate,
    /// Wyświetla pomoc
    #[command(name = "?")]
    Help,
}

#[tokio::main]
async fn main() {
    // Ładuj konfigurację
    let config_path = dirs::config_dir()
        .unwrap()
        .join("zenit")
        .join("config.toml");
    let config = if Path::new(&config_path).exists() {
        Config::load(&config_path).unwrap_or_else(|e| {
            eprintln!("{}", format!("⚠️ Błąd ładowania konfiguracji: {}", e).yellow());
            Config::default()
        })
    } else {
        Config::default()
    };

    // Wyświetl baner powitalny
    print_banner();

    let cli = Cli::parse();

    match cli.command {
        Commands::Install { package } => install(&package, &config).await,
        Commands::Update => update(&config).await,
        Commands::Upgrade => upgrade(&config).await,
        Commands::Autoclean => autoclean(&config).await,
        Commands::Autoremove => autoremove(&config).await,
        Commands::FlatpakInstall { package } => flatpak_install(&package, &config).await,
        Commands::FlatpakUpdate => flatpak_update(&config).await,
        Commands::Help => help(),
    }
}
