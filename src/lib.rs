#![feature(core_intrinsics, exact_chunks)]
//#![allow(unused)]

mod controls;
mod error;
mod geometry;
mod js;
mod mains;
mod map;
mod physics;
mod random;
mod render;
mod webgl;

mod mesh;
mod raytrace;
use wasm_bindgen::prelude::*;

pub use js::*;
pub use mains::*;
pub use random::*;
pub use webgl::*;

// mesh module
pub use mesh::*;
pub use raytrace::*;


extern crate bincode;
extern crate byteorder;
extern crate jpeg_decoder as jpeg;
#[macro_use]
extern crate lazy_static;
extern crate nalgebra as na;
extern crate ncollide3d as nc;
extern crate nphysics3d as np;
extern crate pcg_rand;
extern crate png;
extern crate rand;
extern crate wasm_bindgen;
extern crate webgl_test_common;

// mesh
extern crate rscsg;
extern crate smallpt;
extern crate plexus;
extern crate decorum;
// extern crate delaunator;
extern crate gltf;

use decorum::R32;
use na::{Point2, Point3};
use plexus::prelude::*;
use plexus::primitive::cube::{Cube, Plane};
use plexus::primitive::{self, HashIndexer};

use plexus::graph::Mesh;

use plexus::primitive::sphere::{Bounds, UvSphere};

fn map_unit_uv(position: Point3<R32>, plane: Plane, unit: R32) -> Point2<R32> {
    let map = |position: R32| -> R32 { position / unit };
    match plane {
        Plane::XY => Point2::new(map(position.x), map(position.y)),
        Plane::NXY => Point2::new(-map(position.x), map(position.y)),
        Plane::ZY => Point2::new(map(position.z), map(position.y)),
        Plane::NZY => Point2::new(-map(position.z), map(position.y)),
        Plane::XZ => Point2::new(map(position.x), map(position.z)),
        Plane::XNZ => Point2::new(map(position.x), -map(position.z)),
    }
}

fn test_plexus() {
    let cube = Cube::default();
    // Zip positions and planes into the vertices of a stream of polygons.
    let polygons = primitive::zip_vertices((
        cube.polygons_with_position()
            .map_vertices(|position| -> Point3<R32> { position.into() })
            .map_vertices(|position| position * 8.0.into()),
        cube.polygons_with_plane(),
    ));
    // Use the position and plane to map texture coordinates and then
    // triangulate the polygons and index them.
    let (_, _) = polygons
        .map_vertices(|(position, plane)| {
            (position, plane, map_unit_uv(position, plane, 8.0.into()))
        }).triangulate()
        .index_vertices(HashIndexer::default());
}


fn test_sphere() {
	// Construct a mesh from a sphere primitive. The vertex geometry is convertible
	// to `Point3` via the `FromGeometry` trait in this example.
	let mut mesh = UvSphere::new(8, 8)
	    .polygons_with_position_from(Bounds::unit_width())
	    .collect::<Mesh<Point3<f32>>>();
	// Extrude a face in the mesh.
	let key = mesh.faces().nth(0).unwrap().key();
	if let Ok(face) = mesh.face_mut(key).unwrap().extrude(1.0) {
	    // ...
	    let msg = format!("{}", face.neighboring_faces().count());
	    js::log(&msg);
	}
}


type EdgeIndex = mesh::Index<usize>;

#[wasm_bindgen]
pub fn test_mesh()->Vec<u8> {

	let x = EdgeIndex::new(0usize);
	js::log("test mesh");

	test_plexus();
	test_sphere();

	let bitmap= raytrace::render();

	let y = x;

	bitmap
}




// //

// use std::iter::repeat_with;

// const N: usize = 1_000_000;

// #[wasm_bindgen]
// pub fn test_delaunator() {
//     js::log("begin delaunator 1 ");
//     let points: Vec<_> = repeat_with(rand::random)
//         .map(|(x, y)| delaunator::Point { x, y })
//         .take(N)
//         .collect();

//     js::log("begin delaunator 2");
//     let now = std::time::Instant::now();

//     js::log("begin delaunator 3");
//     let result = delaunator::triangulate(&points).expect("No triangulation exists for this input.");
//     js::log("begin delaunator 4");
//     let elapsed = now.elapsed();

//     js::log("begin delaunator 5");

//    let v = format!(
//         "Triangulated {} points in {}.{}s.\nGenerated {} triangles. Convex hull size: {}",
//         N,
//         elapsed.as_secs(),
//         elapsed.subsec_millis(),
//         result.len(),
//         result.hull.len()
//     );

//    js::log(&v);
// }



//
// notice: gltf used about 800KB in resultant wasm file.
//
use std::{  io};
use std::boxed::Box;
use std::error::Error as StdError;
use std::io::Cursor;
use gltf::Gltf;

pub const BOX_GLTF: &'static [u8] = include_bytes!("../doc/box.gltf");
pub const BROTELI_TXT: &'static [u8] = include_bytes!("../doc/asyoulik.txt");
pub const BROTELI_TXT_COMPRESSED: &'static [u8] = include_bytes!("../doc/asyoulik.txt.compressed");

fn print_tree(node: &gltf::Node, depth: i32) {
    for _ in 0..(depth - 1) {
        js::log("  ");
    }
    js::log(" -");
    js::log(&format!(" Node {}", node.index()));

    js::log(&format!(" ({})", node.name().unwrap_or("<Unnamed>")));


    for child in node.children() {
        print_tree(&child, depth + 1);
    }
}

#[wasm_bindgen]
pub fn run_gltf( )   {


    let gltf = gltf::Gltf::from_reader(Cursor::new(BOX_GLTF)).unwrap();

    let now = std::time::Instant::now();

    for scene in gltf.scenes() {
        js::log(&format!("Scene {}", scene.index()));
        #[cfg(feature = "names")]
       js::log(&format!(" ({:?})", scene.name().unwrap_or("<Unnamed>")));

        for node in scene.nodes() {
            print_tree(&node, 1);
        }
    }

    let elapsed = now.elapsed();
     js::log(&format!(" elapsed: {:?}", elapsed));

}


// About 1MB
extern crate brotli;

use std::io::Write;





// pub struct Buffer {
//   data: Vec<u8>,
//   read_offset: usize,
// }

// pub struct UnlimitedBuffer {
//   data: Vec<u8>,
//   read_offset: usize,
// }


// impl Buffer {
//   pub fn new(buf: &[u8]) -> Buffer {
//     let mut ret = Buffer {
//       data: Vec::<u8>::new(),
//       read_offset: 0,
//     };
//     ret.data.extend(buf);
//     return ret;
//   }
// }
// impl UnlimitedBuffer {
//   pub fn new(buf: &[u8]) -> Self {
//     let mut ret = UnlimitedBuffer {
//       data: Vec::<u8>::new(),
//       read_offset: 0,
//     };
//     ret.data.extend(buf);
//     return ret;
//   }
//   pub fn reset_read(&mut self) {
//       self.read_offset = 0;
//   }
// }
// impl io::Read for Buffer {
//   fn read(self: &mut Self, buf: &mut [u8]) -> io::Result<usize> {
//     if self.read_offset == self.data.len() {
//       self.read_offset = 0;
//     }
//     let bytes_to_read = cmp::min(buf.len(), self.data.len() - self.read_offset);
//     if bytes_to_read > 0 {
//       buf[0..bytes_to_read].clone_from_slice(&self.data[self.read_offset..
//                                               self.read_offset + bytes_to_read]);
//     }
//     self.read_offset += bytes_to_read;
//     return Ok(bytes_to_read);
//   }
// }
// impl io::Write for Buffer {
//   fn write(self: &mut Self, buf: &[u8]) -> io::Result<usize> {
//     if self.read_offset == self.data.len() {
//       return Ok(buf.len());
//     }
//     self.data.extend(buf);
//     return Ok(buf.len());
//   }
//   fn flush(self: &mut Self) -> io::Result<()> {
//     return Ok(());
//   }
// }
// impl io::Read for UnlimitedBuffer {
//   fn read(self: &mut Self, buf: &mut [u8]) -> io::Result<usize> {
//     let bytes_to_read = cmp::min(buf.len(), self.data.len() - self.read_offset);
//     if bytes_to_read > 0 {
//       buf[0..bytes_to_read].clone_from_slice(&self.data[self.read_offset..
//                                               self.read_offset + bytes_to_read]);
//     }
//     self.read_offset += bytes_to_read;
//     return Ok(bytes_to_read);
//   }
// }

// impl io::Write for UnlimitedBuffer {
//   fn write(self: &mut Self, buf: &[u8]) -> io::Result<usize> {
//     self.data.extend(buf);
//     return Ok(buf.len());
//   }
//   fn flush(self: &mut Self) -> io::Result<()> {
//     return Ok(());
//   }
// }




#[wasm_bindgen]
pub fn compress(input: &[u8], buffer_size: usize, quality: u32, lg_window_size: u32) -> Vec<u8> {
    let mut output = Vec::new();
    {
        let mut writer = brotli::CompressorWriter::new(&mut output, buffer_size, quality, lg_window_size);
        writer.write(input).unwrap();
    }
    return output;
}

#[wasm_bindgen]
pub fn decompress(input: &[u8], buffer_size: usize) -> Vec<u8> {
    let mut output = Vec::new();
    {
        let mut writer = brotli::DecompressorWriter::new(&mut output, buffer_size);
        writer.write(&input).unwrap();
    }
    return output;
}

#[wasm_bindgen]
pub fn test_compress() -> Vec<u8> {

      let len = BROTELI_TXT_COMPRESSED.len();


    let mut output = Vec::new();
    {
        let mut writer = brotli::DecompressorWriter::new(&mut output, len);
       match writer.write(&BROTELI_TXT_COMPRESSED) {
              Ok(size) => {
                js::log(&format!("size = {:?}", size))

              }
              Err(e) => js::log(&format!("Error {:?}", e)),
        }
    }
    return output;
}
