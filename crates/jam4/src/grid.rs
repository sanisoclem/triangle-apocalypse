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

use crate::Simulation;

pub fn build_grid(
  commands: &mut Commands,
  meshes: &mut Assets<Mesh>,
  materials: &mut Assets<GridMaterial>,
  frame_size: Vec2,
) {
  let mut grid = Mesh::new(PrimitiveTopology::TriangleList);

  let v_pos = vec![
    [-frame_size.x, -frame_size.y, 0.0],
    [-frame_size.x, frame_size.y, 0.0],
    [frame_size.x, frame_size.y, 0.0],
    [frame_size.x, -frame_size.y, 0.0],
  ];
  grid.insert_attribute(Mesh::ATTRIBUTE_POSITION, v_pos);

  let uvs: Vec<[f32; 2]> = vec![[0.0, 0.0], [0.0, 1.0], [1.0, 1.0], [1.0, 0.0]];

  grid.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

  let indices = vec![0, 2, 1, 2, 0, 3];
  grid.set_indices(Some(Indices::U32(indices)));

  commands
    .spawn(MaterialMesh2dBundle {
      mesh: Mesh2dHandle(meshes.add(grid)),
      material: materials.add(GridMaterial {}),
      transform: Transform::from_translation(Vec3::new(0.0, 0.0, -100.)),
      ..default()
    })
    .insert(Simulation);
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
