use bevy::prelude::*;

pub struct UserBooksPlugin;

impl Plugin for UserBooksPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(UserBooksTimer(Timer::from_seconds(
            2.0,
            TimerMode::Repeating,
        )))
        .add_systems(Startup, propagate_user_books)
        .add_systems(Update, (update_user_books, print_user_books).chain());
    }
}

#[derive(Component, Debug)]
struct Book {
    name: String,
    author: String,
}

fn propagate_user_books(mut commands: Commands) {
    commands.spawn(Book {
        name: "Słowa światłości".to_string(),
        author: "Brandon Sanderson".to_string(),
    });
    commands.spawn(Book {
        name: "Elantris".to_string(),
        author: "Brandon Sanderson".to_string(),
    });
    commands.spawn(Book {
        name: "Red Rising".to_string(),
        author: "Pierce Brown".to_string(),
    });
}

#[derive(Resource)]
struct UserBooksTimer(Timer);

fn print_user_books(time: Res<Time>, mut timer: ResMut<UserBooksTimer>, query: Query<&Book>) {
    if timer.0.tick(time.delta()).just_finished() {
        for book in &query {
            println!("{:#?}", book);
        }
    }
}

fn update_user_books(mut query: Query<&mut Book>) {
    for mut book in &mut query {
        if book.name == "Elantris" {
            book.name = "Elantris: Dusza Cesarzowej".to_string();
            break;
        }
    }
}
