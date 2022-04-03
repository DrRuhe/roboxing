// disable console opening on windows
#![windows_subsystem = "windows"]

use std::f32::consts::PI;

use bevy::prelude::*;

/// Used to help identify our main camera
#[derive(Component)]
struct MainCamera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(get_cursor_coords.chain(turn_towards_location))
        .add_system(move_forward)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("icon.png"),
            ..Default::default()
        })
        .insert(ForwardMovement(20.0))
        .insert(TurnSpeed(PI / 2.0))
        .insert(Timer::from_seconds(0.5, true));
}

#[derive(Component)]
struct TurnSpeed(f32);

fn turn_towards_location(
    In(cursor): In<Option<Vec2>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &GlobalTransform, &TurnSpeed), Without<MainCamera>>,
) {
    if let Some(cursor_location) = cursor {
        for (mut transform, global_transform, turnspeed) in query.iter_mut() {
            //transform.look_at(dbg!(cursor_location.extend(0.0)),Vec3::X);

            let pos = global_transform.translation.truncate();
            let look_direction = cursor_location - pos;
            assert_ne!(
                look_direction,
                Vec2::NAN,
                "look direction is NaN. \n cursor_location: {cursor_location} \n pos: {pos}"
            );
            if look_direction != Vec2::ZERO {
                let angle = transform.up().truncate().angle_between(look_direction);
                let allowed_angle = turnspeed.0 * time.delta_seconds();
                let angle = angle.clamp(-allowed_angle, allowed_angle);
                transform.rotate(Quat::from_rotation_z(angle));

                //let angle = Vec2::Y.angle_between(look_direction);
                //transform.rotation = Quat::from_rotation_z(angle);
            }
        }
    }
}

fn turn_towards<C: Component, T: Component>(
    time: Res<Time>,
    mut entities: Query<
        (&mut Transform, &GlobalTransform, &TurnSpeed),
        (With<C>, Without<MainCamera>),
    >,
    targets: Query<&GlobalTransform, With<T>>,
) {
    for (mut transform, global_transform, turnspeed) in entities.iter_mut() {
        let location = global_transform.translation.truncate();
        let closest_target_location =
            targets
                .iter()
                .map(|t| t.translation.truncate())
                .fold(None, |a, b| {
                    if let Some(a) = a {
                        if location.distance_squared(a) > location.distance_squared(b) {
                            Some(b)
                        } else {
                            Some(a)
                        }
                    } else {
                        Some(b)
                    }
                });

        if let Some(closest_target_location) = closest_target_location {
            let look_direction = closest_target_location - location;
            assert_ne!(
                look_direction,
                Vec2::NAN,
                "look direction is NaN. \n cursor_location: {closest_target_location} \n pos: {location}"
            );
            if look_direction != Vec2::ZERO {
                let angle = transform.up().truncate().angle_between(look_direction);
                let allowed_angle = turnspeed.0 * time.delta_seconds();
                let angle = angle.clamp(-allowed_angle, allowed_angle);
                transform.rotate(Quat::from_rotation_z(angle));

                //let angle = Vec2::Y.angle_between(look_direction);
                //transform.rotation = Quat::from_rotation_z(angle);
            }
        }
    }
}

#[derive(Component)]
struct ForwardMovement(f32);

#[derive(Component)]
struct Target {
    distance: f32,
}

fn move_forward(time: Res<Time>, mut query: Query<(&mut Transform, &ForwardMovement)>) {
    for (mut transform, movement) in query.iter_mut() {
        //let distance = transform.translation.distance(other)

        let direction = transform.up();
        transform.translation += direction * movement.0 * time.delta_seconds();
    }
}

fn place_at_location(
    In(cursor): In<Option<Vec2>>,
    mut query: Query<&mut GlobalTransform, Without<MainCamera>>,
) {
    if let Some(cursor_location) = cursor {
        for mut global_transform in query.iter_mut() {
            global_transform.translation = cursor_location.extend(0.0);
        }
    }
}

fn get_cursor_coords(
    windows: Res<Windows>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) -> Option<Vec2> {
    let (camera, camera_transform) = camera_query
        .get_single()
        .expect("There should only be one camera.");

    let wnd = windows
        .get(camera.window)
        .expect("get the window that the camera is displaying to.");

    // cursor is inside window?
    if let Some(screen_pos) = wnd.cursor_position() {
        let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);
        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;
        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix.inverse();
        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(0.0));
        // reduce it to a 2D value
        let world_pos: Vec2 = world_pos.truncate();
        Some(world_pos)
    } else {
        None
    }
}
