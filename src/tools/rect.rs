use druid::EventCtx;

use crate::{data::grid_list::GridList, shapes::rect::RectShape};

use super::ToolControl;

pub struct RectTool;

impl RectTool {
    pub fn new() -> Self {
        Self {}
    }
}

impl ToolControl for RectTool {
    fn start(
        &mut self,
        ctx: &mut EventCtx,
        event: &druid::MouseEvent,
        shape_list: &mut crate::shapes::ShapeList,
        grid_list: &mut GridList,
    ) {
        let (cell_width, cell_height) = grid_list.cell_size;
        let mouse_row = (event.pos.y / cell_height) as usize;
        let mouse_col = (event.pos.x / cell_width) as usize;
        shape_list.add_shape(Box::new(RectShape::new(mouse_row, mouse_col)));
    }

    fn draw(
        &mut self,
        ctx: &mut EventCtx,
        event: &druid::MouseEvent,
        shape_list: &mut crate::shapes::ShapeList,
        grid_list: &mut GridList,
    ) {
        if let Some(rect) = shape_list.data.last_mut() {
            if let Some(mut rect) = rect.as_any_mut().downcast_mut::<RectShape>() {
                // TODO: Boundary check for row / col access
                let (cell_width, cell_height) = grid_list.cell_size;
                let mouse_row = (event.pos.y / cell_height) as usize;
                let mouse_col = (event.pos.x / cell_width) as usize;
                rect.end = (mouse_row, mouse_col);
            }
        }
    }

    fn input(
        &mut self,
        ctx: &mut EventCtx,
        event: &druid::KeyEvent,
        shape_list: &mut crate::shapes::ShapeList,
        grid_list: &mut GridList,
    ) {
    }

    fn end(
        &mut self,
        ctx: &mut EventCtx,
        event: &druid::MouseEvent,
        shape_list: &mut crate::shapes::ShapeList,
        grid_list: &mut GridList,
    ) {
    }
}