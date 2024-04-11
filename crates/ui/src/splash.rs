use bevy::prelude::*;
use common::utilities::despawn_screen;

use crate::state::LoreLeafState;

const LOGO_FILE: &str = "logo_1024.png";

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
    let icon = asset_server.load(LOGO_FILE);

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
