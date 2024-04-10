use bevy::prelude::*;
use bevy_simple_scroll_view::{ScrollView, ScrollViewPlugin, ScrollableContent};
use common::{
    screens::MainScreenViewData, states::NavigationState, text::TEXT_COLOR,
    utilities::despawn_screen,
};

use crate::library::{
    compare_books_in_user_library, detect_books_in_library, refresh_user_library_on_ui,
    LibraryViewData, RefreshLibraryTimer, UserLibrary,
};

#[derive(Component)]
pub struct OnLibraryScreen;

pub struct LibraryPlugin;

impl Plugin for LibraryPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ScrollViewPlugin)
            .add_systems(OnEnter(NavigationState::Library), (library_setup).chain())
            .add_systems(
                Update,
                (
                    detect_books_in_library,
                    compare_books_in_user_library,
                    refresh_user_library_on_ui,
                )
                    .chain()
                    .run_if(in_state(NavigationState::Library)),
            )
            .add_systems(
                OnExit(NavigationState::Library),
                despawn_screen::<OnLibraryScreen>,
            );
    }
}

const CLR_1: Color = Color::rgb(0.168, 0.168, 0.168);
const CLR_2: Color = Color::rgb(0.109, 0.109, 0.109);
const CLR_3: Color = Color::rgb(0.569, 0.592, 0.647);
const CLR_4: Color = Color::rgb(0.902, 0.4, 0.004);

fn library_setup(mut commands: Commands, main_screen_view_data: Res<MainScreenViewData>) {
    let library_screen_entity = commands
        .spawn((
            NodeBundle {
                style: Style {
                    align_items: AlignItems::FlexStart,
                    align_content: AlignContent::FlexStart,
                    justify_content: JustifyContent::FlexStart,
                    flex_wrap: FlexWrap::Wrap,
                    margin: UiRect::all(Val::Px(16.0)),
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            },
            OnLibraryScreen,
        ))
        .id();

    let scroll_root = commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(80.0),
                    margin: UiRect::all(Val::Px(15.0)),
                    ..default()
                },
                background_color: CLR_2.into(),
                ..default()
            },
            ScrollView::default(),
        ))
        .with_children(|p| {
            p.spawn((
                NodeBundle {
                    style: Style {
                        flex_direction: bevy::ui::FlexDirection::Column,
                        width: Val::Percent(100.0),
                        ..default()
                    },
                    ..default()
                },
                ScrollableContent::default(),
            ))
            .with_children(|scroll_area| {
                for i in 0..21 {
                    scroll_area
                        .spawn(NodeBundle {
                            style: Style {
                                min_width: Val::Px(200.0),
                                margin: UiRect::all(Val::Px(15.0)),
                                border: UiRect::all(Val::Px(5.0)),
                                padding: UiRect::all(Val::Px(30.0)),
                                ..default()
                            },
                            border_color: CLR_3.into(),
                            ..default()
                        })
                        .with_children(|p| {
                            p.spawn(
                                TextBundle::from_section(
                                    format!("Nr {git }", i),
                                    TextStyle {
                                        font_size: 25.0,
                                        color: CLR_3,
                                        ..default()
                                    },
                                )
                                .with_text_justify(JustifyText::Center),
                            );
                        });
                }
            });
        })
        .id();

    commands
        .entity(main_screen_view_data.container_entity)
        .push_children(&[scroll_root]);

    commands.insert_resource(LibraryViewData {
        container_entity: scroll_root,
    });

    commands.insert_resource(RefreshLibraryTimer(Timer::from_seconds(
        5.0,
        TimerMode::Repeating,
    )));

    commands.insert_resource(UserLibrary::empty());
}
