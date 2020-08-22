use bevy::{ecs::rayon::prelude::*, prelude::*};
use std::{
    thread,
    time::{Duration, Instant},
};

fn spawn_system(mut commands: Commands) {
    for i in 0..16usize {
        commands.spawn((i,));
    }
}

fn square_system(mut nums: Query<&mut usize>) {
    nums.iter().into_par_iter().for_each(|mut n| {
        thread::sleep(Duration::from_secs(1));
        *n = *n * *n;
    });
}

fn print_system(num: &usize) {
    print!("{} ", num);
}

fn main() {
    let t0 = Instant::now();
    App::build()
        .add_startup_system(spawn_system.system())
        .add_system(square_system.system())
        .add_system(print_system.system())
        .run();
    let t1 = Instant::now();
    println!("\nTook {:.3}s", (t1 - t0).as_secs_f32());
}
