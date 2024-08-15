use rusty_socket_client::SocketClient;
use std::io::{self, Read, Write};
mod commands;
use commands::Commands;

const COMMANDS: Commands = Commands(&[
    ("connect", "Connect to the server"),
    ("__disconnect", "Disconnect from the server"),
    ("exit", "Exit the program"),
    ("help", "Prints usage guide"),
]);

fn main() {
    println!("---<Command Line Chat Initialized!>---");
    // let mut is_connected: bool = false;
    let mut socket_client: Option<SocketClient> = None;
    let stdin = io::stdin();

    screen_init(false);

    loop {
        let mut input = String::new();
        let _ = stdin.read_line(&mut input);

        let sanitized_input = input.trim();

        if sanitized_input.is_empty() {
            match socket_client {
                Some(_) => screen_init(true),
                None => screen_init(false),
            }
            continue;
        }

        match socket_client {
            None => {
                match sanitized_input {
                    "help" => {
                        print!("{}", COMMANDS);
                        screen_init(false);
                        continue;
                    }
                    "connect" => {
                        match SocketClient::build("ws://127.0.0.1:8080") {
                            Ok(client) => {
                                socket_client = Some(client);
                                println!("Connected successfully.");
                                screen_init(true)
                            }
                            Err(e) => {
                                println!("Connection failed: ({})", e);
                                screen_init(false);
                            }
                        }
                        continue;
                    }
                    "exit" => break,
                    _ => {
                        println!("Command not recognized!");
                        println!("{}", COMMANDS);
                        screen_init(false);
                        continue;
                    }
                }
            }
            Some(ref mut client) => {
                match sanitized_input {
                    "__disconnect" => {
                        client.close().unwrap();
                        socket_client = None;
                        println!("Disconnected successfully!");
                        screen_init(false);
                    },
                    message => {
                        match client.send(message) {
                            Ok(_) => (),
                            Err(e) => println!("Error while sending: {}", e),
                        }
                        screen_init(true);
                        continue;
                    }
                }
            }
        }
    }

    println!("Exiting Program!");
}

fn screen_init(is_connected: bool) {
    match is_connected {
        false => {
            print!("> ");
        }
        true => {
            print!("msg> ");
        }
    }
    io::stdout().flush().unwrap();
}
