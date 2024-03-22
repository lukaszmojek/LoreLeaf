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

    // Tag component used to tag entities added on the splash screen
    #[derive(Component)]
    struct OnSplashScreen;

    // Newtype to use a `Timer` for this screen as a resource
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
                        // This will set the logo to be 200px wide, and auto adjust its height
                        width: Val::Px(512.0),
                        ..default()
                    },
                    image: UiImage::new(icon),
                    ..default()
                });
            });

        // Insert the timer as a resource
        commands.insert_resource(SplashTimer(Timer::from_seconds(0.5, TimerMode::Once)));
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
    use super::{button_system, despawn_screen, LoreLeafState, NORMAL_BUTTON, TEXT_COLOR};
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

    // Tag component used to tag entities added on the home screen
    #[derive(Component)]
    struct OnHomeScreen;

    // All actions that can be triggered from a button click
    #[derive(Component)]
    enum MenuButtonAction {
        Library,
        Reader,
        LoreExplorer,
    }

    // Newtype to use a `Timer` for this screen as a resource
    #[derive(Resource, Deref, DerefMut)]
    struct HomeTimer(Timer);

    fn home_setup(mut menu_state: ResMut<NextState<MenuState>>) {
        menu_state.set(MenuState::Main);
    }

    fn main_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
        // Common style for all buttons on the screen
        let button_style = Style {
            width: Val::Px(250.0),
            height: Val::Px(65.0),
            margin: UiRect::all(Val::Px(20.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border: UiRect::all(Val::Px(5.0)),
            ..default()
        };

        let button_icon_style = Style {
            width: Val::Px(30.0),
            // This takes the icons out of the flexbox flow, to be positioned exactly
            position_type: PositionType::Absolute,
            // The icon will be close to the left border of the button
            left: Val::Px(10.0),
            ..default()
        };
        let button_text_style = TextStyle {
            font_size: 40.0,
            color: TEXT_COLOR,
            ..default()
        };

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
                                    style: button_style.clone(),
                                    border_color: BorderColor(Color::BLACK),
                                    ..default()
                                },
                                MenuButtonAction::Library,
                            ))
                            .with_children(|parent| {
                                parent.spawn(ImageBundle {
                                    style: button_icon_style.clone(),
                                    ..default()
                                });
                                parent.spawn(TextBundle::from_section(
                                    "Library",
                                    button_text_style.clone(),
                                ));
                            });

                        parent
                            .spawn((
                                ButtonBundle {
                                    style: button_style.clone(),
                                    border_color: BorderColor(Color::BLACK),
                                    ..default()
                                },
                                MenuButtonAction::Reader,
                            ))
                            .with_children(|parent| {
                                parent.spawn(ImageBundle {
                                    style: button_icon_style.clone(),
                                    ..default()
                                });
                                parent.spawn(TextBundle::from_section(
                                    "Reader",
                                    button_text_style.clone(),
                                ));
                            });

                        parent
                            .spawn((
                                ButtonBundle {
                                    style: button_style.clone(),
                                    border_color: BorderColor(Color::BLACK),
                                    ..default()
                                },
                                MenuButtonAction::LoreExplorer,
                            ))
                            .with_children(|parent| {
                                parent.spawn(ImageBundle {
                                    style: button_icon_style.clone(),
                                    ..default()
                                });
                                parent.spawn(TextBundle::from_section(
                                    "Lore",
                                    button_text_style.clone(),
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

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const HOVERED_PRESSED_BUTTON: Color = Color::rgb(0.25, 0.65, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

// This system handles changing all buttons color based on mouse interaction
fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut UiImage, &mut BorderColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    // println!("{:?}", interaction_query);

    for (interaction, mut image, mut border_color, children) in &mut interaction_query {
        println!("{:?}", interaction);

        border_color.0 = match *interaction {
            Interaction::Pressed => PRESSED_BUTTON,
            Interaction::Hovered => HOVERED_PRESSED_BUTTON,
            // Interaction::Hovered => HOVERED_BUTTON,
            Interaction::None => NORMAL_BUTTON,
        }
    }
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
