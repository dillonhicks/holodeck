use crate::deps::kiss3d::conrod;

use crate::deps::kiss3d::conrod::{
    position::Positionable,
    widget_ids,
    Borderable,
};

use crate::deps::kiss3d::window::Window;

use crate::{
    theme::Color,
    ui::{
        Draw,
        DrawEvent,
    },
};



// Generate a unique `WidgetId` for each widget.
widget_ids! {
    pub struct InfoPaneIds {
        canvas,
        bordered_background,
        text
    }
}



#[derive(Copy, Clone)]
pub enum InfoPaneMode {
    Normal,
    Large,
    Hidden,
}

impl InfoPaneMode {
    pub fn toggle_hidden(me: Self) -> Self {
        match me {
            Self::Large | Self::Normal => Self::Hidden,
            Self::Hidden => Self::Normal,
        }
    }

    pub fn toggle_large(me: Self) -> Self {
        match me {
            Self::Large => Self::Normal,
            Self::Normal => Self::Large,
            Self::Hidden => Self::Large,
        }
    }
}


pub struct InfoPaneApp {
    InfoPane_mode: InfoPaneMode,
    /*    bitmap: RgbaImage,
     *    bitmap: Bitmap, */
}


impl InfoPaneApp {
    pub fn new() -> Self {
        InfoPaneApp {
            InfoPane_mode: InfoPaneMode::Normal,
            //          bitmap: unsafe { MaybeUninit::zeroed().assume_init() }
        }
    }
}

pub struct InfoPane {
    parent_id: conrod::widget::Id,
    ids:       InfoPaneIds,
    app:       InfoPaneApp,
}

impl InfoPane {
    const MARGIN: conrod::Scalar = 30.0;
    const MAX_ITEMS: usize = 10_000;

    pub fn new(
        parent_id: conrod::widget::Id,
        window: &mut Window,
    ) -> Self {
        InfoPane {
            parent_id,
            ids: InfoPaneIds::new(window.conrod_ui_mut().widget_id_generator()),
            app: InfoPaneApp::new(),
        }
    }

    fn is_hidden(&self) -> bool {
        if let InfoPaneMode::Hidden = self.app.InfoPane_mode {
            true
        } else {
            false
        }
    }

    pub fn toggle_hidden(&mut self) {
        self.app.InfoPane_mode = InfoPaneMode::toggle_hidden(self.app.InfoPane_mode);
    }

    pub fn toggle_size(&mut self) {
        self.app.InfoPane_mode = InfoPaneMode::toggle_large(self.app.InfoPane_mode);
    }
}

impl Draw for InfoPane {
    fn draw(
        &mut self,
        event: &mut DrawEvent,
    ) {
        let DrawEvent {
            eye: _,
            eye_dir: _,
            world,
            // window,
            ui,
            ..
        } = event;


        use conrod::{
            widget,
            Colorable,
            Labelable,
            Positionable,
            Sizeable,
            Widget,
        };


        let (canvas_h, canvas_w) = match self.app.InfoPane_mode {
            InfoPaneMode::Normal => (150.0, 150.0),
            InfoPaneMode::Hidden => (0.0, 0.0),
            InfoPaneMode::Large => (300.0, 300.0),
        };

        // widget::Canvas::new()
        //     .pad(Self::MARGIN)
        //     .align_top()
        //     .pad(Self::MARGIN)
        //     .align_left()
        //     .h(canvas_h)
        //     .w(canvas_w)
        //     .color(conrod::color::TRANSPARENT)
        //     .parent(self.parent_id)
        //     .set(self.ids.canvas, ui);

        if self.is_hidden() {
            return;
        }

        let ids = &self.ids;


        // let canvas_rect = ui.rect_of(ids.canvas).unwrap();
        // let w = canvas_rect.w();
        // let h = canvas_rect.h() * 5.0 / 6.0;

        widget::BorderedRectangle::new([canvas_w, canvas_h])
            .border(3.0)
            .color(Color::holodeck_space_grey().with_a(0.3).into())
            .border_color(Color::holodeck_plasma().with_a(0.3).into())
            .align_right()
            //.bottom_left_with_margin_on(self.parent_id, 0.0)
            .set(ids.bordered_background, ui);


        widget::Text::new(&format!("{} Entities", world.objects.len()))
            .color(conrod::color::WHITE)
            .middle_of(ids.bordered_background)
            .set(ids.text, ui);
    }
}
