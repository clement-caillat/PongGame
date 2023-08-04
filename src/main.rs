#![windows_subsystem = "windows"]
use ball::BallPlugin;
use bevy::prelude::*;
use bevy::core_pipeline::clear_color::ClearColorConfig;
use player::PlayerPlugin;


mod player;
mod ball;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Ping Pong".into(),
                        resolution: (1080.0, 720.0).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                }),
                BallPlugin,
                PlayerPlugin
        ))
        .add_systems(Startup, setup)
        .run();
}


fn setup(mut commands: Commands) {
    let camera = Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::rgb(0.0, 0.0, 0.0))
        },
        ..Default::default()
    };

    commands.spawn(camera);
}