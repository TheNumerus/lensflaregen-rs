use std::ptr;

use gl::types::GLenum;

pub struct Geometry {
    mode: GeometryType,
    count: u32,
    vao: u32,
    vbo: u32,
}

impl Geometry {
    pub fn draw(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(self.mode.into(), 0, self.count as i32);
        }
    }
}

impl Drop for Geometry {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, ptr::addr_of!(self.vao));
            gl::DeleteBuffers(1, ptr::addr_of!(self.vbo));
        }
    }
}

pub struct GeometryBuilder {
    data: Vec<f32>,
    attributes: Vec<AttrSize>,
    mode: GeometryType,
}

impl GeometryBuilder {
    pub fn new(geometry_data: Vec<f32>) -> Self {
        Self {
            data: geometry_data,
            attributes: Vec::new(),
            mode: GeometryType::Triangles,
        }
    }

    pub fn mode(mut self, mode: GeometryType) -> Self {
        self.mode = mode;
        self
    }

    pub fn with_attributes(mut self, attributes: &[AttrSize]) -> Self {
        self.attributes.extend_from_slice(attributes);
        self
    }

    pub fn build(self) -> Geometry {
        let (mut vao, mut vbo) = (0, 0);
        unsafe {
            gl::GenVertexArrays(1, ptr::addr_of_mut!(vao));
            gl::GenBuffers(1, ptr::addr_of_mut!(vbo));

            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            gl::BufferData(
                gl::ARRAY_BUFFER,
                self.data.len() as isize * std::mem::size_of::<f32>() as isize,
                self.data.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            let total = self.attributes.iter().fold(0, |total, a| total + *a as i32);
            let mut offset = 0;

            for (i, attr) in self.attributes.iter().enumerate() {
                gl::VertexAttribPointer(
                    i as u32,
                    *attr as i32,
                    gl::FLOAT,
                    gl::FALSE,
                    total * std::mem::size_of::<f32>() as i32,
                    (offset * std::mem::size_of::<f32>() as i32) as *const _,
                );
                gl::EnableVertexAttribArray(i as u32);
                offset += *attr as i32;
            }

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);

            Geometry {
                vao,
                vbo,
                mode: self.mode,
                count: (self.data.len() as u32 / total as u32),
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum GeometryType {
    Triangles,
    TriangleFan,
    TriangleStrip,
}

impl From<GeometryType> for GLenum {
    fn from(gt: GeometryType) -> Self {
        match gt {
            GeometryType::Triangles => gl::TRIANGLES,
            GeometryType::TriangleFan => gl::TRIANGLE_FAN,
            GeometryType::TriangleStrip => gl::TRIANGLE_STRIP,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum AttrSize {
    Float = 1,
    Vec2 = 2,
    Vec3 = 3,
    Vec4 = 4,
}

static QUAD: [f32; 16] = [
    -1.0, -1.0, 0.0, 0.0, //
    -1.0, 1.0, 0.0, 1.0, //
    1.0, -1.0, 1.0, 0.0, //
    1.0, 1.0, 1.0, 1.0, //
];

pub fn quad() -> Geometry {
    GeometryBuilder::new(QUAD.to_vec())
        .mode(GeometryType::TriangleStrip)
        .with_attributes(&[AttrSize::Vec2, AttrSize::Vec2])
        .build()
}
