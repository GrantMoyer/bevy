use bevy::{prelude::*, tasks::prelude::*};
use rand::random;

struct Velocity(Vec2);

fn spawn_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dComponents::default());
    let texture_handle = asset_server.load("assets/branding/icon.png").unwrap();
    for _ in 0..128 {
        commands
            .spawn(SpriteComponents {
                material: materials.add(texture_handle.into()),
                translation: Translation::new(0.0, 0.0, 0.0),
                scale: Scale(0.1),
                ..Default::default()
            })
            .with(Velocity(
                20.0 * Vec2::new(random::<f32>() - 0.5, random::<f32>() - 0.5),
            ));
    }
}

// Move sprties according to their velocity
fn move_system(pool: Res<ComputeTaskPool>, mut sprites: Query<(&mut Translation, &Velocity)>) {
    // Compute the new location of each sprite in parallel on the ComputeTaskPool using batches of 32 sprties
    sprites.iter().par_iter(32).for_each(&pool, |(mut t, v)| {
        t.0 += v.0.extend(0.0);
    });
}

// Bounce sprties outside the window
fn bounce_system(
    pool: Res<ComputeTaskPool>,
    windows: Res<Windows>,
    mut sprites: Query<(&Translation, &mut Velocity)>,
) {
    let Window { width, height, .. } = windows.get_primary().expect("No primary window");
    let left = *width as f32 / -2.0;
    let right = *width as f32 / 2.0;
    let bottom = *height as f32 / -2.0;
    let top = *height as f32 / 2.0;
    sprites
        .iter()
        .par_iter(32)
        // Filter out sprites that don't need to be bounced
        .filter(|(t, _)| !(left < t.x() && t.x() < right && bottom < t.y() && t.y() < top))
        // For simplicity, just reverse the velocity; don't use realistic bounces
        .for_each(&pool, |(_, mut v)| {
            v.0 = -v.0;
        });
}

fn main() {
    App::build()
        .add_default_plugins()
        .add_startup_system(spawn_system.system())
        .add_system(move_system.system())
        .add_system(bounce_system.system())
        .run();
}
