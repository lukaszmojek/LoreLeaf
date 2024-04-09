use std::time::{SystemTime, UNIX_EPOCH};

use bevy::prelude::*;

//TODO: Move that to some common crate?
// Generic system that takes a component as a parameter, and will despawn all entities with that component
pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    println!("DESPAWN: {:?}", to_despawn);

    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn print_current_time(method_name: &str) {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    let in_seconds = since_the_epoch.as_secs();

    println!("{:?}", method_name.to_uppercase());
    println!("{:?}", in_seconds);
}
