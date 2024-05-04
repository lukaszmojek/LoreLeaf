use bevy::prelude::*;
use common::{
    screens::MainScreenViewData, states::NavigationState, text::TEXT_COLOR,
    utilities::despawn_screen,
};

use crate::toolbar::ReaderToolbarBundle;

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
    let reader_screen = commands
        .spawn((
            NodeBundle {
                style: Style {
                    align_items: AlignItems::FlexStart,
                    align_content: AlignContent::FlexStart,
                    justify_content: JustifyContent::FlexStart,
                    flex_wrap: FlexWrap::Wrap,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            },
            OnReaderScreen,
        ))
        .with_children(|parent| {
            parent.spawn(ReaderToolbarBundle::new());

            parent.spawn(
                TextBundle::from_section(
                    "READER",
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
        .push_children(&[reader_screen]);
}
