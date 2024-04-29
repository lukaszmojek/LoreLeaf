use crate::buttons::{
    button_interaction_style_system, ButtonConfiguration, ButtonProperties, NavigationButtonBundle,
    NORMAL_BUTTON, PRESSED_BUTTON,
};

use crate::state::LoreLeafState;
use bevy::prelude::*;
use common::screens::MainScreenViewData;
use common::{states::NavigationState, text::TEXT_COLOR, utilities::despawn_screen};
use library::plugin::LibraryPlugin;

pub struct HomePlugin;

impl Plugin for HomePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<NavigationState>()
            .add_systems(OnEnter(LoreLeafState::Home), home_navigation_setup)
            .add_systems(
                Update,
                (
                    navigation_button_interaction_system,
                    button_interaction_style_system,
                )
                    .run_if(in_state(LoreLeafState::Home)),
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

#[derive(Component, Debug)]
pub enum NavigationButtonAction {
    Home,
    Library,
    Reader,
    LoreExplorer,
}

pub struct NavigationButtonProperties {
    pub action: NavigationButtonAction,
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

//main screen setup
fn home_navigation_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let main_screen_entity = commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Start,
                    justify_content: JustifyContent::Start,
                    flex_direction: FlexDirection::Row,
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
                    spawn_navigation_button(
                        parent,
                        &asset_server,
                        "home",
                        NavigationButtonAction::Home,
                    );
                    spawn_navigation_button(
                        parent,
                        &asset_server,
                        "library",
                        NavigationButtonAction::Library,
                    );
                    spawn_navigation_button(
                        parent,
                        &asset_server,
                        "eyeglasses",
                        NavigationButtonAction::Reader,
                    );
                    spawn_navigation_button(
                        parent,
                        &asset_server,
                        "explore", //lore
                        NavigationButtonAction::LoreExplorer,
                    );
                });
        })
        .id();

    commands.insert_resource(MainScreenViewData {
        container_entity: main_screen_entity,
    });
}

fn spawn_navigation_button(
    parent: &mut ChildBuilder<'_>,
    asset_server: &Res<AssetServer>,
    icon_name: &str,
    button_action: NavigationButtonAction,
) {
    parent
        .spawn(NavigationButtonBundle {
            button: ButtonBundle {
                style: ButtonConfiguration::instance().style.clone(),
                border_color: BorderColor(Color::BLACK),
                ..default()
            },
            properties: default(),
            action: button_action,
        })
        .with_children(|parent| {
            let icon: Handle<Image> = asset_server.load(format!("menu/{}.png", icon_name));
            parent.spawn(ImageBundle {
                style: ButtonConfiguration::instance().icon_style.clone(),
                image: UiImage::new(icon),
                ..default()
            });
        });
}

fn navigation_button_interaction_system(
    mut interaction_query: Query<
        (&Interaction, &mut BorderColor, &NavigationButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    current_navigation_state: Res<State<NavigationState>>,
    mut next_navigation_state: ResMut<NextState<NavigationState>>,
) {
    for (interaction, mut border_color, menu_button_action) in &mut interaction_query {
        let assigned_navigation_stae = match menu_button_action {
            NavigationButtonAction::Home => NavigationState::Home,
            NavigationButtonAction::Library => NavigationState::Library,
            NavigationButtonAction::Reader => NavigationState::Reader,
            NavigationButtonAction::LoreExplorer => NavigationState::LoreExplorer,
        };

        match *interaction {
            Interaction::Hovered => {
                border_color.0 = Color::WHITE;
            }
            Interaction::Pressed => {
                border_color.0 = PRESSED_BUTTON;

                match menu_button_action {
                    NavigationButtonAction::Home => {
                        next_navigation_state.set(NavigationState::Home);
                    }
                    NavigationButtonAction::Library => {
                        next_navigation_state.set(NavigationState::Library);
                    }
                    NavigationButtonAction::Reader => {
                        next_navigation_state.set(NavigationState::Reader);
                    }
                    NavigationButtonAction::LoreExplorer => {
                        next_navigation_state.set(NavigationState::LoreExplorer);
                    }
                }
            }
            Interaction::None => border_color.0 = NORMAL_BUTTON,
        }

        if current_navigation_state.as_ref() == &assigned_navigation_stae {
            border_color.0 = Color::YELLOW;
        }
    }
}
