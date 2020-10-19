use crate::{
    config::Config,
    deps::{
        kiss3d::{
            camera::{
                Camera,
                FirstPerson,
            },
            context::Context,
            nalgebra::{
                Isometry,
                Point3,
                Translation3,
                Vector3,
            },
            planar_camera::PlanarCamera,
            post_processing::PostProcessingEffect,
            renderer::Renderer,
            resource::{
                AllocationType,
                BufferType,
                Effect,
                GPUVec,
                ShaderAttribute,
                ShaderUniform,
            },
        },
        na::Matrix4,
    },
    geometry::spherical_to_cartesian,
};

pub type GraphicsNode = crate::deps::kiss3d::scene::SceneNode;

pub type PlanarGraphicsNode = crate::deps::kiss3d::scene::PlanarSceneNode;

pub type CameraEffects<'a> = (
    Option<&'a mut dyn Camera>,
    Option<&'a mut dyn PlanarCamera>,
    Option<&'a mut dyn Renderer>,
    Option<&'a mut dyn PostProcessingEffect>,
);

struct SkyPointGenerator {
    radius: f32,
}


impl SkyPointGenerator {
    #[inline(always)]
    fn next(&self) -> Point3<f32> {
        let r = self.radius;
        let theta = rand::random::<f32>() * (2.0 * std::f32::consts::PI);
        let phi = rand::random::<f32>() * (std::f32::consts::PI);
        spherical_to_cartesian(r, theta, phi)
    }
}


pub struct GraphicsManager {
    camera:     FirstPerson,
    #[cfg(not(target_arch = "wasm32"))]
    pointcloud: PointCloudRenderer,
    generator:  SkyPointGenerator,
}


impl GraphicsManager {
    pub fn with_config(config: &Config) -> Self {
        let height = 1.10 * ((config.block_size / 2.0f32) + (config.player_size.height() / 2.0f32));
        let eye = Point3::new(0.0f32, height, 0.0);
        let at = Point3::new(10.0f32, height, 10.0);
        let camera = FirstPerson::new_with_frustrum(config.field_of_view, config.znear, config.zfar, eye, at);

        // camera.unbind_movement_keys();
        // camera.set_move_step(0.0f32);
        // camera.move_dir(true, true, false, false);

        let generator = SkyPointGenerator {
            radius: (config.world_bounds.width() / 2.0) * 0.95,
        };

        Self {
            camera,
            #[cfg(not(target_arch = "wasm32"))]
            pointcloud: PointCloudRenderer::new(config.star_size),
            generator,
        }
    }

    pub fn camera(&self) -> &FirstPerson {
        &self.camera
    }

    pub fn camera_mut(&mut self) -> &mut FirstPerson {
        &mut self.camera
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn camera_and_effects_and_renderer(&mut self) -> CameraEffects<'_> {
        (Some(&mut self.camera), None, Some(&mut self.pointcloud), None)
    }

    #[cfg(target_arch = "wasm32")]
    pub fn camera_and_effects_and_renderer(&mut self) -> CameraEffects<'_> {
        (Some(&mut self.camera), None, None, None)
    }

    pub fn on_update(&mut self) {
        #[cfg(all(not(target_arch = "wasm32"), feature = "pointcloud"))]
        if self.pointcloud.num_points() < 50_000 {
            // Add some random points to the point cloud.
            for _ in 0..1_000 {
                let point = self.generator.next();
                self.pointcloud.push(point, rand::random());
            }
        }
    }
}


impl Default for GraphicsManager {
    fn default() -> Self {
        let eye = Point3::new(0.0f32, 0.4, 0.0);
        let at = Point3::new(10.0f32, 0.4, 10.0);
        let camera = FirstPerson::new(eye, at);


        Self {
            camera,
            #[cfg(not(target_arch = "wasm32"))]
            pointcloud: PointCloudRenderer::new(4.0),
            generator: SkyPointGenerator { radius: 100000.0f32 },
        }
    }
}


/// Structure which manages the display of long-living points.
#[cfg(not(target_arch = "wasm32"))]
struct PointCloudRenderer {
    shader:         Effect,
    pos:            ShaderAttribute<Point3<f32>>,
    color:          ShaderAttribute<Point3<f32>>,
    proj:           ShaderUniform<Matrix4<f32>>,
    view:           ShaderUniform<Matrix4<f32>>,
    colored_points: GPUVec<Point3<f32>>,
    point_size:     f32,
}

#[cfg(not(target_arch = "wasm32"))]
impl PointCloudRenderer {
    /// Creates a new points renderer.
    fn new(point_size: f32) -> PointCloudRenderer {
        let mut shader = Effect::new_from_str(VERTEX_SHADER_SRC, FRAGMENT_SHADER_SRC);

        shader.use_program();

        PointCloudRenderer {
            colored_points: GPUVec::new(Vec::new(), BufferType::Array, AllocationType::StreamDraw),
            pos: shader.get_attrib::<Point3<f32>>("position").unwrap(),
            color: shader.get_attrib::<Point3<f32>>("color").unwrap(),
            proj: shader.get_uniform::<Matrix4<f32>>("proj").unwrap(),
            view: shader.get_uniform::<Matrix4<f32>>("view").unwrap(),
            shader,
            point_size,
        }
    }

    fn push(
        &mut self,
        point: Point3<f32>,
        color: Point3<f32>,
    ) {
        if let Some(colored_points) = self.colored_points.data_mut() {
            colored_points.push(point);
            colored_points.push(color);
        }
    }

    fn num_points(&self) -> usize {
        self.colored_points.len() / 2
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Renderer for PointCloudRenderer {
    /// Actually draws the points.
    fn render(
        &mut self,
        pass: usize,
        camera: &mut dyn Camera,
    ) {
        if self.colored_points.len() == 0 {
            return;
        }

        self.shader.use_program();
        self.pos.enable();
        self.color.enable();

        camera.upload(pass, &mut self.proj, &mut self.view);

        self.color.bind_sub_buffer(&mut self.colored_points, 1, 1);
        self.pos.bind_sub_buffer(&mut self.colored_points, 1, 0);

        let ctxt = Context::get();
        ctxt.point_size(self.point_size);
        ctxt.draw_arrays(Context::POINTS, 0, (self.colored_points.len() / 2) as i32);

        self.pos.disable();
        self.color.disable();
    }
}

#[cfg(not(target_arch = "wasm32"))]
const VERTEX_SHADER_SRC: &'static str = "#version 100
    attribute vec3 position;
    attribute vec3 color;
    varying   vec3 Color;
    uniform   mat4 proj;
    uniform   mat4 view;
    void main() {
        gl_Position = proj * view * vec4(position, 1.0);
        Color = color;
    }";

#[cfg(not(target_arch = "wasm32"))]
const FRAGMENT_SHADER_SRC: &'static str = "#version 100
#ifdef GL_FRAGMENT_PRECISION_HIGH
   precision highp float;
#else
   precision mediump float;
#endif

    varying vec3 Color;
    void main() {
        gl_FragColor = vec4(Color, 1.0);
    }";
