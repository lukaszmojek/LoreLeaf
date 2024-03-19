use bevy::prelude::*;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum LoreLeafState {
    #[default]
    Splash,
    Home,
}

pub mod splash {
    use super::{despawn_screen, LoreLeafState};
    use bevy::prelude::*;

    pub struct SplashPlugin;

    impl Plugin for SplashPlugin {
        fn build(&self, app: &mut App) {
            app.init_state::<LoreLeafState>()
                .add_systems(OnEnter(LoreLeafState::Splash), home_setup)
                .add_systems(Update, countdown.run_if(in_state(LoreLeafState::Splash)))
                .add_systems(
                    OnExit(LoreLeafState::Splash),
                    despawn_screen::<OnSplashScreen>,
                );
        }
    }

    // Tag component used to tag entities added on the splash screen
    #[derive(Component)]
    struct OnSplashScreen;

    // Newtype to use a `Timer` for this screen as a resource
    #[derive(Resource, Deref, DerefMut)]
    struct SplashTimer(Timer);

    fn home_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
        let icon = asset_server.load("logo_1024.png");

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
                OnSplashScreen,
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
        commands.insert_resource(SplashTimer(Timer::from_seconds(1.0, TimerMode::Once)));
    }

    // Tick the timer, and change state when finished
    fn countdown(
        mut game_state: ResMut<NextState<LoreLeafState>>,
        time: Res<Time>,
        mut timer: ResMut<SplashTimer>,
    ) {
        if timer.tick(time.delta()).finished() {
            game_state.set(LoreLeafState::Home);
        }
    }
}

pub mod home {
    use super::{despawn_screen, LoreLeafState};
    use bevy::prelude::*;

    pub struct HomePlugin;

    // State used for the current menu screen
    #[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
    pub enum NavigationState {
        #[default]
        Library,
        Reader,
        LoreExplorer,
    }

    impl Plugin for HomePlugin {
        fn build(&self, app: &mut App) {
            app.init_state::<NavigationState>()
                .add_systems(OnEnter(LoreLeafState::Home), home_setup)
                .add_systems(Update, countdown.run_if(in_state(LoreLeafState::Home)))
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
        println!("in HOME")
    }
}

// Tag component used to mark which setting is currently selected
#[derive(Component)]
struct SelectedOption;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const HOVERED_PRESSED_BUTTON: Color = Color::rgb(0.25, 0.65, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

// This system handles changing all buttons color based on mouse interaction
fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut Sprite, Option<&SelectedOption>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut image, selected) in &mut interaction_query {
        image.color = match (*interaction, selected) {
            (Interaction::Pressed, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON,
            (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON,
            (Interaction::Hovered, None) => HOVERED_BUTTON,
            (Interaction::None, None) => NORMAL_BUTTON,
        }
    }
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
