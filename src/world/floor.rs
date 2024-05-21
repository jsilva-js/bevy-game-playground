use bevy::prelude::*;
use bevy::render::mesh::Mesh;
use bevy_math::prelude::Plane3d;

pub fn spawn_floor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let floor = PbrBundle {
        mesh: meshes.add(Mesh::from(Plane3d::new(Vec3::new(15.0, 15.0, 15.0)))),
        material: materials.add(Color::DARK_GREEN),
        ..default()
    };

    commands.spawn(floor);
}
