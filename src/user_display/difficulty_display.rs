use crate::{
    api::display::{
        create_map, get_screen_size, map_from_str, DisplayControllerError, Layout, Map, Point,
    },
    entities::Borders,
};

pub struct DifficultyDisplay<'name> {
    pub name: &'name str,
    pub level: u32,
    pub position: Point<i64>,
    pub layout: Layout,
}

type DifficultyResult<T> = Result<T, DisplayControllerError>;

impl<'name> DifficultyDisplay<'name> {
    pub fn new(name: &'name str, level: u32, position: Point<i64>) -> DifficultyResult<Self> {
        let screen_size = get_screen_size();

        let size = Point {
            width: screen_size.width - 12,
            height: screen_size.height / 5,
        };

        let mut parent_element = Layout::new(&size, None);

        let border = Borders::new(&(size - (1 as i64).into()))?;

        parent_element.draw_map(&border.drawable.layout.map, position, &Default::default())?;

        Ok(DifficultyDisplay {
            name,
            level,
            position,
            layout: parent_element,
        })

        // let map = map_from_str(str, color);
    }
}
