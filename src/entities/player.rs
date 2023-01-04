use crossterm::event::{Event, KeyCode};

use crate::{
    api::display::{Element, Map, Point},
    components::Drawable,
};

use super::controller::Controller;

pub struct Player {
    drawable: Drawable,
}

impl Player {
    pub fn new() -> Self {
        Self {
            drawable: Drawable {
                map: Map::new(
                    &Point { x: 1, y: 1 },
                    Some(Element::new_default_colors('😃')),
                ),
                location: Point { x: 0, y: 0 },
            },
        }
    }
}

// impl Controller for Player {
//     fn up(&mut self) -> &mut Self {
//         todo!()
//     }

//     fn down(&mut self) -> &mut Self {
//         todo!()
//     }

//     fn left(&mut self) -> &mut Self {
//         todo!()
//     }

//     fn right(&mut self) -> &mut Self {
//         todo!()
//     }

//     fn handle_event(&mut self, event: crossterm::event::Event) -> &mut Self {
//         if event == Event::Key(KeyCode::Up.into()) {
//             return self.up();
//         }

//         self
//     }
// }
