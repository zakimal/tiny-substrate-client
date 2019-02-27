extern crate websocket;
extern crate serde;
extern crate serde_json;

use serde_json::json;
use colored::*;
use std::io::stdin;
use serde_json::Value;
use std::sync::mpsc::channel;
use std::thread;

use websocket::client::ClientBuilder;
use websocket::{Message, OwnedMessage};

const CONNECTION: &'static str = "ws://127.0.0.1:9944";

fn main() {
    let mut counter = 1;
    println!("Connecting to {}", CONNECTION);
    let client = ClientBuilder::new(CONNECTION)
        .unwrap()
        .add_protocol("rust-websocket")
        .connect_insecure()
        .unwrap();
    println!("Successfully connected");
    let (mut receiver, mut sender) = client.split().unwrap();
    let (tx, rx) = channel();
    let tx_1 = tx.clone();
    let send_loop = thread::spawn(move || {
        loop {
            // Send loop
            let message = match rx.recv() {
                Ok(m) => m,
                Err(e) => {
                    println!("Send Loop: {:?}", e);
                    return;
                }
            };
            match message {
                OwnedMessage::Close(_) => {
                    let _ = sender.send_message(&message);
                    // If it's a close message, just send it and then return.
                    return;
                }
                _ => (),
            }
            // Send the message
            match sender.send_message(&message) {
                Ok(()) => (),
                Err(e) => {
                    println!("Send Loop: {:?}", e);
                    let _ = sender.send_message(&Message::close());
                    return;
                }
            }
        }
    });

    let receive_loop = thread::spawn(move || {
        // Receive loop
        for message in receiver.incoming_messages() {
            let message = match message {
                Ok(m) => m,
                Err(e) => {
                    println!("Receive Loop: {:?}", e);
                    let _ = tx_1.send(OwnedMessage::Close(None));
                    return;
                }
            };
            match message {
                OwnedMessage::Close(_) => {
                    // Got a close message, so send a close message and return
                    let _ = tx_1.send(OwnedMessage::Close(None));
                    return;
                }
                OwnedMessage::Ping(data) => {
                    match tx_1.send(OwnedMessage::Pong(data)) {
                        // Send a pong in response
                        Ok(()) => (),
                        Err(e) => {
                            println!("Receive Loop: {:?}", e);
                            return;
                        }
                    }
                }
                // Say what we received
                _ => {
                    match message {
                        OwnedMessage::Text(message) => {
                            let obj: Value = serde_json::from_str(&message)
                                .expect("Error: Could not parse json");
                            println!("{}", serde_json::to_string_pretty(&obj).expect("Error: Could not parse json").bold());
                        },
                        OwnedMessage::Binary(message) => {
                            println!("{:?}", message)
                        },
                        _ => (),
                    }
                },
            }
        }
    });
    println!(r#"

████████╗██╗███╗   ██╗██╗   ██╗    ███████╗██╗   ██╗██████╗ ███████╗████████╗██████╗  █████╗ ████████╗███████╗     ██████╗██╗     ██╗███████╗███╗   ██╗████████╗
╚══██╔══╝██║████╗  ██║╚██╗ ██╔╝    ██╔════╝██║   ██║██╔══██╗██╔════╝╚══██╔══╝██╔══██╗██╔══██╗╚══██╔══╝██╔════╝    ██╔════╝██║     ██║██╔════╝████╗  ██║╚══██╔══╝
   ██║   ██║██╔██╗ ██║ ╚████╔╝     ███████╗██║   ██║██████╔╝███████╗   ██║   ██████╔╝███████║   ██║   █████╗      ██║     ██║     ██║█████╗  ██╔██╗ ██║   ██║
   ██║   ██║██║╚██╗██║  ╚██╔╝      ╚════██║██║   ██║██╔══██╗╚════██║   ██║   ██╔══██╗██╔══██║   ██║   ██╔══╝      ██║     ██║     ██║██╔══╝  ██║╚██╗██║   ██║
   ██║   ██║██║ ╚████║   ██║       ███████║╚██████╔╝██████╔╝███████║   ██║   ██║  ██║██║  ██║   ██║   ███████╗    ╚██████╗███████╗██║███████╗██║ ╚████║   ██║
   ╚═╝   ╚═╝╚═╝  ╚═══╝   ╚═╝       ╚══════╝ ╚═════╝ ╚═════╝ ╚══════╝   ╚═╝   ╚═╝  ╚═╝╚═╝  ╚═╝   ╚═╝   ╚══════╝     ╚═════╝╚══════╝╚═╝╚══════╝╚═╝  ╚═══╝   ╚═╝

COMMANDS:
    /close
        Close the socket to the substrate chain.
    /ping
        Ping
    /help
        Show this message
    <SYSTEM>
    /system_name
        Return the name of this chain
    /system_version
        Return the version of this chain
    /system_chain
        Return the chain's level (?)
    /system_properties
        Return some properties like `isSyncing`, `peers`, `shouldHavePeers`
    /system_health
        Return the chain's health
    /system_peers
        Return the list of peers

    <STATE>
    /state_call <name> <bytes> <hash>
        Call a contract at a block's state.
    /state_callAt <name> <bytes> <hash>
        Same above.
    /state_getKeys <key> <hash>
        Returns the keys with prefix, leave empty to get all the keys
    /state_getStorage <key> <hash>
        Returns a storage entry at a specific block's state.
    /state_getStorageAt <key> <hash>
        Same above.
    /state_getStorageHash <key> <hash>
        Returns the hash of a storage entry at a block's state.
    /state_getStorageHashAt <key> <hash>
        Same above.
    /state_getMetadata <hash>
        Return the runtime metadata as an opaque blob.
    /state_getRuntimeVersion <hash>
        Get the runtime version.
    /chain_getRuntimeVersion <hash>
        Same above.
    /state_queryStorage <key>, <key>..., <block>, <hash>
        Query historical storage entries (by key) starting from a block given as the second parameter.
    /state_subscribeRuntimeVersion <metadata> <subscriber>
        New runtime version subscription.
    /chain_subscribeRuntimeVersion <metadata> <subscriber>
        Same above.
    /state_unsubscribeRuntimeVersion <metadata> <subscription id>
        Unsubscribe from runtime version subscription.
    /chain_unsubscribeRuntimeVersion <metadata> <subscription id>
        Same above.
    /state_subscribeStorage <metadata> <name>
        New storage subscription.
    state_unsubscribeStorage <metadata> <subscription id>
        Unsubscribe from storage subscription.

    <CHAIN>
    /chain_getHeader <hash>
        Get header of a relay chain block.
    /chain_getBlock <hash>
        Get header and body of a relay chain block.
    /chain_getBlockHash <number>
        Get hash of the n-th block in the canon chain.
        By default returns latest block hash.
    /chain_getHead <number>
        Same above.
    /chain_getFinalisedHead
        Get hash of the last finalised block in the canon chain.
    /chain_subscribeNewHead <metadata> <subscriber>
        New head subscription.
    /subscribe_newHead <metadata> <subscriber>
        Same above.
    /chain_unsubscribeNewHead <metadata> <subscription id>
        Unsubscribe from new head subscription.
    /unsubscribe_newHead <metadata> <subscription id>
        Same above.
    /chain_subscribeFinalisedHeads <metadata> <subscriber>
        New head subscription.
    /chain_unsubscribeFinalisedHeads <metadata> <subscription id>
        Unsubscribe from new head subscription.

    <AHTHOR>
    /author_submitExtrinsic <extrinsic>
        Submit hex-encoded extrinsic for inclusion in block.
    /author_pendingExtrinsics
        Returns all pending extrinsics, potentially grouped by sender.
    /author_submitAndWatchExtrinsic <metadata> <subscriber> <bytes>
        Submit an extrinsic to watch.
    /author_unwatchExtrinsic <metadata> <subscription id>
        Unsubscribe from extrinsic watching.
    "#);
    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let trimmed = input.trim();
        let splitted: Vec<&str> = trimmed.split(' ').collect();
        let message = match splitted[0] {
            "/close" => {
                // Close the connection
                let _ = tx.send(OwnedMessage::Close(None));
                break;
            }
            // Send a ping
            "/ping" => OwnedMessage::Ping(b"PING".to_vec()),

            // system
            "/system_name" => {
                let msg = json!({
                    "jsonrpc": "2.0",
                    "id": counter,
                    "method": "system_name"
                });
                OwnedMessage::Text(msg.to_string())
            },
            "/system_version" => {
                let msg = json!({
                    "jsonrpc": "2.0",
                    "id": counter,
                    "method": "system_version"
                });
                OwnedMessage::Text(msg.to_string())
            },
            "/system_chain" => {
                let msg = json!({
                    "jsonrpc": "2.0",
                    "id": counter,
                    "method": "system_chain"
                });
                OwnedMessage::Text(msg.to_string())
            },
            "/system_properties" => {
                let msg = json!({
                    "jsonrpc": "2.0",
                    "id": counter,
                    "method": "system_properties"
                });
                OwnedMessage::Text(msg.to_string())
            },
            "/system_health" => {
                let msg = json!({
                    "jsonrpc": "2.0",
                    "id": counter,
                    "method": "system_health"
                });
                OwnedMessage::Text(msg.to_string())
            },
            "/system_peers" => {
                let msg = json!({
                    "jsonrpc": "2.0",
                    "id": counter,
                    "method": "system_peers"
                });
                OwnedMessage::Text(msg.to_string())
            },
            "/get_balance" => {
                // AliceのAccountId: 0xce3f3d8f09e3411403f5ca59d042a40e
                // {"id":437,"jsonrpc":"2.0","method":"state_subscribeStorage","params":[["0xce3f3d8f09e3411403f5ca59d042a40e"]]}
                let msg = json!({
                    "jsonrpc": "2.0",
                    "id": counter,
                    "method": "state_subscribeStorage",
                    "params": [["0xce3f3d8f09e3411403f5ca59d042a40e"]]
                });
                OwnedMessage::Text(msg.to_string())
            },
            _ => {
                OwnedMessage::Text(trimmed.to_string())
            },
        };
        counter += 1;
        match tx.send(message) {
            Ok(()) => (),
            Err(e) => {
                println!("Main Loop: {:?}", e);
                break;
            }
        }
    }
    println!("Waiting for child threads to exit");
    let _ = send_loop.join();
    let _ = receive_loop.join();
    println!("Exited");
}

