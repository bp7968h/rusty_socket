use std::collections::HashMap;
use std::fmt;
use rusty_socket_client::SocketClient;
use std::io::{self, Read, Write};
mod commands;
use commands::Commands;

const COMMANDS: Commands = Commands(&[
    ("connect", "Connect to the server"),
    ("disconnect", "Disconnect from the server"),
    ("exit", "Exit the program"),
    ("help", "Prints usage guide"),
]);


fn main() {
    println!("---<Command Line Chat Initialized!>---");
    let mut is_connected: bool = false;
    let stdin = io::stdin();

    screen_init(is_connected);

    loop {
        let mut input = String::new();
        let _ = stdin.read_line(&mut input);

        let sanitized_input = input.trim();

        if sanitized_input.is_empty() {
            screen_init(is_connected);
            continue;
        }

        match is_connected {
            false => {
                match sanitized_input {
                    "help" => {
                        print!("{}", COMMANDS);
                        screen_init(is_connected);
                        continue;
                    },
                    "connect" => {
                        todo!();
                    },
                    "disconnect" => {
                        todo!();
                    },
                    "exit" => break,
                    _ => {
                        println!("Command not recognized!");
                        println!("{}", COMMANDS);
                        screen_init(is_connected);
                        continue;
                    }
                }
            },
            true => {
                match sanitized_input {
                    "disconnect" => {
                        todo!();
                    },
                    _ => {
                        // send message;
                        todo!()
                    }
                }
            }
        }
    }
}

fn screen_init(is_connected: bool) {
    match is_connected {
        false => {
            print!("> ");
        },
        true => {
            print!("msg> ");
        }
    }
    io::stdout().flush().unwrap();
}