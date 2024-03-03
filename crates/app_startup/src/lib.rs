use bevy::prelude::*;
use ui;

pub fn start_app() {
    App::new()
        .add_plugins(
            (DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: [870., 1066.].into(),
                    resize_constraints: WindowResizeConstraints {
                        min_width: 800.,
                        min_height: 480.,
                        ..Default::default()
                    },
                    title: "LoreLeaf".to_string(),
                    ..Default::default()
                }),
                ..Default::default()
            }),
        ui::UserBooksPlugin))
        .run();
}

