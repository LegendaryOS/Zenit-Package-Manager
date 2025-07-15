use colored::*;

pub fn print_banner() {
    println!("{}", r#"
    ╔════════════════════════════╗
    ║   Zenit Package Manager    ║
    ╚════════════════════════════╝
    "#.bold().cyan());
    println!("{}", "Witaj w Zenit!".bold().green());
    println!("{}", "Minimalistyczny i szybki menedżer pakietów dla Arch Linux.".italic());
    println!();
}
