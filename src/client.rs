/* multi-ipc-test/src/client.rs: multi-program IPC test client
 * This is free and unencumbered software released into the public domain.
 *
 * Anyone is free to copy, modify, publish, use, compile, sell, or
 * distribute this software, either in source code form or as a compiled
 * binary, for any purpose, commercial or non-commercial, and by any
 * means.
 *
 * In jurisdictions that recognize copyright laws, the author or authors
 * of this software dedicate any and all copyright interest in the
 * software to the public domain. We make this dedication for the benefit
 * of the public at large and to the detriment of our heirs and
 * successors. We intend this dedication to be an overt act of
 * relinquishment in perpetuity of all present and future rights to this
 * software under copyright law.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
 * EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
 * MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
 * IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * For more information, please refer to <http://unlicense.org>
 */

extern crate ipc_channel;

// Use the ~~proprietary~~ custom-designed data carrier struct, called a Packet.
use packet::Packet;

use ipc_channel::ipc::{self, IpcReceiver, IpcSender};
use std::env;
use std::io::{stdin, Read};
use std::process::exit;

// Main entry point
fn main() {
    // Process command-line arguments. There should be at least one (server name).
    // Note: the first “argument”, args[0], is really the program’s name! So it technically requires two args,
    // but we’ll ignore args[0] for clarity.
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        // Too few arguments, show the usage and exit.
        eprintln!("Error: must specify at least one argument\n\
Usage: {} <server-name> [quit]", args[0].to_owned());
        exit(1);
    }

    // The server name.
    let serv_name = args[1].to_owned();

    // Handle for stopping the server
    let stop: bool = if args.len() > 2 {
        // Print a warning if there are three or even more arguments.
        if args.len() > 3 {
            eprintln!("Warning: too many arguments ({}, expected 2), third and next ones will be ignored", args.len()-1);
        }

        // If we have two and the second is "stop", then accept it. Otherwise, print a warning.
        if args[2].eq(&String::from("stop")) {
            true
        } else {
            eprintln!("Warning: unrecognized second argument, it will be ignored");
            false
        }
    } else {
        false
    };

    // Read data from the standard input. You can simply pipe anything here via the shell.
    let mut data: String = String::new();
    stdin().read_to_string(&mut data).expect("Input error!");

    // Try to connect to the server, create an IPC channel and send a Packet
    // with the data, the stop switch, and the new channel’s sender that would
    // provide a callback to this client.
    let sender: IpcSender<Packet> = IpcSender::connect(serv_name.clone())
        .expect("Can’t connect!");
    let (tx, rx): (IpcSender<Packet>, IpcReceiver<Packet>) = ipc::channel()
        .expect("Can’t establish callback!");
    sender.send(Packet { data, stop, sender: Some(Box::new(tx)) })
        .expect("Can’t send the packet!");

    println!("Packet successfully sent to {}", serv_name);

    // Receive the reply and print either the new server name or the farewell message.
    let reply = rx.recv().expect("Can’t receive reply!");
    match reply.stop {
        false => println!("New server name: {}", reply.data),
        true => println!("Server is shutting down. {}", reply.data),
    };
}
