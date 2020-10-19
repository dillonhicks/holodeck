#[derive(Copy, Clone, Debug, PartialEq)]
pub enum RunMode {
    Initializing,
    Running,
    Paused,
}



#[derive(Copy, Clone, Debug)]
pub struct Client {
    pub run_mode: RunMode,
    /* pub draw_colls: bool,
     * pub highlighted_body: Option<RigidBodyHandle>,
     *    pub grabbed_object: Option<DefaultBodyPartHandle>,
     *    pub grabbed_object_constraint: Option<DefaultJointConstraintHandle>,
     * pub grabbed_object_plane: (Point3<f32>, Vector3<f32>),
     * pub can_grab_behind_ground: bool,
     * pub drawing_ray: Option<Point2<f32>>,
     * pub prev_flags: TestbedStateFlags,
     * pub flags: TestbedStateFlags,
     * pub action_flags: TestbedActionFlags,
     * pub backend_names: Vec<&'static str>,
     * pub example_names: Vec<&'static str>,
     * pub selected_example: usize,
     * pub selected_backend: usize,
     * pub physx_use_two_friction_directions: bool,
     * pub num_threads: usize,
     * pub snapshot: Option<PhysicsSnapshot>,
     * #[cfg(feature = "parallel")]
     * pub thread_pool: rapier::rayon::ThreadPool,
     * pub timestep_id: usize, */
}


impl Default for Client {
    fn default() -> Self {
        Self {
            run_mode: RunMode::Running,
        }
    }
}
