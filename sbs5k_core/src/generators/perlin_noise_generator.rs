use std::collections::BTreeMap;
use std::f32::consts::PI;

use crate::block::Block;
use crate::chunk::{Chunk, ChunkCoordinate, ChunkSource, CHUNK_DEPTH, CHUNK_HEIGHT, CHUNK_WIDTH};
use crate::maths::{interpolate, modulo_fp};

use glm::{cos, floor, sin};
use nalgebra::Vector2;
use rand;
use rand::Rng;

type Index = (i32, i32);

struct NormalisedPerlinNoiseSource {
    generated_vectors: BTreeMap<Index, Vector2<f32>>,
}

impl NormalisedPerlinNoiseSource {
    pub fn new() -> Self {
        NormalisedPerlinNoiseSource {
            generated_vectors: Default::default(),
        }
    }

    pub fn sample(&mut self, x: f32, y: f32) -> f32 {
        let sx = modulo_fp(x, 1.0);
        let sy = modulo_fp(y, 1.0);

        let x0 = floor(x) as i32;
        let x1 = x0 + 1;
        let y0 = floor(y) as i32;
        let y1 = y0 + 1;

        let n0 = self.dot_grid_gradient((x0, y0), x, y);
        let n1 = self.dot_grid_gradient((x1, y0), x, y);
        let ix0 = interpolate(n0, n1, sx);

        let n0 = self.dot_grid_gradient((x0, y1), x, y);
        let n1 = self.dot_grid_gradient((x1, y1), x, y);
        let ix1 = interpolate(n0, n1, sx);

        interpolate(ix0, ix1, sy)
    }

    fn get_vector_at(&mut self, index: Index) -> Vector2<f32> {
        if let Some(v) = self.generated_vectors.get(&index) {
            return *v;
        }
        let vector = generate_random_vector();
        self.generated_vectors.insert(index, vector);
        vector
    }

    fn dot_grid_gradient(&mut self, index: Index, x: f32, y: f32) -> f32 {
        let gradient_vector = self.get_vector_at(index);

        let (x_rounded, y_rounded) = index;
        let dx = x - (x_rounded as f32);
        let dy = y - (y_rounded as f32);
        let displacement_vector = Vector2::new(dx, dy);

        gradient_vector.dot(&displacement_vector)
    }
}

struct PerlinNoiseComponent {
    period: u32,
    amplitude: f32,
    noise_source: NormalisedPerlinNoiseSource,
}

impl PerlinNoiseComponent {
    pub fn new(period: u32, amplitude: f32) -> Self {
        PerlinNoiseComponent {
            period,
            amplitude,
            noise_source: NormalisedPerlinNoiseSource::new(),
        }
    }

    pub fn sample(&mut self, index: Index) -> f32 {
        let (x, z) = index;
        let x_normalised = x as f32 / self.period as f32;
        let z_normalised = z as f32 / self.period as f32;
        let noise = self.noise_source.sample(x_normalised, z_normalised);
        noise * self.amplitude
    }
}

pub struct PerlinNoiseGenerator {
    components: Vec<PerlinNoiseComponent>,
}

impl PerlinNoiseGenerator {
    pub fn new() -> Self {
        PerlinNoiseGenerator {
            components: vec![
                PerlinNoiseComponent::new(128, 30.0),
                PerlinNoiseComponent::new(64, 30.0),
                PerlinNoiseComponent::new(32, 15.0),
                PerlinNoiseComponent::new(16, 2.0),
            ],
        }
    }

    pub fn get_offset_at(&mut self, global_x: i32, global_z: i32) -> i32 {
        self.components
            .iter_mut()
            .map(|component| component.sample((global_x, global_z)))
            .sum::<f32>() as i32
    }
}

impl ChunkSource for PerlinNoiseGenerator {
    fn get_chunk_at(&mut self, coordinate: ChunkCoordinate) -> Box<Chunk> {
        let mut chunk: Box<Chunk> = Default::default();

        #[allow(clippy::needless_range_loop)]
        for x in 0..CHUNK_WIDTH {
            for y in 0..CHUNK_HEIGHT {
                for z in 0..CHUNK_DEPTH {
                    let global_x = coordinate.i * (CHUNK_WIDTH as i32) + (x as i32);
                    let global_z = coordinate.j * (CHUNK_DEPTH as i32) + (z as i32);

                    let offset = self.get_offset_at(global_x, global_z);

                    let empty_start = glm::max(65 + offset, 0);
                    let grass_start = glm::max(empty_start - 1, 0);
                    let dirt_start = glm::max(grass_start - 3, 0);

                    let block_type = if y >= empty_start as usize {
                        Block::Empty
                    } else if y >= grass_start as usize {
                        Block::Grass
                    } else if y >= dirt_start as usize {
                        Block::Dirt
                    } else {
                        Block::Stone
                    };
                    chunk.set_block_at(x, y, z, block_type);
                }
            }
        }

        chunk
    }
}

impl Default for PerlinNoiseGenerator {
    fn default() -> Self {
        PerlinNoiseGenerator::new()
    }
}

fn generate_random_vector() -> Vector2<f32> {
    let mut rng = rand::thread_rng();
    let theta = rng.gen::<f32>() * PI;
    Vector2::new(cos(theta), sin(theta))
}
