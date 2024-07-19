use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, startup)
        .run();
}

fn startup(mut commands: Commands) {
    let id =
        commands.spawn_empty().observe(on_my_event).id();
    let id2 =
        commands.spawn_empty().observe(on_my_event).id();
    let id3 =
        commands.spawn_empty().observe(on_my_event).id();

    commands.trigger_targets(
        MyEvent {
            message: "Success".to_string(),
        },
        id,
    );

    commands.trigger_targets(
        MyEvent {
            message: "Second Event".to_string(),
        },
        vec![id, id2, id3],
    );
}

#[derive(Debug, Event)]
struct MyEvent {
    message: String,
}

fn on_my_event(trigger: Trigger<MyEvent>) {
    info!(
        "on_my_event {} {:?}",
        trigger.entity(),
        trigger.event()
    );
}
