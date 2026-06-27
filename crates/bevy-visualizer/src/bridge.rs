use bevy::prelude::*;

use crossbeam_channel::{unbounded, Receiver, Sender};
use std::sync::OnceLock;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[derive(Event)]
pub struct IncomingCommand(pub String);

#[derive(Resource)]
pub struct CommandReceiver(pub Receiver<String>);

pub struct BridgePlugin;
static COMMAND_SENDER: OnceLock<Sender<String>> = OnceLock::new();

impl Plugin for BridgePlugin {
    fn build(&self, app: &mut App) {
        let (tx, rx) = unbounded();
        let _ = COMMAND_SENDER.set(tx);

        app.add_event::<IncomingCommand>()
            .insert_resource(CommandReceiver(rx))
            .add_systems(Update, process_commands);
    }
}

fn process_commands(
    receiver: Res<CommandReceiver>,
    mut events: EventWriter<IncomingCommand>,
) {
    while let Ok(cmd) = receiver.0.try_recv() {
        events.send(IncomingCommand(cmd));
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn send_command(command: String) {
    if let Some(sender) = COMMAND_SENDER.get() {
        let _ = sender.send(command);
    }
}
