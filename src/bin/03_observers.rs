use bevy::{ecs::component::StorageType, prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, startup)
        .observe(on_add)
        .observe(on_insert)
        .observe(on_remove)
        .run();
}

fn startup(mut commands: Commands) {
    let id = commands.spawn(ExampleComponent(0)).id();

    commands.entity(id).insert(ExampleComponent(10));

    commands.entity(id).remove::<ExampleComponent>();
    commands.entity(id).remove::<ExampleComponent>();

    commands.entity(id).insert(ExampleComponent(100));
}

#[derive(Debug, Component)]
struct ExampleComponent(i32);

fn on_add(
    trigger: Trigger<OnAdd, ExampleComponent>,
    query: Query<&ExampleComponent>,
) {
    info!(
        "on_add {} {:?}",
        trigger.entity(),
        query.get(trigger.entity())
    );
}

fn on_insert(
    trigger: Trigger<OnInsert, ExampleComponent>,
    query: Query<&ExampleComponent>,
) {
    info!(
        "on_insert {} {:?}",
        trigger.entity(),
        query.get(trigger.entity())
    );
}
fn on_remove(
    trigger: Trigger<OnRemove, ExampleComponent>,
    query: Query<&ExampleComponent>,
) {
    info!(
        "on_remove {} {:?}",
        trigger.entity(),
        query.get(trigger.entity())
    );
}
