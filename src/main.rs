pub mod animations;
mod block;
mod hologramify;
mod interface;
mod setup;
mod setup_menu;
mod simulation;
mod structure;
mod world;

use animations::AnimationPlugin;
use bevy::prelude::*;
use bevy_mod_raycast::{DefaultRaycastingPlugin, RaycastSystem};
use bevy_obj::ObjPlugin;
use block::{update_raycast_position_from_cursor, BlockRaycastSet};
use hologramify::HologramifyPlugin;
use interface::InterfacePlugin;
use setup::SetupPlugin;
use setup_menu::MenuPlugin;
use simulation::SimulationPlugin;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum GameState {
    Menu,
    Level,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ObjPlugin)
        .add_plugin(DefaultRaycastingPlugin::<BlockRaycastSet>::default())
        .add_state(GameState::Level)
        .add_plugin(SimulationPlugin)
        .add_plugin(AnimationPlugin)
        .add_plugin(HologramifyPlugin)
        .add_plugin(SetupPlugin)
        .add_plugin(InterfacePlugin)
        .add_plugin(MenuPlugin)
        .insert_resource(AmbientLight {
            color: Color::hex("264653").unwrap(),
            brightness: 5.0,
        })
        .insert_resource(ClearColor(Color::hex("264653").unwrap() * 0.6))
        .add_system_to_stage(
            CoreStage::First,
            update_raycast_position_from_cursor.before(RaycastSystem::BuildRays::<BlockRaycastSet>),
        )
        .run();
}
