/* multi-ipc-test/src/packet.rs: carrier for data to be sent via the IPC channel
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

extern crate serde;
extern crate ipc_channel;

use serde::ser::{Serialize, Serializer, SerializeStruct};
use serde::Deserialize;
use ipc_channel::ipc::IpcSender;

#[derive(Deserialize)]
pub struct Packet {
    pub data: String,
    pub stop: bool,
    pub sender: Option<Box<IpcSender<Packet>>>,
}

impl Serialize for Packet {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer,
    {
        let mut state = serializer.serialize_struct("Packet", 2)?;
        state.serialize_field("data", &self.data)?;
        state.serialize_field("stop", &self.stop)?;
        state.serialize_field("sender", &self.sender)?;
        state.end()
    }
}