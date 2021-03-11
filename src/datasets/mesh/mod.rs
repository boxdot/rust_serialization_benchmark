pub mod mesh_capnp;
pub mod mesh_generated;
mod mesh_generated_flatdata;

pub mod mesh_prost {
    include!(concat!(env!("OUT_DIR"), "/prost.mesh.rs"));
}

use crate::{bench_capnp, bench_flatbuffers, bench_flatdata, bench_prost, Generate};
use core::pin::Pin;
use flatbuffers::{FlatBufferBuilder, WIPOffset};
pub use mesh_capnp as cp;
pub use mesh_generated::mesh as fb;
pub use mesh_generated_flatdata::mesh as fd;
use rand::Rng;
use rkyv::Archived;
use std::sync::Arc;

#[derive(
    Clone,
    Copy,
    abomonation_derive::Abomonation,
    rkyv::Archive,
    rkyv::Serialize,
    rkyv::Deserialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[archive(copy)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Generate for Vector3 {
    fn generate<R: Rng>(rand: &mut R) -> Self {
        Self {
            x: rand.gen(),
            y: rand.gen(),
            z: rand.gen(),
        }
    }
}

impl Into<fb::Vector3> for Vector3 {
    fn into(self) -> fb::Vector3 {
        fb::Vector3::new(self.x, self.y, self.z)
    }
}

impl Vector3 {
    fn serialize_flatdata(&self, v: &mut fd::Vector3) {
        let encode_f32 = |value: f32| u32::from_le_bytes(value.to_le_bytes());
        v.set_x(encode_f32(self.x));
        v.set_y(encode_f32(self.y));
        v.set_z(encode_f32(self.z));
    }

    pub fn from_flatdata(v: &fd::Vector3) -> Self {
        let decode_f32 = |value: u32| f32::from_le_bytes(value.to_le_bytes());
        Self {
            x: decode_f32(v.x()),
            y: decode_f32(v.y()),
            z: decode_f32(v.z()),
        }
    }
}

impl<'a> bench_capnp::Serialize<'a> for Vector3 {
    type Reader = cp::vector3::Reader<'a>;
    type Builder = cp::vector3::Builder<'a>;

    fn serialize_capnp(&self, builder: &mut Self::Builder) {
        builder.set_x(self.x);
        builder.set_y(self.y);
        builder.set_z(self.z);
    }
}

impl bench_prost::Serialize for Vector3 {
    type Message = mesh_prost::Vector3;

    fn serialize_pb(&self) -> Self::Message {
        let mut result = Self::Message::default();
        result.x = self.x;
        result.y = self.y;
        result.z = self.z;
        result
    }
}

#[derive(
    Clone,
    Copy,
    abomonation_derive::Abomonation,
    rkyv::Archive,
    rkyv::Serialize,
    rkyv::Deserialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[archive(copy)]
pub struct Triangle {
    pub v0: Vector3,
    pub v1: Vector3,
    pub v2: Vector3,
    pub normal: Vector3,
}

impl Generate for Triangle {
    fn generate<R: Rng>(rand: &mut R) -> Self {
        Self {
            v0: Vector3::generate(rand),
            v1: Vector3::generate(rand),
            v2: Vector3::generate(rand),
            normal: Vector3::generate(rand),
        }
    }
}

impl Into<fb::Triangle> for Triangle {
    fn into(self) -> fb::Triangle {
        fb::Triangle::new(
            &self.v0.into(),
            &self.v1.into(),
            &self.v2.into(),
            &self.normal.into(),
        )
    }
}

impl<'a> bench_capnp::Serialize<'a> for Triangle {
    type Reader = cp::triangle::Reader<'a>;
    type Builder = cp::triangle::Builder<'a>;

    fn serialize_capnp(&self, builder: &mut Self::Builder) {
        self.v0.serialize_capnp(&mut builder.reborrow().init_v0());
        self.v1.serialize_capnp(&mut builder.reborrow().init_v1());
        self.v2.serialize_capnp(&mut builder.reborrow().init_v2());
        self.normal
            .serialize_capnp(&mut builder.reborrow().init_normal());
    }
}

impl bench_prost::Serialize for Triangle {
    type Message = mesh_prost::Triangle;

    fn serialize_pb(&self) -> Self::Message {
        let mut result = Self::Message::default();
        result.v0 = Some(self.v0.serialize_pb());
        result.v1 = Some(self.v1.serialize_pb());
        result.v2 = Some(self.v2.serialize_pb());
        result.normal = Some(self.normal.serialize_pb());
        result
    }
}

#[derive(
    abomonation_derive::Abomonation,
    rkyv::Archive,
    rkyv::Serialize,
    rkyv::Deserialize,
    serde::Serialize,
    serde::Deserialize,
)]
pub struct Mesh {
    pub triangles: Vec<Triangle>,
}

impl ArchivedMesh {
    pub fn triangles_pin(self: Pin<&mut Self>) -> Pin<&mut Archived<Vec<Triangle>>> {
        unsafe { self.map_unchecked_mut(|s| &mut s.triangles) }
    }
}

impl<'a> bench_flatbuffers::Serialize<'a> for Mesh {
    type Target = fb::Mesh<'a>;

    fn serialize_fb<'b>(&self, fbb: &'b mut FlatBufferBuilder<'a>) -> WIPOffset<Self::Target>
    where
        'a: 'b,
    {
        fbb.start_vector::<fb::Triangle>(self.triangles.len());
        for triangle in self.triangles.iter().cloned() {
            fbb.push::<fb::Triangle>(triangle.into());
        }
        let triangles = fbb.end_vector(self.triangles.len());

        let mut builder = fb::MeshBuilder::new(fbb);
        builder.add_triangles(triangles);
        builder.finish()
    }
}

impl<'a> bench_capnp::Serialize<'a> for Mesh {
    type Reader = cp::mesh::Reader<'a>;
    type Builder = cp::mesh::Builder<'a>;

    fn serialize_capnp(&self, builder: &mut Self::Builder) {
        let mut mesh = builder
            .reborrow()
            .init_triangles(self.triangles.len() as u32);
        for (i, value) in self.triangles.iter().enumerate() {
            value.serialize_capnp(&mut mesh.reborrow().get(i as u32));
        }
    }
}

impl bench_prost::Serialize for Mesh {
    type Message = mesh_prost::Mesh;

    fn serialize_pb(&self) -> Self::Message {
        let mut result = Self::Message::default();
        for triangle in self.triangles.iter() {
            result.triangles.push(triangle.serialize_pb());
        }
        result
    }
}

impl bench_flatdata::Serialize for Mesh {
    type Archive = fd::Mesh;

    fn serialize_fd(&self) -> std::io::Result<Arc<flatdata::MemoryResourceStorage>> {
        let storage = flatdata::MemoryResourceStorage::new("/mesh");
        let builder = fd::MeshBuilder::new(storage.clone()).expect("failed to create builder");

        let mut triangles = builder.start_triangles()?;
        for triangle in self.triangles.iter() {
            triangle.v0.serialize_flatdata(triangles.grow()?);
            triangle.v1.serialize_flatdata(triangles.grow()?);
            triangle.v2.serialize_flatdata(triangles.grow()?);
            triangle.normal.serialize_flatdata(triangles.grow()?);
        }
        triangles.close().expect("close failed"); // flush vector

        Ok(storage)
    }
}
