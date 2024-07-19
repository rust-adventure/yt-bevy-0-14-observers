use bevy::{
    ecs::{component::ComponentId, world::DeferredWorld},
    prelude::*,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, startup)
        .run();
}

fn startup(mut commands: Commands) {
    let id = commands.spawn(ExampleHooksComponent(0)).id();

    commands.entity(id).insert(ExampleHooksComponent(10));

    commands.entity(id).remove::<ExampleHooksComponent>();
    commands.entity(id).remove::<ExampleHooksComponent>();

    commands.entity(id).insert(ExampleHooksComponent(100));
    commands.entity(id).insert(ExampleHooksComponent(1000));
}

#[derive(Component)]
#[component(on_add = my_on_add_hook)]
#[component(on_insert = my_on_insert_hook)]
#[component(on_replace = my_on_replace_hook)]
#[component(on_remove = my_on_remove_hook)]
struct ExampleHooksComponent(i32);

fn my_on_add_hook(
    _world: DeferredWorld,
    entity: Entity,
    _id: ComponentId,
) {
    info!("on_add {entity}",);
}

fn my_on_insert_hook(
    _world: DeferredWorld,
    entity: Entity,
    _id: ComponentId,
) {
    info!("on_insert {entity}");
}

fn my_on_replace_hook(
    _world: DeferredWorld,
    entity: Entity,
    _id: ComponentId,
) {
    info!("on_replace {entity}");
}

fn my_on_remove_hook(
    _world: DeferredWorld,
    entity: Entity,
    _id: ComponentId,
) {
    info!("on_remove {entity}");
}
