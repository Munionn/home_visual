use bevy::prelude::*;

pub mod camera;
pub mod floorplan;
pub mod items;
pub mod robot_plugin;
pub mod devices;
pub mod bridge;

pub struct HomeVisualizerPlugin;

impl Plugin for HomeVisualizerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            camera::CameraPlugin,
            floorplan::FloorplanPlugin,
            items::ItemsPlugin,
            robot_plugin::RobotPlugin { robot_id: "robot_01".to_string() },
            devices::DevicesPlugin,
            bridge::BridgePlugin,
        ));
    }
}

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn start_bevy_app() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugins(HomeVisualizerPlugin);
    app.run();
}
