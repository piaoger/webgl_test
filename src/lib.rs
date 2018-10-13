#![feature(core_intrinsics, chunks_exact)]
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
pub mod uyun;
use wasm_bindgen::prelude::*;

pub use js::*;
pub use mains::*;
pub use random::*;
pub use uyun::*;
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
extern crate decorum;
extern crate delaunator;
extern crate gltf;
extern crate plexus;
extern crate rscsg;
extern crate smallpt;

// uyun
extern crate futures;
extern crate js_sys;
//extern crate wasm_bindgen;
extern crate wasm_bindgen_futures;
extern crate web_sys;
#[macro_use]
extern crate serde_derive;

use decorum::R32;
use na::{Point2, Point3};
use plexus::{
    prelude::*,
    primitive::{
        self,
        cube::{Cube, Plane},
        HashIndexer,
    },
};

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
        })
        .triangulate()
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
pub fn test_mesh() -> Vec<u8> {
    let x = EdgeIndex::new(0usize);
    js::log("test mesh");

    test_plexus();
    test_sphere();

    let bitmap = raytrace::render();
    let y = x;

    bitmap
}

//
// disable for build error in src/random.rs
// caused by rand::random
use rand::{Rng, SeedableRng, XorShiftRng};
use std::iter::repeat_with;

const N: usize = 1_000_000;

#[wasm_bindgen]
pub fn test_delaunator() {
    js::log("begin delaunator");

    let mut rng = XorShiftRng::from_seed([0; 16]);
    let points: Vec<_> = repeat_with(|| rng.gen())
        .map(|(x, y)| delaunator::Point { x, y })
        .take(N)
        .collect();

    let result = delaunator::triangulate(&points)
        .expect("No triangulation exists for this input.");

    let v = format!(
        "Triangulated {} points.\nGenerated {} triangles. Convex hull size: \
         {}",
        N,
        result.len(),
        result.hull.len()
    );

    js::log(&v);
}

///
/// gltf
// png-decoder (rayon) is used, so that it will crashes this app
// gltf crate should be modified
//
// notice: gltf used about 800KB in resultant wasm file.
//
use gltf::Gltf;
use std::{
    boxed::Box,
    error::Error as StdError,
    io::{self, Cursor},
};

pub const BOX_GLTF: &'static [u8] = include_bytes!("../assets/box.gltf");

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
pub fn run_gltf() {
    let gltf = gltf::Gltf::from_reader(Cursor::new(BOX_GLTF)).unwrap();

    // std::time::Instant::now() is not available for wasm
    // RuntimeError: unreachable
    // at __rust_start_panic (wasm-function[32403]:1)
    // at rust_panic (wasm-function[32400]:30)
    // at std::panicking::rust_panic_with_hook::h8483c444dde558e8 (wasm-function[32395]:448)
    // at std::panicking::begin_panic::hed6abd73d4f40a34 (wasm-function[32390]:40)
    // at std::sys::wasm::TimeSysCall::perform::hb743778cdf763bbf (wasm-function[32345]:13)
    // at std::time::Instant::now::h4099eac0a348b9b4 (wasm-function[32301]:16)
    // at webgl_test::run_gltf::h374960a9b80f7091 (wasm-function[14961]:208)
    // at run_gltf (wasm-function[14962]:35)
    // at Module.run_gltf (http://localhost:11484/0.index.js:452:67)
    // at ./lib/index.js.webgl_test.then.bg (http://localhost:11484/index.js:707:8)
    //let now = std::time::Instant::now();

    for scene in gltf.scenes() {
        js::log(&format!("Scene {}", scene.index()));
        #[cfg(feature = "names")]
        js::log(&format!(" ({:?})", scene.name().unwrap_or("<Unnamed>")));

        for node in scene.nodes() {
            print_tree(&node, 1);
        }
    }

    //let elapsed = now.elapsed();
    //js::log(&format!(" elapsed: {:?}", elapsed));
}

// About 1MB
extern crate brotli;

use std::io::Write;

pub const BROTELI_TXT: &'static [u8] = include_bytes!("../assets/asyoulik.txt");
pub const BROTELI_TXT_COMPRESSED: &'static [u8] =
    include_bytes!("../assets/asyoulik.txt.compressed");

#[wasm_bindgen]
pub fn compress(
    input: &[u8],
    buffer_size: usize,
    quality: u32,
    lg_window_size: u32,
) -> Vec<u8> {
    let mut output = Vec::new();
    {
        let mut writer = brotli::CompressorWriter::new(
            &mut output,
            buffer_size,
            quality,
            lg_window_size,
        );
        writer.write(input).unwrap();
    }
    return output;
}

#[wasm_bindgen]
pub fn decompress(input: &[u8], buffer_size: usize) -> Vec<u8> {
    let mut output = Vec::new();
    {
        let mut writer =
            brotli::DecompressorWriter::new(&mut output, buffer_size);
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
            Ok(size) => js::log(&format!("size = {:?}", size)),
            Err(e) => js::log(&format!("Error {:?}", e)),
        }
    }
    return output;
}

// use futures::{future, Future};
// use js_sys::Promise;
// //use wasm_bindgen::prelude::*;
// use wasm_bindgen::JsCast;
// use wasm_bindgen_futures::future_to_promise;
// use wasm_bindgen_futures::JsFuture;
// use web_sys::{Request, RequestInit, RequestMode, Response};

// #[derive(Debug, Serialize, Deserialize)]
// struct AccountInfo {
//     #[serde(rename = "identityName")]
//     identity_name: String,
//     #[serde(rename = "displayname")]
//     display_name: String,
//     #[serde(rename = "headimgurl")]
//     avatar: String,
//     #[serde(rename = "id")]
//     id: String,
// }

// #[wasm_bindgen]
// pub fn run() -> Promise {
//     let mut opts = RequestInit::new();
//     opts.method("GET");
//     opts.mode(RequestMode::Cors);

//     let accout_id = "eb64654a9c904612a63bf2a9197ad692";
//     let end_point = format!("{}{}", "https://yun.uzhujia.com/api/accounts/v1/accounts/", accout_id);

//     let request = Request::new_with_str_and_init(
//         &end_point,
//         &opts,
//     ).unwrap();

//     request
//         .headers()
//         .set("Accept", "application/json")
//         .unwrap();

//     let window = web_sys::window().unwrap();
//     let request_promise = window.fetch_with_request(&request);

//     let future = JsFuture::from(request_promise)
//         .and_then(|resp_value| {
//             // `resp_value` is a `Response` object.
//             assert!(resp_value.is_instance_of::<Response>());
//             let resp: Response = resp_value.dyn_into().unwrap();
//             resp.json()

//         }).and_then(|json_value: Promise| {
//             // Convert this other `Promise` into a rust `Future`.
//             JsFuture::from(json_value)

//         }).and_then(|json| {
//             // Use serde to parse the JSON into a struct.
//             let account_info: AccountInfo = json.into_serde().unwrap();

//             // Send the `Branch` struct back to JS as an `Object`.
//             future::ok(JsValue::from_serde(&account_info).unwrap())
//         });

//     // Convert this Rust `Future` back into a JS `Promise`.
//     future_to_promise(future)
// }
