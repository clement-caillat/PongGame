use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
pub struct BallPlugin;

use crate::player::{Player, Score};

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
    
    commands.spawn((
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
            speed: 50.0
        },
    ));
}


fn ball_movement(
    mut ball: Query<(&mut Transform, &mut Ball)>,
    window: Query<&Window>,
    time: Res<Time>,
    mut score: Query<(&mut Text, &mut Score)>
) {

    let win = window.get_single().unwrap();

    for (mut text , mut score)in &mut score {
        for (mut transform, mut _ball) in &mut ball {
            
            transform.translation.x += _ball.velocity.x * _ball.speed * time.delta_seconds();
            transform.translation.y += _ball.velocity.y * _ball.speed * time.delta_seconds();
            
            if transform.translation.x + 10.0 >= win.width() / 2.0 {
                transform.translation.x = 0.0;
                _ball.velocity.x = -_ball.velocity.x;
                score.player1_score += 1;
                
            } else if transform.translation.x - 10.0 <= -win.width() / 2.0 {
                transform.translation.x = 0.0;
                _ball.velocity.x = -_ball.velocity.x;
                score.player2_score += 1;
            }

            text.sections[0].value = format!("Red : {} | Blue : {}\r", score.player1_score, score.player2_score);

            if transform.translation.y + 10.0 >= win.height() / 2.0  
            || transform.translation.y - 10.0 <= -win.height() / 2.0 {
                _ball.velocity.y = -_ball.velocity.y;
            }

        }
    }
}

fn ball_player_collision(
    mut ball: Query<(&Transform, &mut Ball)>,
    player: Query<(&Transform, &Player)>,
) {


    for (ball_transform, mut _ball) in &mut ball {
        for (player_transform, _) in &player {

            let ball_radius = 10.0;
            let ball_pos = ball_transform.translation;
            let player_pos = player_transform.translation;
            let player_width = 30.0;
            let player_height = 150.0;
            let player_top = player_pos.y + player_height * 0.5;
            let player_bottom = player_pos.y - player_height * 0.5;

            if ball_pos.x + ball_radius >= player_pos.x - player_width * 0.5
                && ball_pos.x - ball_radius <= player_pos.x + player_width * 0.5
            {
                if ball_pos.y + ball_radius >= player_bottom && ball_pos.y - ball_radius <= player_top {
                    _ball.velocity.x = -_ball.velocity.x;
                }
            }
        }
    }
}