pub mod prelude;
pub mod networking;

use crate::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(networking::NetworkingPlugin);
    app.run();
}
