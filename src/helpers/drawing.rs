use crate::api::display::Point;

/// Method checks whether the position is within the provided dimensions
///
/// # Returns
///
/// * True if the position is outside the dimensions
pub fn get_is_position_outside_dimensions(dimensions: &Point<i64>, position: &Point<i64>) -> bool {
    position.width < 0
        || position.height < 0
        || position.width >= dimensions.width
        || position.height >= dimensions.height
}
