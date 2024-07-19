use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, startup)
        .observe(on_my_event)
        .run();
}

fn startup(mut commands: Commands) {
    commands.trigger(MyEvent { spawn_n_times: 4 });
}

#[derive(Debug, Event)]
struct MyEvent {
    spawn_n_times: usize,
}

fn on_my_event(
    trigger: Trigger<MyEvent>,
    query: Query<Entity>,
    mut commands: Commands,
) {
    warn!(
        "on_my_event {} {:?}",
        trigger.entity(),
        trigger.event()
    );
    for entity in &query {
        commands.entity(entity).log_components();
    }
    let Some(rest) =
        trigger.event().spawn_n_times.checked_sub(1)
    else {
        info!("done recursing");
        return;
    };
    commands.trigger(MyEvent {
        spawn_n_times: rest,
    });
}
