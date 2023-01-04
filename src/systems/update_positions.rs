use crate::components::Drawable;

pub fn update_positions(drawable_items: Vec<&mut impl Drawable>) {
    for drawable in drawable_items {
        drawable.update_position();
    }
}
