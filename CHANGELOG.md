# Changelog

## Unreleased

- Add `bevy_flair` stylesheets
- Add `bevy-inspector-egui`
- Update state management to use bevy `State` instead of a custom `StateMachine`
- Add `renet_visualizer` to visualize network traffic on the server
- Refactor shared code into `lib` namespace
- Drop `lib::` prefix
- Increase world size
- Fix issue where chunks were only serialized on the client
- Add feature flags for debug rendering
- Add grass

## 0.1.1

- Upgrade to Bevy 0.15.2
- Fix collisions
- Add wireframe rendering feature flag
- Improve player sync system flakiness
- Add prompt to chat

## 0.1.0

- Initial release
- Basic terrain generation
- Basic client/server networking
- Player controller
- Raycasting
- Setup Bevy project
- Setup GitHub CI
