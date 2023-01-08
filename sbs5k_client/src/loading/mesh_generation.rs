use std::rc::Rc;

use nalgebra::{Point3, Vector3};

use sbs5k_core::block::{Block, NON_EMPTY_BLOCKS_COUNT};
use sbs5k_core::chunk::{Chunk, ChunkCoordinate, CHUNK_DEPTH, CHUNK_HEIGHT, CHUNK_WIDTH};
use sbs5k_core::cube::CubeFace;
use sbs5k_engine::model::{Model, VertexData, VertexDataLayoutInfo};
use sbs5k_engine::texture::{ImageFileFormat, Texture, TextureCoordinate};
use sbs5k_engine::SceneObject;

use crate::resources;

const EPSILON: f32 = 0.01;

/// Generates renderable meshes from chunks.
///
/// It is recommended that one `MeshGenerator` be used for all mesh generation, rather than creating
/// a new object for each mesh to generate, because doing so will enable it to reuse its block
/// texture across all chunks.
pub struct MeshGenerator {
    blocks_texture: Rc<Texture>,
}

impl MeshGenerator {
    pub(crate) fn new() -> Self {
        MeshGenerator {
            blocks_texture: Rc::new(Texture::new(
                resources::cubes_texture(),
                ImageFileFormat::Png,
            )),
        }
    }

    /// Compute a renderable mesh from the blocks in a chunk.
    ///
    /// This function omits any faces that wouldn't be externally visible. If two blocks are adjacent,
    /// then it'll elide the two faces that are touching each other, since there's no way they could be
    /// seen.
    ///
    /// The structure generated by this function will need to be rebuild whenever a block is modified.
    ///
    /// TODO: Cull more aggressively (only emit the 3D convex hull) for chunks that the player's not currently in
    ///
    /// TODO: Also don't emit if there is still a block there in another chunk
    pub(crate) fn chunk_to_scene_object(
        &self,
        chunk: &Chunk,
        coordinate: ChunkCoordinate,
    ) -> SceneObject {
        let mut vertex_buffer: Vec<f32> = vec![];
        let mut index_buffer: Vec<u32> = vec![];

        for x in 0..CHUNK_WIDTH as i32 {
            for y in 0..CHUNK_HEIGHT as i32 {
                for z in 0..CHUNK_DEPTH as i32 {
                    if !chunk.has_block_at(x, y, z) {
                        continue;
                    }
                    let block = chunk.get_block_at(x as usize, y as usize, z as usize);

                    if !chunk.has_block_at(x + 1, y, z) {
                        emit_pos_x_face(
                            block,
                            x as f32,
                            y as f32,
                            z as f32,
                            &mut vertex_buffer,
                            &mut index_buffer,
                        );
                    }

                    if !chunk.has_block_at(x - 1, y, z) {
                        emit_neg_x_face(
                            block,
                            x as f32,
                            y as f32,
                            z as f32,
                            &mut vertex_buffer,
                            &mut index_buffer,
                        );
                    }

                    if !chunk.has_block_at(x, y + 1, z) {
                        emit_pos_y_face(
                            block,
                            x as f32,
                            y as f32,
                            z as f32,
                            &mut vertex_buffer,
                            &mut index_buffer,
                        );
                    }

                    if !chunk.has_block_at(x, y - 1, z) {
                        emit_neg_y_face(
                            block,
                            x as f32,
                            y as f32,
                            z as f32,
                            &mut vertex_buffer,
                            &mut index_buffer,
                        );
                    }

                    if !chunk.has_block_at(x, y, z + 1) {
                        emit_pos_z_face(
                            block,
                            x as f32,
                            y as f32,
                            z as f32,
                            &mut vertex_buffer,
                            &mut index_buffer,
                        );
                    }

                    if !chunk.has_block_at(x, y, z - 1) {
                        emit_neg_z_face(
                            block,
                            x as f32,
                            y as f32,
                            z as f32,
                            &mut vertex_buffer,
                            &mut index_buffer,
                        );
                    }
                }
            }
        }

        let model_layout_info = VertexDataLayoutInfo {
            position_offset: 0,
            normal_offset: Some(3),
            texture_offset: Some(6),
        };
        let vertices = VertexData::new(
            vertex_buffer.as_slice(),
            index_buffer.as_slice(),
            model_layout_info,
        );

        let model = Model {
            vertices,
            texture: self.blocks_texture.clone(),
        };

        let chunk_x = (coordinate.i * CHUNK_WIDTH as i32) as f32;
        let chunk_y = 0.0;
        let chunk_z = (coordinate.j * CHUNK_DEPTH as i32) as f32;
        let position = Point3::new(chunk_x, chunk_y, chunk_z);

        let orientation = Vector3::new(0.0, 0.0, 0.0);
        let scale = 1.0;

        SceneObject {
            position,
            orientation,
            scale,
            model,
        }
    }
}

#[inline]
fn emit_pos_x_face(
    block: Block,
    x: f32,
    y: f32,
    z: f32,
    vertex_buffer: &mut Vec<f32>,
    index_buffer: &mut Vec<u32>,
) {
    let normal = Vector3::new(1.0, 0.0, 0.0);
    let points = [
        Point3::new(x + 1.0, y + 1.0, z + 1.0),
        Point3::new(x + 1.0, y + 0.0, z + 1.0),
        Point3::new(x + 1.0, y + 0.0, z + 0.0),
        Point3::new(x + 1.0, y + 1.0, z + 0.0),
    ];

    emit_face(
        &points,
        normal,
        block,
        CubeFace::PosX,
        vertex_buffer,
        index_buffer,
    );
}

#[inline]
fn emit_neg_x_face(
    block: Block,
    x: f32,
    y: f32,
    z: f32,
    vertex_buffer: &mut Vec<f32>,
    index_buffer: &mut Vec<u32>,
) {
    let normal = Vector3::new(-1.0, 0.0, 0.0);
    let points = [
        Point3::new(x, y + 1.0, z),
        Point3::new(x, y, z),
        Point3::new(x, y, z + 1.0),
        Point3::new(x, y + 1.0, z + 1.0),
    ];

    emit_face(
        &points,
        normal,
        block,
        CubeFace::NegX,
        vertex_buffer,
        index_buffer,
    );
}

#[inline]
fn emit_pos_y_face(
    block: Block,
    x: f32,
    y: f32,
    z: f32,
    vertex_buffer: &mut Vec<f32>,
    index_buffer: &mut Vec<u32>,
) {
    let normal = Vector3::new(0.0, 1.0, 0.0);
    let points = [
        Point3::new(x, y + 1.0, z),
        Point3::new(x, y + 1.0, z + 1.0),
        Point3::new(x + 1.0, y + 1.0, z + 1.0),
        Point3::new(x + 1.0, y + 1.0, z),
    ];

    emit_face(
        &points,
        normal,
        block,
        CubeFace::PosY,
        vertex_buffer,
        index_buffer,
    );
}

#[inline]
fn emit_neg_y_face(
    block: Block,
    x: f32,
    y: f32,
    z: f32,
    vertex_buffer: &mut Vec<f32>,
    index_buffer: &mut Vec<u32>,
) {
    let normal = Vector3::new(0.0, -1.0, 0.0);
    let points = [
        Point3::new(x, y, z + 1.0),
        Point3::new(x, y, z),
        Point3::new(x + 1.0, y, z),
        Point3::new(x + 1.0, y, z + 1.0),
    ];

    emit_face(
        &points,
        normal,
        block,
        CubeFace::NegY,
        vertex_buffer,
        index_buffer,
    );
}

#[inline]
fn emit_pos_z_face(
    block: Block,
    x: f32,
    y: f32,
    z: f32,
    vertex_buffer: &mut Vec<f32>,
    index_buffer: &mut Vec<u32>,
) {
    let normal = Vector3::new(0.0, 0.0, 1.0);
    let points = [
        Point3::new(x + 0.0, y + 1.0, z + 1.0),
        Point3::new(x + 0.0, y + 0.0, z + 1.0),
        Point3::new(x + 1.0, y + 0.0, z + 1.0),
        Point3::new(x + 1.0, y + 1.0, z + 1.0),
    ];

    emit_face(
        &points,
        normal,
        block,
        CubeFace::NegZ,
        vertex_buffer,
        index_buffer,
    );
}

#[inline]
fn emit_neg_z_face(
    block: Block,
    x: f32,
    y: f32,
    z: f32,
    vertex_buffer: &mut Vec<f32>,
    index_buffer: &mut Vec<u32>,
) {
    let normal = Vector3::new(0.0, 0.0, -1.0);
    let points = [
        Point3::new(x + 1.0, y + 1.0, z),
        Point3::new(x + 1.0, y, z),
        Point3::new(x, y, z),
        Point3::new(x, y + 1.0, z),
    ];

    emit_face(
        &points,
        normal,
        block,
        CubeFace::PosZ,
        vertex_buffer,
        index_buffer,
    );
}

/// Create a face for a cube
fn emit_face(
    points: &[Point3<f32>; 4],
    normal: Vector3<f32>,
    block: Block,
    face: CubeFace,
    vertex_buffer: &mut Vec<f32>,
    index_buffer: &mut Vec<u32>,
) {
    let index = (vertex_buffer.len() as u32) / 8;
    let (tex_coords_start, tex_coords_end) = get_texture_coordinates(block, face);

    vertex_buffer.extend_from_slice(&[
        points[0].x,
        points[0].y,
        points[0].z,
        normal.x,
        normal.y,
        normal.z,
        tex_coords_start.u,
        tex_coords_start.v,
    ]);

    vertex_buffer.extend_from_slice(&[
        points[1].x,
        points[1].y,
        points[1].z,
        normal.x,
        normal.y,
        normal.z,
        tex_coords_start.u,
        tex_coords_end.v,
    ]);

    vertex_buffer.extend_from_slice(&[
        points[2].x,
        points[2].y,
        points[2].z,
        normal.x,
        normal.y,
        normal.z,
        tex_coords_end.u,
        tex_coords_end.v,
    ]);

    vertex_buffer.extend_from_slice(&[
        points[3].x,
        points[3].y,
        points[3].z,
        normal.x,
        normal.y,
        normal.z,
        tex_coords_end.u,
        tex_coords_start.v,
    ]);

    index_buffer.extend_from_slice(&[index, index + 1, index + 2, index + 2, index + 3, index]);
}

#[inline]
pub(crate) fn get_texture_coordinates(
    block: Block,
    cube_face: CubeFace,
) -> (TextureCoordinate, TextureCoordinate) {
    let u_index = match cube_face {
        CubeFace::PosX => 5,
        CubeFace::NegX => 4,
        CubeFace::PosY => 0,
        CubeFace::NegY => 1,
        CubeFace::PosZ => 3,
        CubeFace::NegZ => 2,
    };

    let v_index = match block {
        Block::Grass => 0,
        Block::Dirt => 1,
        Block::Stone => 2,

        _ => panic!("Don't have a texture mapping for block type: {:?}", block),
    };

    // We have to add an epsilon to avoid a rendering bug in which texture coordinates accidentally
    // round to an adjacent texture, causing a random colour border around blocks' edges.

    // TODO: Come up with a better fix

    let u_start = (u_index as f32) / 6.0;
    let v_start = (v_index as f32) / (NON_EMPTY_BLOCKS_COUNT as f32);
    let start_coords = TextureCoordinate {
        u: u_start + EPSILON,
        v: v_start + EPSILON,
    };

    let u_end = u_start + 1.0 / 6.0;
    let v_end = v_start + (1.0 / (NON_EMPTY_BLOCKS_COUNT as f32));
    let end_coords = TextureCoordinate {
        u: u_end - EPSILON,
        v: v_end - EPSILON,
    };

    (start_coords, end_coords)
}
