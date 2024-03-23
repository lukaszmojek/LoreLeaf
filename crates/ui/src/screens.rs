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
    use crate::buttons::{button_system, ButtonConfiguration};

    use super::{despawn_screen, LoreLeafState, TEXT_COLOR};
    use bevy::{app::AppExit, prelude::*};

    pub struct HomePlugin;

    // State used for the current menu screen
    #[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
    pub enum NavigationState {
        #[default]
        Library,
        Reader,
        LoreExplorer,
    }

    // State used for the current menu screen
    #[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
    enum MenuState {
        Main,
        Settings,
        SettingsDisplay,
        SettingsSound,
        #[default]
        Disabled,
    }

    impl Plugin for HomePlugin {
        fn build(&self, app: &mut App) {
            app.init_state::<NavigationState>()
                .add_systems(OnEnter(LoreLeafState::Home), home_setup)
                .add_systems(OnExit(LoreLeafState::Home), despawn_screen::<OnHomeScreen>)
                // .add_systems(OnEnter(NavigationState::Library), home_setup)
                // .add_systems(Update, countdown.run_if(in_state(LoreLeafState::Home)))
                .init_state::<MenuState>()
                .add_systems(OnEnter(MenuState::Main), main_menu_setup)
                .add_systems(
                    Update,
                    (navigation_action, button_system).run_if(in_state(LoreLeafState::Home)),
                );
        }
    }

    #[derive(Component)]
    struct OnHomeScreen;

    #[derive(Component)]
    enum MenuButtonAction {
        Library,
        Reader,
        LoreExplorer,
    }

    #[derive(Resource, Deref, DerefMut)]
    struct HomeTimer(Timer);

    fn home_setup(mut menu_state: ResMut<NextState<MenuState>>) {
        menu_state.set(MenuState::Main);
    }

    fn main_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands
            .spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                },
                //TODO: Should probably be navigation
                OnHomeScreen,
            ))
            .with_children(|parent| {
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: BackgroundColor::from(Color::CRIMSON),
                        ..default()
                    })
                    .with_children(|parent| {
                        // Display the game name
                        parent.spawn(
                            TextBundle::from_section(
                                "Nav",
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

                        parent
                            .spawn((
                                ButtonBundle {
                                    style: ButtonConfiguration::instance().style.clone(),
                                    border_color: BorderColor(Color::BLACK),
                                    ..default()
                                },
                                MenuButtonAction::Library,
                            ))
                            .with_children(|parent| {
                                parent.spawn(ImageBundle {
                                    style: ButtonConfiguration::instance().icon_style.clone(),
                                    ..default()
                                });
                                parent.spawn(TextBundle::from_section(
                                    "Library",
                                    ButtonConfiguration::instance().text_style.clone(),
                                ));
                            });

                        parent
                            .spawn((
                                ButtonBundle {
                                    style: ButtonConfiguration::instance().style.clone(),
                                    border_color: BorderColor(Color::BLACK),
                                    ..default()
                                },
                                MenuButtonAction::Reader,
                            ))
                            .with_children(|parent| {
                                parent.spawn(ImageBundle {
                                    style: ButtonConfiguration::instance().icon_style.clone(),
                                    ..default()
                                });
                                parent.spawn(TextBundle::from_section(
                                    "Reader",
                                    ButtonConfiguration::instance().text_style.clone(),
                                ));
                            });

                        parent
                            .spawn((
                                ButtonBundle {
                                    style: ButtonConfiguration::instance().style.clone(),
                                    border_color: BorderColor(Color::BLACK),
                                    ..default()
                                },
                                MenuButtonAction::LoreExplorer,
                            ))
                            .with_children(|parent| {
                                parent.spawn(ImageBundle {
                                    style: ButtonConfiguration::instance().icon_style.clone(),
                                    ..default()
                                });
                                parent.spawn(TextBundle::from_section(
                                    "Lore",
                                    ButtonConfiguration::instance().text_style.clone(),
                                ));
                            });
                    });
            });
    }

    fn navigation_action(
        interaction_query: Query<
            (&Interaction, &MenuButtonAction),
            (Changed<Interaction>, With<Button>),
        >,
        mut menu_state: ResMut<NextState<MenuState>>,
        mut navigation_state: ResMut<NextState<NavigationState>>,
    ) {
        // println!("navigation_action");
        for (interaction, menu_button_action) in &interaction_query {
            if *interaction == Interaction::Pressed {
                match menu_button_action {
                    MenuButtonAction::Library => {
                        navigation_state.set(NavigationState::Library);
                        menu_state.set(MenuState::Main);
                    }
                    MenuButtonAction::Reader => {
                        navigation_state.set(NavigationState::Reader);
                    }
                    MenuButtonAction::LoreExplorer => {
                        navigation_state.set(NavigationState::LoreExplorer);
                    }
                }
            }
        }
    }
}

const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
