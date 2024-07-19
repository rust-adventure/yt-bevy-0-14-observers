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

#[derive(Debug)]
struct ExampleHooksComponent(i32);

impl Component for ExampleHooksComponent {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(
        hooks: &mut bevy::ecs::component::ComponentHooks,
    ) {
        hooks.on_add(|world, entity, _component_id| {
            let component_value = world
                .get::<ExampleHooksComponent>(entity)
                .unwrap();
            info!("on_add {entity} {:?}", component_value);
        });
        hooks.on_insert(|world, entity, _component_id| {
            let component_value = world
                .get::<ExampleHooksComponent>(entity)
                .unwrap();
            info!(
                "on_insert {entity} {:?}",
                component_value
            );
        });
        hooks.on_remove(|world, entity, _component_id| {
            let component_value = world
                .get::<ExampleHooksComponent>(entity)
                .unwrap();
            info!(
                "on_remove {entity} {:?}",
                component_value
            );
        });
    }
}
