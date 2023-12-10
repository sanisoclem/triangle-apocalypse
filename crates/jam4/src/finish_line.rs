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

pub fn build_finish_line(
  commands: &mut Commands,
  meshes: &mut Assets<Mesh>,
  materials: &mut Assets<FinishLineMaterial>,
  frame_specs: Vec4,
) {
  let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

  let v_pos = vec![
    [-frame_specs.z, -frame_specs.w, 0.0],
    [-frame_specs.z, frame_specs.w, 0.0],
    [frame_specs.z, frame_specs.w, 0.0],
    [frame_specs.z, -frame_specs.w, 0.0],
  ];
  mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, v_pos);

  let uvs: Vec<[f32; 2]> = vec![[0.0, 0.0], [0.0, 1.0], [1.0, 1.0], [1.0, 0.0]];

  mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

  let indices = vec![0, 2, 1, 2, 0, 3];
  mesh.set_indices(Some(Indices::U32(indices)));

  commands
    .spawn(MaterialMesh2dBundle {
      mesh: Mesh2dHandle(meshes.add(mesh)),
      material: materials.add(FinishLineMaterial {}),
      transform: Transform::from_translation(frame_specs.xy().extend(-100.0)),
      ..default()
    })
    .insert(Simulation);
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct FinishLineMaterial {}

impl Material2d for FinishLineMaterial {
  fn fragment_shader() -> ShaderRef {
    "preload/finish_line.wgsl".into()
  }
  fn vertex_shader() -> ShaderRef {
    "preload/finish_line.wgsl".into()
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
