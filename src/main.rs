use bevy::prelude::*;
use bevy::render::mesh::Mesh;
mod player;
use player::PlayerPlugin;


fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(PlayerPlugin)
    .add_systems((spawn_floor, spawn_camera, spawn_light)).run();
}

// fn spawn_player(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>
// ) {
//     let player = PbrBundle {
//         mesh: meshes.add(Mesh::from(shape::Cube::new(1.0))),
//         material: materials.add(Color::BLUE.into()),
//         transform: Transform::from_xyz(0.0, 0.0, 0.0),
//         ..default()
//     };

//     commands.spawn(player);
// }

fn spawn_light(mut commands: Commands) {
    let light = PointLightBundle {
        point_light: PointLight {
            intensity: 2000.0,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 5.0, 0.0),
        ..default()
    };

    commands.spawn(light);
}

fn spawn_camera(mut commands: Commands) {
    let camera = Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    };

    commands.spawn(camera);
}

fn spawn_floor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let floor = PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane::from_size(15.0))),
        material: materials.add(Color::DARK_GREEN.into()),
        ..default()
    };

    commands.spawn(floor);
}


pub fn setup(mut commands: Commands) {
    commands.spawn(Person {
        name: "alex".to_string(),
    });
}

#[derive(Component)]
pub struct Person {
    pub name: String,
}
