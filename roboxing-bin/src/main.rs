// disable console opening on windows
#![windows_subsystem = "windows"]

use std::{f32::consts::PI, time::Duration};

use bevy::prelude::*;
use rand::prelude::*;

/// Used to help identify our main camera
#[derive(Component)]
struct MainCamera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(get_cursor_coords.chain(place_at_location::<Cursor>))
        .add_system(turn_towards::<Turret, Cursor>)
        .add_system(turn_towards::<Tank, Checkpoint>)
        .add_system(move_checkpoint)
        .add_system(move_forward)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);

    commands.spawn_bundle(CursorBundle::default());

    commands
        .spawn_bundle(TankBundle::new(&asset_server))
        .with_children(|parent| {
            parent.spawn_bundle(TurretBundle::new(&asset_server));
        });

    // cursor entity
    commands.spawn_bundle(CheckpointBundle::new(&asset_server));
}

#[derive(Bundle)]
struct TankBundle {
    forward_movement: ForwardMovement,
    turn_speed: TurnSpeed,
    tank: Tank,
    #[bundle]
    sprite: SpriteBundle,
}

impl TankBundle {
    fn new(asset_server: &Res<AssetServer>) -> Self {
        Self {
            forward_movement: ForwardMovement(50.0),
            turn_speed: TurnSpeed(18.0*DEG),
            tank: Default::default(),
            sprite: SpriteBundle {
                texture: asset_server.load("icon.png"),
                ..Default::default()
            },
        }
    }
}

#[derive(Bundle)]
struct TurretBundle {
    turn_speed: TurnSpeed,
    turret: Turret,
    #[bundle]
    sprite: SpriteBundle,
}

const DEG:f32 = 2.0*PI/360.0;

impl TurretBundle {
    fn new(asset_server: &Res<AssetServer>) -> Self {
        Self {
            turn_speed: TurnSpeed(25.0*DEG),
            turret: Default::default(),
            sprite: SpriteBundle {
                texture: asset_server.load("icon.png"),
                transform: Transform {
                    scale: Vec3::splat(0.5),
                    translation: Vec3::new(0.0, 0.0, 1.0),
                    ..Default::default()
                },
                ..Default::default()
            },
        }
    }
}

#[derive(Bundle)]
struct CheckpointBundle {
    checkpoint: Checkpoint,
    timer: Timer,
    #[bundle]
    sprite: SpriteBundle,
}

impl CheckpointBundle {
    fn new(asset_server: &Res<AssetServer>) -> Self {
        Self {
            timer: Timer::new(Duration::from_secs(3), true),
            checkpoint: Default::default(),
            sprite: SpriteBundle {
                texture: asset_server.load("icon.png"),
                transform: Transform {
                    scale: Vec3::splat(0.1),
                    translation: Vec3::new(0.0, 0.0, 2.0),
                    ..Default::default()
                },
                ..Default::default()
            },
        }
    }
}

#[derive(Component, Default)]
struct Cursor;

#[derive(Bundle, Default)]
struct CursorBundle {
    cursor: Cursor,
    global_transform: GlobalTransform,
}

#[derive(Component, Default)]
struct Turret;
#[derive(Component, Default)]
struct Tank;

#[derive(Component, Default)]
struct Checkpoint;

#[derive(Component)]
struct TurnSpeed(f32);

impl Default for TurnSpeed {
    fn default() -> Self {
        Self(PI / 2.0)
    }
}
#[derive(Component)]
struct ForwardMovement(f32);

impl Default for ForwardMovement {
    fn default() -> Self {
        Self(20.0)
    }
}

fn move_checkpoint(
    time: Res<Time>,
    mut query: Query<(&mut Timer, &mut Transform), With<Checkpoint>>,
) {
    for (mut timer, mut transform) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            let mut rng = rand::thread_rng();

            let x: f32 = rng.gen_range(-300.0..300.0);
            let y: f32 = rng.gen_range(-300.0..300.0);

            let new_checkpoint_location = Vec3::new(x, y, 0.0);

            transform.translation = new_checkpoint_location;
        }
    }
}

fn turn_towards<C: Component, T: Component>(
    time: Res<Time>,
    mut entities: Query<(&mut Transform, &GlobalTransform, &TurnSpeed), With<C>>,
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
                let angle = global_transform
                    .up()
                    .truncate()
                    .angle_between(look_direction);
                let allowed_angle = turnspeed.0 * time.delta_seconds();
                let angle = angle.clamp(-allowed_angle, allowed_angle);
                transform.rotate(Quat::from_rotation_z(angle));

                //let angle = Vec2::Y.angle_between(look_direction);
                //transform.rotation = Quat::from_rotation_z(angle);
            }
        }
    }
}

fn move_forward(time: Res<Time>, mut query: Query<(&mut Transform, &ForwardMovement)>) {
    for (mut transform, movement) in query.iter_mut() {
        //let distance = transform.translation.distance(other)

        let direction = transform.up();
        transform.translation += direction * movement.0 * time.delta_seconds();
    }
}

fn place_at_location<T: Component>(
    In(cursor): In<Option<Vec2>>,
    mut query: Query<&mut GlobalTransform, With<T>>,
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
