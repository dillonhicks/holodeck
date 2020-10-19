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
    pub struct MinimapIds {
        canvas,
        bordered_background,
        player_marker,
        point_map,
        points[],
    }
}


#[derive(Copy, Clone)]
pub enum MinimapMode {
    Normal,
    Large,
    Hidden,
}

impl MinimapMode {
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
// struct Bitmap([u8;  (100 * 100)/8]);
// impl Bitmap {
//     /// return true if the value was set
//     pub fn set(&mut self, x: u32, y: u32) -> bool{
//         let bitpos = x + (y * 100);
//         let bytepos = (bitpos / 8) as usize;
//         let byteoff = (bitpos % 8) as u8;
//
//         let value = self.0[bytepos];
//         self.0[bytepos] |= (1 << byteoff);
//         value != self.0[bytepos]
//     }
//
//     pub fn clear(&mut self) {
//         self.0.iter_mut().map(|x| *x = 0);
//     }
// }

pub struct MinimapApp {
    minimap_mode: MinimapMode,
    /*    bitmap: RgbaImage,
     *    bitmap: Bitmap, */
}


impl MinimapApp {
    pub fn new() -> Self {
        MinimapApp {
            minimap_mode: MinimapMode::Normal,
            //          bitmap: unsafe { MaybeUninit::zeroed().assume_init() }
        }
    }
}

pub struct Minimap {
    parent_id: conrod::widget::Id,
    ids:       MinimapIds,
    app:       MinimapApp,
}

impl Minimap {
    const MARGIN: conrod::Scalar = 30.0;
    const MAX_ITEMS: usize = 10_000;

    pub fn new(
        parent_id: conrod::widget::Id,
        window: &mut Window,
    ) -> Self {
        Minimap {
            parent_id,
            ids: MinimapIds::new(window.conrod_ui_mut().widget_id_generator()),
            app: MinimapApp::new(),
        }
    }

    fn is_hidden(&self) -> bool {
        if let MinimapMode::Hidden = self.app.minimap_mode {
            true
        } else {
            false
        }
    }

    pub fn toggle_hidden(&mut self) {
        self.app.minimap_mode = MinimapMode::toggle_hidden(self.app.minimap_mode);
    }

    pub fn toggle_size(&mut self) {
        self.app.minimap_mode = MinimapMode::toggle_large(self.app.minimap_mode);
    }

    // pub fn clear_bitmap(&mut self) {
    //     self.app.bitmap.clear();//iter_mut().map(|x| *x = 0);
    // }
}

impl Draw for Minimap {
    fn draw(
        &mut self,
        event: &mut DrawEvent,
    ) {
        let DrawEvent {
            eye: _,
            eye_dir: _,
            world: _,
            //  window,
            ui,
            ..
        } = event;



        use conrod::{
            widget,
            Colorable,
            Positionable,
            Sizeable,
            Widget,
        };


        let (canvas_h, canvas_w) = match self.app.minimap_mode {
            MinimapMode::Normal => (150.0, 150.0),
            MinimapMode::Hidden => (0.0, 0.0),
            MinimapMode::Large => (300.0, 300.0),
        };


        widget::Canvas::new()
            .pad(Self::MARGIN)
            .align_top()
            .pad(Self::MARGIN)
            .h(canvas_h)
            .w(canvas_w)
            .align_right()
            .align_top()
            //.parent(self.parent_id)
            .color(conrod::color::TRANSPARENT)
            .set(self.ids.canvas, ui);

        if self.is_hidden() {
            return;
        }

        let ids = &mut self.ids;


        widget::BorderedRectangle::new([canvas_w, canvas_h])
            .border(3.0)
            .color(Color::holodeck_space_grey().with_a(0.3).into())
            .border_color(Color::holodeck_plasma().with_a(0.3).into())
            .middle_of(ids.canvas)
            //.bottom_right_with_margin_on(self.parent_id, 0.0)
            .set(ids.bordered_background, ui);

        // let min_x = 0.0f64;
        // let min_z = canvas_w;
        // let max_x = 0.0f64;
        // let max_z = canvas_h;
        //
        // let min_x = event.world.bounds.min.x as conrod::Scalar;
        // let min_z = event.world.bounds.min.z as conrod::Scalar;
        // let max_x =event.world.bounds.max.x as conrod::Scalar;
        // let max_z = event.world.bounds.max.z as conrod::Scalar;
        //
        // let mut points = Vec::with_capacity(event.world.objects.len());
        //
        // for obj in event.world.objects.values() {
        //     let translation = obj.scene_node().data().local_translation();
        //     let x = translation.x as conrod::Scalar;
        //     let y = translation.z as conrod::Scalar;
        //
        //     points.push([x, y]);
        // }
        //
        // widget::EnvelopeEditor::new(&points[..], min_x, min_z, max_x, max_z)
        //     .border(3.0)
        //     .point_radius(4.0)
        //     .color(Color::holodeck_space_grey().with_a(0.3).into())
        //     .border_color(Color::holodeck_plasma().with_a(0.3).into())
        //     .middle_of(ids.canvas)
        //     .set(ids.point_map, ui);

        if ids.points.len() < event.world.objects.len() {
            ids.points
                .resize(event.world.objects.len(), &mut ui.widget_id_generator());
        }

        // let bitmap = &mut self.app.bitmap;
        // bitmap.clear();

        // use image::{
        //     GenericImage,
        //     GenericImageView,
        //     ImageBuffer,
        //     RgbImage,
        // };

        // Construct a new RGB ImageBuffer with the specified width and height.


        let iter = event
            .world
            .objects
            .values()
            .zip(ids.points.iter())
            .take(Minimap::MAX_ITEMS);

        use crate::world::Entity;

        for (obj, id) in iter {
            let translation = obj.scene_node().data().local_translation();
            let x = (translation.x * (canvas_w as f32 / event.world.bounds.width())) as f64;
            let y = (translation.z * (canvas_h as f32 / event.world.bounds.depth())) as f64;
            // let translation = obj.scene_node().data().local_translation();
            // let x = (translation.x + (event.world.bounds.width() / 2.0) / event.world.bounds.width());
            // let y =(translation.y + (event.world.bounds.depth() / 2.0)) / event.world.bounds.depth());

            widget::Rectangle::fill_with([1.0, 1.0], conrod::color::DARK_ORANGE)
                .x_y_relative_to(ids.bordered_background, x as conrod::Scalar, y as conrod::Scalar)
                .set(*id, ui);

            // //self.img.put_pixel(x as u32, y as u32, Rgba::from([255, 255, 255, 255]));
            // if bitmap.set(x as u32, y as u32) {
            //
            // }
        }

        let x = (event.world.position.x * (canvas_w as f32 / event.world.bounds.width())) as f64;
        let y = (event.world.position.z * (canvas_h as f32 / event.world.bounds.depth())) as f64;
        widget::Rectangle::fill_with([2.0, 2.0], Color::holodeck_grid().into())
            .x_y_relative_to(ids.bordered_background, x as conrod::Scalar, y as conrod::Scalar)
            .set(ids.player_marker, ui);
    }
}
