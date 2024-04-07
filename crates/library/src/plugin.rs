use bevy::prelude::*;
use common::{
    screens::MainScreenViewData, states::NavigationState, text::TEXT_COLOR,
    utilities::despawn_screen,
};

use crate::library::{
    compare_books_in_user_library, detect_books_in_library, refresh_user_library_on_ui,
    LibraryViewData, RefreshLibraryTimer, UserLibrary,
};

const LIBRARY_SCREEN: &str = "LIBRARY";

#[derive(Component)]
pub struct OnLibraryScreen;

pub struct LibraryPlugin;

impl Plugin for LibraryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(NavigationState::Library), (library_setup).chain())
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

fn library_setup(mut commands: Commands, main_screen_view_data: Res<MainScreenViewData>) {
    let library_screen_entity = commands
        .spawn(NodeBundle {
            style: Style {
                align_items: AlignItems::FlexStart,
                justify_content: JustifyContent::FlexStart,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    LIBRARY_SCREEN,
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
        })
        .id();

    commands
        .entity(main_screen_view_data.container_entity)
        .push_children(&[library_screen_entity]);

    commands.insert_resource(LibraryViewData {
        container_entity: library_screen_entity,
    });

    commands.insert_resource(RefreshLibraryTimer(Timer::from_seconds(
        2.0,
        TimerMode::Repeating,
    )));
    commands.insert_resource(UserLibrary::empty());
}
