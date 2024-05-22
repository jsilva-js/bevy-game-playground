use bevy::prelude::*;

pub fn spawn_light(mut commands: Commands) {
    let light = PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(2.0, 2.0, 1.0),
        ..default()
    };

    commands.spawn(light);
}

