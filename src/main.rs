/* multi-ipc-test/src/main.rs: multi-program IPC test server
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

use ipc_channel::ipc::{IpcOneShotServer, IpcSender};
use std::process::exit;

// Main entry point
fn main() {
    // This program uses a rotating server model, where a one-shot server
    // processes a single message and gives its place to another one.

    // This mutable variable will be shared across all iterations.
    // None indicates that it’s the first iteration.
    // It also allows to bypass borrowing problems.
    let mut sender: Option<Box<IpcSender<Packet>>> = None;

    // Farewell message
    let farewell = "Goodbye fellow Rustaceans!";

    // The listening loop
    loop {
        /* A one-shot server supplied by ipc-channel justifies its name: it
         * receives a single message,  accepts it and then passes away. Here,
         * we have to re-create the server after each message, and as a result
         * the server name constantly changes. The first server’s name is printed
         * standard output, while the subsequent servers transmit their names
         * to the client. Unfortunately I haven’t found a better choice to make
         * a persistent server program that would accept data from multiple
         * clients without re-creating new server instances.
         */

        // Create a server.
        let (server, serv_name) = IpcOneShotServer::new()
            .expect("Fatal error on server initialization!");

        // Here’s where the Option comes into play: we use it to determine whether
        // is this the first  server or not.
        if let Some(realsend) = sender {
            // This is not the first server, so we send its name back to the client.
            realsend.send(Packet {
                data: serv_name.clone(),
                stop: false,
                sender: None
            }).unwrap_or_else(|err| {
                // Failing that, we print it to standard error.
                eprintln!("Warning: Cannot reply to the client: {}\
The current server name is {}", err, serv_name);
            });
        } else {
            // This is the first server, so we just print out its name
            // (to standard error as to differentiate it from incoming data).
            eprintln!("Server name is {}", serv_name);
        }

        // Accept the incoming packet. This will consume the server.
        let (_, packet): (_, Packet) = server.accept()
            .expect("Fatal error on accepting a message!");

        // Print the message
        print!("{}", packet.data);

        // Retrieve the callback channel.
        sender = packet.sender;
        let actsend = sender.to_owned().expect("No callback channel!");

        // If we are to quit, we should tell the client about that
        // before going down.
        if packet.stop {
            actsend.send(Packet {
                data: farewell.to_owned(),
                stop: true,
                sender: None
            }).unwrap_or_else(|err| {
                eprintln!("Warning: Please pass these words to the client,\
I can’t do it myself ({}):\
{}", err, farewell.to_owned());
            });
            exit(0);
        }

        /* Theoretically, passing a Sender back to the client via the Packet
         * would allow to establish a normal two-way link between the client and
         * the server; the Packet struct, with sender field no longer used and
         * set to None, would be used for transmitting data.
         * This is out of scope for this program.
         */
    }
}

