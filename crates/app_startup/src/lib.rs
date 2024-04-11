use bevy::{asset::load_internal_binary_asset, prelude::*, winit::WinitSettings};
use ui::state::LoreLeafState;

const LORE_LEAF_TITLE: &str = "LoreLeaf";

pub fn start_app() {
    let mut app = App::new();

    app.init_state::<LoreLeafState>()
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
        ));

    // This needs to happen after `DefaultPlugins` is added.
    // It is an workaround used for replacing font with custom one and was mentioned here: https://github.com/bevyengine/bevy/discussions/11839
    // Other option would be the need of specyfying font each time a TextBundle is used, just like it was done here: https://github.com/bevyengine/bevy/pull/9259/files
    load_internal_binary_asset!(
        app,
        TextStyle::default().font,
        "../../../assets/fonts/ReemKufiFun-Regular.ttf",
        |bytes: &[u8], _path: String| { Font::try_from_bytes(bytes.to_vec()).unwrap() }
    );

    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
