use crate::deps::kiss3d::conrod;



use crate::deps::kiss3d::conrod::widget_ids;

use crate::deps::kiss3d::window::Window;


use crate::ui::{
    Draw,
    DrawEvent,
    InfoPane,
    Minimap,
};



// Generate a unique `WidgetId` for each widget.
widget_ids! {
    pub struct HudIds {
        canvas,
    }
}

pub struct HeadsUpDisplay {
    ids:       HudIds,
    minimap:   Minimap,
    info_pane: InfoPane,
}

impl HeadsUpDisplay {
    pub fn new(window: &mut Window) -> Self {
        window.conrod_ui_mut().theme = Self::theme();
        let ids = HudIds::new(window.conrod_ui_mut().widget_id_generator());

        let parent_id = ids.canvas;

        HeadsUpDisplay {
            ids,
            minimap: Minimap::new(parent_id, window),
            info_pane: InfoPane::new(parent_id, window),
        }
    }

    pub fn minimap(&self) -> &Minimap {
        &self.minimap
    }

    pub fn minimap_mut(&mut self) -> &mut Minimap {
        &mut self.minimap
    }

    pub fn theme() -> conrod::Theme {
        use conrod::position::{
            Align,
            Direction,
            Padding,
            Position,
            Relative,
        };
        let charcoal = conrod::color::CHARCOAL;
        charcoal.alpha(0.5);

        conrod::Theme {
            name:                   "Demo Theme".to_string(),
            padding:                Padding::none(),
            x_position:             Position::Relative(Relative::Align(Align::Start), None),
            y_position:             Position::Relative(Relative::Direction(Direction::Backwards, 20.0), None),
            background_color:       charcoal,
            shape_color:            conrod::color::LIGHT_CHARCOAL,
            border_color:           conrod::color::BLACK,
            border_width:           0.0,
            label_color:            conrod::color::WHITE,
            font_id:                None,
            font_size_large:        26,
            font_size_medium:       18,
            font_size_small:        12,
            widget_styling:         conrod::theme::StyleMap::default(),
            mouse_drag_threshold:   0.0,
            double_click_threshold: std::time::Duration::from_millis(500),
        }
    }
}

impl Draw for HeadsUpDisplay {
    fn draw(
        &mut self,
        event: &mut DrawEvent,
    ) {
        // widget::Canvas::new()
        //     .h()
        //     .w(0)
        //     .color(conrod::color::TRANSPARENT)
        //     .set(self.ids.canvas, event.ui);
        //

        self.minimap.draw(event);
        // self.info_pane.draw(event);
    }
}
