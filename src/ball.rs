use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
pub struct BallPlugin;

use crate::player::Player;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ball);
        app.add_systems(Update, (ball_movement, ball_player_collision));
    }
}

#[derive(Component)]
pub struct Ball {
    velocity: Vec2,
    speed: f32
}

fn spawn_ball(mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>) {
    
    // let texture: Handle<Image> = asset_server.load("ball.png");
    
    commands.spawn((
        // SpriteBundle {
        //     texture,
        //     transform: Transform {
        //         scale: Vec3::new(0.1, 0.1, 0.0),
        //         translation: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
        //         ..default()
        //     },
        //     ..default()
        // },
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Circle::new(10.0))).into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform {
                translation: Vec3 { x: -450.0, y: 0.0, z: (0.0) },
                ..default()
            },
            ..default()
        },
        Ball {
            velocity: Vec2::new(10.0, 10.0),
            speed: 40.0
        },
    ));
}


fn ball_movement(
    mut ball: Query<(&mut Transform, &mut Ball)>,
    window: Query<&Window>,
    time: Res<Time>,
) {

    let win = window.get_single().unwrap();
    // println!("Width : {}\nHeigth : {}", win.width(), win.height());

    for (mut transform, mut _ball) in &mut ball {
        
        transform.translation.x += _ball.velocity.x * _ball.speed * time.delta_seconds();
        transform.translation.y += _ball.velocity.y * _ball.speed * time.delta_seconds();

        if transform.translation.x + 10.0 >= win.width() / 2.0
        || transform.translation.x - 10.0 <= -win.width() / 2.0 {
            _ball.velocity.x = -_ball.velocity.x;
        }
        if transform.translation.y + 10.0 >= win.height() / 2.0  
        || transform.translation.y - 10.0 <= -win.height() / 2.0 {
            _ball.velocity.y = -_ball.velocity.y;
        }

    }
}

fn ball_player_collision(
    mut ball: Query<(&Transform, &mut Ball)>,
    player: Query<(&Transform, &Player)>,
    mut query: Query<&mut Text>,
) {


    for (ball_transform, mut _ball) in &mut ball {
        for (player_transform, _) in &player {
            let ball_pos = ball_transform.translation;
            let player_pos = player_transform.translation;

            // print!("Ball : {} | Player : {}      \r", ball_pos, player_pos.y);
            // print!("Player : {}\r", player_pos);

            // let distance = Vec2::new(ball_pos.x - player_pos.x, ball_pos.y - player_pos.y).length();
            // let combined_radius = 25.6 + 15.0;

            // if distance <= combined_radius {
            //     _ball.velocity.x = -_ball.velocity.x;
            // }

                for mut text in &mut query {
                    text.sections[1].value = format!("{:+08.3}", ball_pos.x);
                }

            if ball_pos.x - 10.0 <= player_pos.x + 15.0 
            && ball_pos.y <= player_pos.y - 75.0
            && ball_pos.y >= player_pos.y + 75.0 {
                _ball.velocity.x = -_ball.velocity.x;
            }

        }
    }
}