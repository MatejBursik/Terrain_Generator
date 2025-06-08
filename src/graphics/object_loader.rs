use gltf;

pub struct ObjectLoader {
    vertices: Vec<f32>,
    indices: Vec<u32>
}

impl ObjectLoader {
    pub fn new(object_path: &str) -> ObjectLoader {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        // Load the GLTF file
        let (gltf, buffers, _images) = gltf::import(object_path)
            .expect("Failed to load GLTF file");

        // Process all meshes in the GLTF file
        for mesh in gltf.meshes() {
            for primitive in mesh.primitives() {
                // Get vertex positions
                if let Some(position_accessor) = primitive.get(&gltf::Semantic::Positions) {
                    let view = position_accessor.view().unwrap();
                    let buffer = &buffers[view.buffer().index()];
                    let start = view.offset() + position_accessor.offset();
                    
                    match position_accessor.data_type() {
                        gltf::accessor::DataType::F32 => {
                            let stride = position_accessor.size();
                            let count = position_accessor.count();
                            
                            for i in 0..count {
                                let offset = start + i * stride;
                                let x = f32::from_le_bytes([
                                    buffer[offset],
                                    buffer[offset + 1],
                                    buffer[offset + 2],
                                    buffer[offset + 3],
                                ]);
                                let y = f32::from_le_bytes([
                                    buffer[offset + 4],
                                    buffer[offset + 5],
                                    buffer[offset + 6],
                                    buffer[offset + 7],
                                ]);
                                let z = f32::from_le_bytes([
                                    buffer[offset + 8],
                                    buffer[offset + 9],
                                    buffer[offset + 10],
                                    buffer[offset + 11],
                                ]);
                                vertices.extend_from_slice(&[x, y, z]);
                            }
                        }
                        _ => panic!("Unsupported position data type"),
                    }
                }

                // Get indices
                if let Some(indices_accessor) = primitive.indices() {
                    let view = indices_accessor.view().unwrap();
                    let buffer = &buffers[view.buffer().index()];
                    let start = view.offset() + indices_accessor.offset();
                    
                    match indices_accessor.data_type() {
                        gltf::accessor::DataType::U16 => {
                            let count = indices_accessor.count();
                            for i in 0..count {
                                let offset = start + i * 2;
                                let index = u16::from_le_bytes([
                                    buffer[offset],
                                    buffer[offset + 1],
                                ]) as u32;
                                indices.push(index);
                            }
                        }
                        gltf::accessor::DataType::U32 => {
                            let count = indices_accessor.count();
                            for i in 0..count {
                                let offset = start + i * 4;
                                let index = u32::from_le_bytes([
                                    buffer[offset],
                                    buffer[offset + 1],
                                    buffer[offset + 2],
                                    buffer[offset + 3],
                                ]);
                                indices.push(index);
                            }
                        }
                        _ => panic!("Unsupported index data type"),
                    }
                }
            }
        }

        ObjectLoader {
            vertices,
            indices
        }
    }

    pub fn get_vertices(&self) -> &Vec<f32> {
        &self.vertices
    }

    pub fn get_indices(&self) -> Vec<i32> {
        self.indices.iter().map(|&i| i as i32).collect()
    }

    pub fn get_triangle_count(&self) -> i32 {
        self.indices.len() as i32
    }
}
