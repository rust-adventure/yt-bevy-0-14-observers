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

#[derive(Debug)]
struct ExampleComponent(i32);

impl Component for ExampleComponent {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(
        hooks: &mut bevy::ecs::component::ComponentHooks,
    ) {
        hooks.on_add(|world, entity, _component_id| {
            let component_value = world
                .get::<ExampleComponent>(entity)
                .unwrap();
            info!(
                "hook: on_add {entity} {:?}",
                component_value
            );
        });
        hooks.on_insert(|world, entity, _component_id| {
            let component_value = world
                .get::<ExampleComponent>(entity)
                .unwrap();
            info!(
                "hook: on_insert {entity} {:?}",
                component_value
            );
        });
        hooks.on_remove(|world, entity, _component_id| {
            let component_value = world
                .get::<ExampleComponent>(entity)
                .unwrap();
            info!(
                "hook: on_remove {entity} {:?}",
                component_value
            );
        });
    }
}

fn on_add(
    trigger: Trigger<OnAdd, ExampleComponent>,
    query: Query<&ExampleComponent>,
) {
    info!(
        "observed: on_add {} {:?}",
        trigger.entity(),
        query.get(trigger.entity())
    );
}

fn on_insert(
    trigger: Trigger<OnInsert, ExampleComponent>,
    query: Query<&ExampleComponent>,
) {
    info!(
        "observed: on_insert {} {:?}",
        trigger.entity(),
        query.get(trigger.entity())
    );
}
fn on_remove(
    trigger: Trigger<OnRemove, ExampleComponent>,
    query: Query<&ExampleComponent>,
) {
    info!(
        "observed: on_remove {} {:?}",
        trigger.entity(),
        query.get(trigger.entity())
    );
}
