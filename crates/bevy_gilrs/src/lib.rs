#![allow(clippy::type_complexity)]

mod converter;
mod gilrs_system;
mod rumble;

use bevy_app::{App, Plugin, PostUpdate, PreStartup, PreUpdate};
use bevy_ecs::prelude::*;
use bevy_input::InputSystem;
use bevy_utils::tracing::error;
pub use gilrs::ff;
use gilrs::GilrsBuilder;
use gilrs_system::{gilrs_event_startup_system, gilrs_event_system};
use rumble::{play_gilrs_rumble, RumblesManager};
pub use rumble::{RumbleIntensity, RumbleRequest};

#[derive(Default)]
pub struct GilrsPlugin;

impl Plugin for GilrsPlugin {
    fn build(&self, app: &mut App) {
        match GilrsBuilder::new()
            .with_default_filters(false)
            .set_update_state(false)
            .build()
        {
            Ok(gilrs) => {
                app.insert_non_send_resource(gilrs)
                    .add_event::<RumbleRequest>()
                    .init_non_send_resource::<RumblesManager>()
                    .add_systems(PreStartup, gilrs_event_startup_system)
                    .add_systems(PreUpdate, gilrs_event_system.before(InputSystem))
                    .add_systems(PostUpdate, play_gilrs_rumble);
            }
            Err(err) => error!("Failed to start Gilrs. {}", err),
        }
    }
}
