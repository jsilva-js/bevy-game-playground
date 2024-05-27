use bevy::prelude::*;

#[derive(Component)]
pub struct MainPlayer;


#[derive(Component)]
pub struct Speed {
    pub value: f32
}


pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    let player = (PbrBundle {
            mesh: meshes.add(Mesh::from(Cuboid::new(1.0, 1.0, 1.0))),
            material: materials.add(Color::BLUE),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        Speed { value: 2.0 },
        MainPlayer
    );
    commands.spawn(player);
}