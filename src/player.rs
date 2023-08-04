use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        app.add_systems(Update, (move_player, move_player_2));
    }
}

#[derive(Component)]
pub struct Player {
    id: i8
}



fn spawn_player(mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2 { x: 30.0, y: 150.0 }))).into(),
            material: materials.add(ColorMaterial::from(Color::RED)),
            transform: Transform {
                translation: Vec3 { x: -450.0, y: 0.0, z: (0.0) },
                ..default()
            },
            ..default()
        },
        Player {
            id: 1
        }
    ));

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2 { x: 30.0, y: 150.0 }))).into(),
            material: materials.add(ColorMaterial::from(Color::BLUE)),
            transform: Transform {
                translation: Vec3 { x: 450.0, y: 0.0, z: (0.0) },
                ..default()
            },
            ..default()
        },
        Player {
            id: 2
        }
    ));

}


fn move_player(
    mut raket: Query<(&mut Transform, &Player)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    window: Query<&Window>
) {

    let win = window.get_single().unwrap();

    for (mut transform, raket) in &mut raket {

        if raket.id == 1 {
            if input.pressed(KeyCode::W) {
                if transform.translation.y + 75.0 <= win.height() / 2.0 {
                    transform.translation.y += 400.0 * time.delta_seconds();
                }
            }
            if input.pressed(KeyCode::S) {
                if transform.translation.y - 75.0 >= -win.height() / 2.0 {
                    transform.translation.y -= 400.0 * time.delta_seconds();
                }
            }
        }
    }
}

fn move_player_2(
    mut raket: Query<(&mut Transform, &Player)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    window: Query<&Window>,
) {
    let win = window.get_single().unwrap();

    for (mut transform, raket) in &mut raket {
        // Assuming you have a unique identifier for player 2, like player_id = 2
        if raket.id == 2 {
            if input.pressed(KeyCode::O) {
                if transform.translation.y + 75.0 <= win.height() / 2.0 {
                    transform.translation.y += 400.0 * time.delta_seconds();
                }
            }
            if input.pressed(KeyCode::L) {
                if transform.translation.y - 75.0 >= -win.height() / 2.0 {
                    transform.translation.y -= 400.0 * time.delta_seconds();
                }
            }
        }
    }
}