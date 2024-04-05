use bevy::prelude::*;

//TODO: Move that to some common crate?
// Generic system that takes a component as a parameter, and will despawn all entities with that component
pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    println!("DESPAWN: {:?}", to_despawn);

    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
