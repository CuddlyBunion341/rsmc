use crate::prelude::*;

pub fn example_system(
  mut _commands: Commands,
  time: Res<Time>,
  mut _example_resource: ResMut<sand_resources::ExampleResource>,
  mut example_writer: EventWriter<sand_events::ExampleEvent>,
  query: Query<(Entity, &sand_components::ExampleComponent)>,
) {
  for (entity, component) in query.iter() {
    if component.active {
      example_writer.send(sand_events::ExampleEvent {
        message: format!("Entity {} is active", entity.index()),
        timestamp: time.elapsed_seconds(),
      });
    }
  }
}
