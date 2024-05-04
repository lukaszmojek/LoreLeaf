use bevy::prelude::*;
use common::{screens::MainScreenViewData, states::NavigationState, utilities::despawn_screen};

#[derive(Component)]
pub struct OnReaderScreen;

pub struct ReaderPlugin;

impl Plugin for ReaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(NavigationState::Reader), (reader_setup).chain())
            // .add_systems(Update, ().chain().run_if(in_state(NavigationState::Reader)))
            // .add_systems(Update, ().run_if(in_state(NavigationState::Library)))
            .add_systems(
                OnExit(NavigationState::Reader),
                despawn_screen::<OnReaderScreen>,
            );
    }
}

fn reader_setup(mut commands: Commands, main_screen_view_data: Res<MainScreenViewData>) {
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
            OnReaderScreen,
        ))
        .id();

    commands
        .entity(main_screen_view_data.container_entity)
        .push_children(&[library_screen_entity]);
}
