use bevy::prelude::*;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum LoreLeafState {
    #[default]
    Home,
    Reader,
    LoreExplorer,
}

pub struct HomePlugin;

impl Plugin for HomePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<LoreLeafState>()
            // When entering the state, spawn everything needed for this screen
            .add_systems(OnEnter(LoreLeafState::Home), home_setup)
            // While in this state, run the `countdown` system
            .add_systems(Update, countdown.run_if(in_state(LoreLeafState::Home)))
            // When exiting the state, despawn everything that was spawned for this screen
            .add_systems(OnExit(LoreLeafState::Home), despawn_screen::<OnHomeScreen>);
    }
}

// Tag component used to tag entities added on the home screen
#[derive(Component)]
struct OnHomeScreen;

// Newtype to use a `Timer` for this screen as a resource
#[derive(Resource, Deref, DerefMut)]
struct HomeTimer(Timer);

fn home_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let icon = asset_server.load("logo_1024.png");
    // Display the logo
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            },
            OnHomeScreen,
        ))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                style: Style {
                    // This will set the logo to be 200px wide, and auto adjust its height
                    width: Val::Px(512.0),
                    ..default()
                },
                image: UiImage::new(icon),
                ..default()
            });
        });
    // Insert the timer as a resource
    commands.insert_resource(HomeTimer(Timer::from_seconds(1.0, TimerMode::Once)));
}

// Tick the timer, and change state when finished
fn countdown(
    mut game_state: ResMut<NextState<LoreLeafState>>,
    time: Res<Time>,
    mut timer: ResMut<HomeTimer>,
) {
    if timer.tick(time.delta()).finished() {
        game_state.set(LoreLeafState::Reader);
    }
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
