use bevy_log::{tracing_subscriber::{self, layer::{Context, SubscriberExt}, Layer}, Level, LogPlugin};

use crate::prelude::*;

pub mod components;
pub mod events;
pub mod resources;
pub mod systems;

pub struct DashboardPlugin;

struct MyCustomLayer;
impl<S: bevy::utils::tracing::Subscriber> Layer<S> for MyCustomLayer {
    fn on_event(&self, event: &bevy::utils::tracing::Event<'_>, _ctx: Context<'_, S>) {
        println!("Event: {:?}", event)
    }
}

fn do_sth_with_subscriber(subscriber: std::boxed::Box<dyn bevy::utils::tracing::Subscriber + Send + Sync>) -> std::boxed::Box<dyn bevy::utils::tracing::Subscriber + Send + Sync> {
    print!("Hello from do_sth_with_subscriber");

    let my_custom_layer = MyCustomLayer;

    
    Box::new(subscriber.with(my_custom_layer))
}

impl Plugin for DashboardPlugin {
    fn build(&self, app: &mut App) {
        info!("Building DashboardPlugin");

        app.add_plugins(LogPlugin {
            level: Level::DEBUG,
            filter: "wgpu=error,bevy_render=info,bevy_ecs=trace".to_string(),
            update_subscriber: Some(do_sth_with_subscriber),
        });

        // app.add_systems(Update, systems::run_basic_ui);
        // app.add_systems(Update, systems::quit_system);

        app.insert_resource(resources::ExampleResource::default());
    }
}
