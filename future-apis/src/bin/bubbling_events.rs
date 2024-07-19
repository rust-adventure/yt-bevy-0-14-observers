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

    commands.entity(id).add_child(id2);
    commands.entity(id2).add_child(id3);

    commands.trigger_targets(
        MyEvent {
            message: "First".to_string(),
        },
        id3,
    );

    commands.trigger_targets(
        MyEvent {
            message: "Second".to_string(),
        },
        id3,
    );
}

#[derive(Debug, Component)]
struct MyEvent {
    message: String,
}

impl Event for MyEvent {
    type Traversal = Parent; // This event will propagate up from child to parent.
    const AUTO_PROPAGATE: bool = true; // This event will propagate by default.
}

fn on_my_event(mut trigger: Trigger<MyEvent>) {
    info!(
        "on_my_event {} {:?}",
        trigger.entity(),
        trigger.event()
    );

    if trigger.event().message == "Second" {
        trigger.propagate(false);
    }
}
