use bevy::{app::App, MinimalPlugins};
use networking::NetworkingPlugin;

mod networking;

fn main() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(NetworkingPlugin);
    app.run();
}

