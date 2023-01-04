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

pub fn get_is_position_outside_dimensions_with_offset(
    dimensions: &Point<i64>,
    position: &Point<i64>,
    offset: &Point<i64>,
) -> bool {
    if get_is_position_outside_dimensions(dimensions, position) {
        return true;
    }

    if position.width < offset.width || position.height < offset.height {
        return true;
    }

    false
}
