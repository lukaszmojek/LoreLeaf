use bevy::{prelude::*, winit::WinitSettings};
use ui::screens::LoreLeafState;

const LORE_LEAF_TITLE: &str = "LoreLeaf";

pub fn start_app() {
    App::new()
        .init_state::<LoreLeafState>()
        // TODO: Think how to turn that off for development
        // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
        // .insert_resource(WinitSettings::desktop_app())
        .add_systems(Startup, setup)
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: [870., 1066.].into(),
                    resize_constraints: WindowResizeConstraints {
                        min_width: 800.,
                        min_height: 480.,
                        ..Default::default()
                    },
                    title: LORE_LEAF_TITLE.to_string(),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            ui::SplashPlugin,
            ui::HomePlugin,
        ))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
