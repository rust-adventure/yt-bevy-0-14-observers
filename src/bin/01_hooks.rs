use bevy::{ecs::component::StorageType, prelude::*};

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
}

struct ExampleHooksComponent(i32);

impl Component for ExampleHooksComponent {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(
        hooks: &mut bevy::ecs::component::ComponentHooks,
    ) {
        hooks.on_add(
            |mut _world, entity, _component_id| {
                info!("on_add {entity}",);
            },
        );
        hooks.on_insert(
            |mut _world, entity, _component_id| {
                info!("on_insert {entity}");
            },
        );
        hooks.on_remove(
            |mut _world, entity, _component_id| {
                info!("on_remove {entity}");
            },
        );
    }
}
