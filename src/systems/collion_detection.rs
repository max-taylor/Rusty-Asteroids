use crate::components::Drawable;

pub fn run_collision_detection(player: &impl Drawable, asteroids: Vec<&impl Drawable>) {
    for asteroid in asteroids {
        asteroid.get_drawable_state().layout
    }
}
