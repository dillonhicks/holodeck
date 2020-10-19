use crate::{
    deps::{
        kiss3d::{
            resource::{
                Mesh,
                MeshManager,
            },
            window::Window,
        },
        na::{
            Point3,
            Translation3,
            Vector3,
        },
    },
    engine::GraphicsNode,
    theme::Color,
    world::entity::Id,
};
use std::{
    cell::RefCell,
    rc::Rc,
};

#[derive(Clone)]
pub struct Shell {
    pub id:    Option<Id>,
    pub gfx:   GraphicsNode,
    pub color: Color,
}

impl Shell {
    const MESH_NAME: &'static str = "holodeck_shell";

    pub fn new(
        r: f32,
        color: Color,
        pos: Point3<f32>,
        window: &mut Window,
    ) -> Shell {
        let mut node = window.add_mesh(Self::mesh(), Vector3::new(r * 2.0, r * 2.0, r * 2.0));
        node.append_translation(&Translation3 { vector: pos.coords });
        let mut shell = Shell {
            id: None,
            gfx: node,
            color,
        };

        shell.set_color(color);
        shell
    }

    pub fn scene_node(&self) -> &GraphicsNode {
        &self.gfx
    }

    pub fn scene_node_mut(&mut self) -> &mut GraphicsNode {
        &mut self.gfx
    }

    pub fn set_color(
        &mut self,
        color: Color,
    ) {
        let Color(r, g, b, ..) = color;
        self.gfx.set_color(r, g, b);
        self.color = color;
    }

    pub fn set_position(
        &mut self,
        point: Point3<f32>,
    ) {
        self.scene_node_mut()
            .set_local_translation(Translation3 { vector: point.coords });
    }

    fn mesh() -> Rc<RefCell<Mesh>> {
        MeshManager::get_global_manager(|mm| {
            match mm.get(Self::MESH_NAME) {
                Some(mesh) => Some(mesh),
                None => {
                    Some(mm.add_trimesh(
                        crate::world::procedural::unit_shell(200, 200),
                        false,
                        Self::MESH_NAME,
                    ))
                }
            }
        })
        .unwrap_or_else(|| panic!("could not load mesh `{}`", Self::MESH_NAME))
    }
}
