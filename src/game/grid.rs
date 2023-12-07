use bevy::{
  prelude::*,
  render::{
    mesh::{Indices, MeshVertexBufferLayout},
    render_resource::{
      AsBindGroup, PrimitiveTopology, RenderPipelineDescriptor, ShaderRef,
      SpecializedMeshPipelineError,
    },
  },
  sprite::{Material2d, Material2dKey, MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_smud::prelude::Frame;
use jam4::level::{LevelManager, LevelRegistry};

pub fn build_grid(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<GridMaterial>>,
  lvl: Res<LevelManager>,
  reg: Res<LevelRegistry>,
) {
  let cur_level_id = lvl.current_level.expect("should have an active level");
  let cur_level = reg
    .levels
    .get(&cur_level_id)
    .expect("active level should be in registry");

  let Frame::Quad(sz) = cur_level
    .bounds_sdf
    .clone()
    .expect("should have bounds for a grid")
    .frame;
  // sz = 400.;
  let mut grid = Mesh::new(PrimitiveTopology::TriangleList);

  let v_pos = vec![
    [-sz, -sz, 0.0],
    [-sz, sz, 0.0],
    [sz, sz, 0.0],
    [sz, -sz, 0.0],
  ];
  grid.insert_attribute(Mesh::ATTRIBUTE_POSITION, v_pos);

  let vertex_colors: Vec<[f32; 2]> = vec![[0.0, 0.0], [0.0, 1.0], [1.0, 1.0], [1.0, 0.0]];

  grid.insert_attribute(Mesh::ATTRIBUTE_UV_0, vertex_colors);

  let indices = vec![0, 2, 1, 2, 0, 3];
  grid.set_indices(Some(Indices::U32(indices)));

  commands.spawn(MaterialMesh2dBundle {
    mesh: Mesh2dHandle(meshes.add(grid)),
    material: materials.add(GridMaterial {}),
    transform: Transform::from_translation(Vec3::new(0.0,0.0,-100.)),
    ..default()
  });
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct GridMaterial {}

impl Material2d for GridMaterial {
  fn fragment_shader() -> ShaderRef {
    "preload/grid.wgsl".into()
  }
  fn vertex_shader() -> ShaderRef {
    "preload/grid.wgsl".into()
  }

  fn specialize(
    descriptor: &mut RenderPipelineDescriptor,
    layout: &MeshVertexBufferLayout,
    _key: Material2dKey<Self>,
  ) -> Result<(), SpecializedMeshPipelineError> {
    let vertex_layout = layout.get_layout(&[
      Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
      Mesh::ATTRIBUTE_UV_0.at_shader_location(1),
    ])?;
    descriptor.vertex.buffers = vec![vertex_layout];
    Ok(())
  }
}
