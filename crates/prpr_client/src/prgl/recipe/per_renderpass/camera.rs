use super::*;

crate::shader_attr! {
  struct CameraAttribute {
    view_mat: mat4,
    proj_mat: mat4,
    view_proj_mat: mat4,
    camera_pos: vec3,
    camera_dummy: float,
    camera_target_pos: vec3,
    camera_dummy2: float,
    fovy: float,
    aspect_ratio: float,
    near: float,
    far: float,
  }
}

pub struct Camera {
  pub ubo: Arc<UniformBuffer<CameraAttribute>>,
  pub camera_pos: Vec3,
  pub camera_target_pos: Vec3,
  pub fovy: f32,
  pub aspect_ratio: f32,
  pub near: f32,
  pub far: f32,
}

impl Camera {
  pub fn new(ctx: &ArcGlContext) -> Self {
    let near = 0.01;
    let far = 1000.0;
    let aspect_ratio = 1.0;
    let fovy = 3.141592 * 0.25;
    let camera_pos = Vec3::ONE;
    let camera_target_pos = Vec3::ZERO;
    let view_mat = Self::to_view_mat(camera_pos, camera_target_pos);
    let proj_mat = Self::to_proj_mat(fovy, aspect_ratio, near, far);
    let attr = UniformBuffer::new(
      ctx,
      CameraAttribute {
        view_mat,
        proj_mat,
        view_proj_mat: proj_mat * view_mat,
        camera_pos,
        camera_dummy: 0.0,
        camera_target_pos,
        camera_dummy2: 0.0,
        fovy,
        aspect_ratio,
        near,
        far,
      },
    );
    Self {
      ubo: Arc::new(attr),
      camera_pos,
      camera_target_pos,
      fovy,
      aspect_ratio,
      near,
      far,
    }
  }

  pub fn update(&self) {
    let view_mat = Self::to_view_mat(self.camera_pos, self.camera_target_pos);
    let proj_mat = Self::to_proj_mat(self.fovy, self.aspect_ratio, self.near, self.far);
    let view_proj_mat = proj_mat * view_mat;
    let mut ubo = self.ubo.write_lock();
    ubo.view_mat = view_mat;
    ubo.proj_mat = proj_mat;
    ubo.view_proj_mat = view_proj_mat;
    ubo.camera_pos = self.camera_pos;
    ubo.camera_target_pos = self.camera_target_pos;
    ubo.fovy = self.fovy;
    ubo.aspect_ratio = self.aspect_ratio;
    ubo.near = self.near;
    ubo.far = self.far;
  }

  fn to_view_mat(camera_pos: Vec3, target_pos: Vec3) -> Mat4 {
    Mat4::look_at_rh(camera_pos, target_pos, Vec3::Y)
  }
  fn to_proj_mat(fovy: f32, aspect_ratio: f32, near: f32, far: f32) -> Mat4 {
    Mat4::perspective_rh(fovy, aspect_ratio, near, far)
  }
}

impl RenderPassBindable for Camera {
  fn bind(&self, renderpass: &mut RenderPass) {
    renderpass.add_uniform_buffer(&self.ubo);
  }
}
