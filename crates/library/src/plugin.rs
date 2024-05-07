use bevy::prelude::*;
use common::{
    flex_container::{FlexContainer, FlexContainerStyle},
    screens::MainScreenViewData,
    states::NavigationState,
    utilities::despawn_screen,
};

use crate::library::{
    book_interaction_system, compare_books_in_user_library, detect_books_in_library,
    refresh_user_library_on_ui, LibraryViewData, RefreshLibraryTimer, UserLibrary,
};

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
                Update,
                (book_interaction_system).run_if(in_state(NavigationState::Library)),
            )
            .add_systems(
                OnExit(NavigationState::Library),
                despawn_screen::<OnLibraryScreen>,
            );
    }
}

fn library_setup(
    mut commands: Commands,
    main_screen_view_data: Res<MainScreenViewData>,
    user_library: Option<ResMut<UserLibrary>>,
) {
    let flex_container_style = FlexContainerStyle {
        margin: UiRect::all(Val::Px(16.0)),
        ..default()
    };

    let library_screen_entity = commands
        .spawn((
            FlexContainer::new(Some(flex_container_style)),
            OnLibraryScreen,
        ))
        .id();

    commands
        .entity(main_screen_view_data.container_entity)
        .push_children(&[library_screen_entity]);

    commands.insert_resource(LibraryViewData {
        container_entity: library_screen_entity,
    });

    commands.insert_resource(RefreshLibraryTimer(Timer::from_seconds(
        5.0,
        TimerMode::Repeating,
    )));

    if let Some(mut library) = user_library {
        library.clear_displayed();
    } else {
        commands.insert_resource(UserLibrary::empty());
    }
}
