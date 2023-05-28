
/* 
./kvs --help
A key-vaule store

Usage: kvs [COMMAND]

Commands:
  get   get a vaule from a key: get [key]
  set   set a key/vaule pair: set [key] [vaule]
  rm    remove the a key/vaule pair: rm [key]
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help information
  -V, --version  Print version information

*/

/*** builder ***/
use std::{env, process};
use clap::{arg, command, Command};


fn main() {
    let matches = command!() // requires `cargo` feature
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
                Command::new("get")
                .about("get a vaule from a key: get [key]")
                .arg(arg!([KEY]).help("A String key").required(true)),
                
        )
        .subcommand(
            Command::new("set")
                .about("set a key/vaule pair: set [key] [vaule]")
                .arg(arg!([KEY]).help("A String key").required(true))
                .arg(arg!([VALUE]).help("A String vaule").required(true)), 
        )
        .subcommand(
            Command::new("rm")
                .about("remove the a key/vaule pair: rm [key]")
                .arg(arg!([KEY]).help("A String key").required(true)), 
        )
        .get_matches();
    

    match matches.subcommand() {
        Some(("get", _matches)) => {
            println!("KEY{:?}", _matches.get_one::<String>("KEY").unwrap());
            eprintln!("unimplemented");
            process::exit(-1)
        },
        Some(("set", _matches)) => {
            println!(
                "KEY{:?}, VALUE{:?}",
                _matches.get_one::<String>("KEY").unwrap(),
                _matches.get_one::<String>("VALUE").unwrap()
            );
            eprintln!("unimplemented");
            process::exit(-1)

        },
        Some(("rm", _matches)) => {
            println!("KEY{:?}", _matches.get_one::<String>("KEY").unwrap());
            eprintln!("unimplemented");
            process::exit(-1)
        },
        _ => process::exit(-1),
    }
}