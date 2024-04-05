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
            app.add_systems(OnEnter(LoreLeafState::Splash), splash_setup)
                .add_systems(Update, countdown.run_if(in_state(LoreLeafState::Splash)))
                .add_systems(
                    OnExit(LoreLeafState::Splash),
                    despawn_screen::<OnSplashScreen>,
                );
        }
    }

    #[derive(Component)]
    struct OnSplashScreen;

    #[derive(Resource, Deref, DerefMut)]
    struct SplashTimer(Timer);

    fn splash_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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
                        width: Val::Px(512.0),
                        ..default()
                    },
                    image: UiImage::new(icon),
                    ..default()
                });
            });

        commands.insert_resource(SplashTimer(Timer::from_seconds(0.5, TimerMode::Once)));
    }

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
    use crate::{
        buttons::{button_system, ButtonConfiguration},
        library::LibraryPlugin,
        text::TEXT_COLOR,
    };

    use super::{despawn_screen, LoreLeafState};
    use bevy::prelude::*;

    pub struct HomePlugin;

    // State used for the current menu screen
    #[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
    pub enum NavigationState {
        Home,
        Library,
        Reader,
        LoreExplorer,
        #[default]
        Disabled,
    }

    impl Plugin for HomePlugin {
        fn build(&self, app: &mut App) {
            app.init_state::<NavigationState>()
                .add_systems(OnEnter(LoreLeafState::Home), home_navigation_setup)
                .add_systems(
                    Update,
                    (navigation_action, button_system).run_if(in_state(LoreLeafState::Home)),
                )
                .add_systems(OnExit(LoreLeafState::Home), despawn_screen::<OnHomeScreen>) //TODO: List all possible navigation states?
                .add_systems(OnEnter(NavigationState::Home), home_setup)
                .add_systems(
                    OnExit(NavigationState::Home),
                    despawn_screen::<OnHomeScreen>,
                )
                .add_plugins(LibraryPlugin);
        }
    }

    #[derive(Component)]
    struct OnHomeScreen;

    #[derive(Component)]
    struct OnReaderScreen;

    #[derive(Component)]
    struct OrLoreExplorerScreen;

    #[derive(Component)]
    struct OnNavigation;

    #[derive(Component)]
    enum NavigationButtonAction {
        Home,
        Library,
        Reader,
        LoreExplorer,
    }

    #[derive(Resource, Deref, DerefMut)]
    struct HomeTimer(Timer);

    fn home_setup(mut commands: Commands) {
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
                parent.spawn(
                    TextBundle::from_section(
                        "HOME",
                        TextStyle {
                            font_size: 80.0,
                            color: TEXT_COLOR,
                            ..default()
                        },
                    )
                    .with_style(Style {
                        margin: UiRect::all(Val::Px(50.0)),
                        ..default()
                    }),
                );
            });
    }

    fn home_navigation_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands
            .spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        align_items: AlignItems::Start,
                        justify_content: JustifyContent::Start,
                        ..default()
                    },
                    ..default()
                },
                OnNavigation,
            ))
            .with_children(|parent| {
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            width: Val::Percent(10.0),
                            max_width: Val::Px(74.0),
                            height: Val::Percent(100.0),
                            ..default()
                        },
                        background_color: BackgroundColor::from(Color::CRIMSON),
                        ..default()
                    })
                    .with_children(|parent| {
                        spawn_button(parent, &asset_server, "home", NavigationButtonAction::Home);
                        spawn_button(
                            parent,
                            &asset_server,
                            "library",
                            NavigationButtonAction::Library,
                        );
                        spawn_button(
                            parent,
                            &asset_server,
                            "eyeglasses",
                            NavigationButtonAction::Reader,
                        );
                        spawn_button(
                            parent,
                            &asset_server,
                            "explore", //lore
                            NavigationButtonAction::LoreExplorer,
                        );
                    });
            });
    }

    fn spawn_button(
        parent: &mut ChildBuilder<'_>,
        asset_server: &Res<AssetServer>,
        icon_name: &str,
        button_action: NavigationButtonAction,
    ) {
        parent
            .spawn((
                ButtonBundle {
                    style: ButtonConfiguration::instance().style.clone(),
                    border_color: BorderColor(Color::BLACK),
                    ..default()
                },
                button_action,
            ))
            .with_children(|parent| {
                let icon: Handle<Image> = asset_server.load(format!("menu/{}.png", icon_name));
                parent.spawn(ImageBundle {
                    style: ButtonConfiguration::instance().icon_style.clone(),
                    image: UiImage::new(icon),
                    ..default()
                });
            });
    }

    fn navigation_action(
        interaction_query: Query<
            (&Interaction, &NavigationButtonAction),
            (Changed<Interaction>, With<Button>),
        >,
        mut navigation_state: ResMut<NextState<NavigationState>>,
    ) {
        for (interaction, menu_button_action) in &interaction_query {
            if *interaction == Interaction::Pressed {
                match menu_button_action {
                    NavigationButtonAction::Home => {
                        navigation_state.set(NavigationState::Home);
                    }
                    NavigationButtonAction::Library => {
                        navigation_state.set(NavigationState::Library);
                    }
                    NavigationButtonAction::Reader => {
                        navigation_state.set(NavigationState::Reader);
                    }
                    NavigationButtonAction::LoreExplorer => {
                        navigation_state.set(NavigationState::LoreExplorer);
                    }
                    _ => panic!("Unknown navigation button action!",),
                }
            }
        }
    }
}

//TODO: Move that to some common crate?
// Generic system that takes a component as a parameter, and will despawn all entities with that component
pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    println!("DESPAWN: {:?}", to_despawn);

    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
