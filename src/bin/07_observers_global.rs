use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, startup)
        .observe(on_my_event)
        .run();
}

fn startup(mut commands: Commands) {
    commands.trigger(MyEvent {
        message: "Success".to_string(),
    });
}

#[derive(Debug, Event)]
struct MyEvent {
    message: String,
}

fn on_my_event(
    trigger: Trigger<MyEvent>,
    query: Query<Entity>,
    mut commands: Commands,
) {
    info!(
        "on_my_event {} {:?}",
        trigger.entity(),
        trigger.event()
    );
    for entity in &query {
        commands.entity(entity).log_components();
    }
}
