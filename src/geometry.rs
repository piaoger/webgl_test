use na;
use std::intrinsics;

pub type CubeCoord = na::Point3<isize>;

#[derive(Clone, Debug)]
pub struct CubeRing {
    cube:   CubeCoord,
    radius: usize,
    i:      u8,
    j:      usize,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
#[repr(u8)]
pub enum HexDir {
    Southeast = 0,
    Northeast = 1,
    North = 2,
    Northwest = 3,
    Southwest = 4,
    South = 5,
}

pub const SQRT_3_ON_2: f32 = 0.866_025_4;
pub const SQRT_3: f32 = 1.732_050_8;

#[cfg_attr(rustfmt, rustfmt_skip)]
pub const HEXAGON: [[f32; 2]; 6] = [
    [ 1.0,  0.0        ],
    [ 0.5,  SQRT_3_ON_2],
    [-0.5,  SQRT_3_ON_2],
    [-1.0,  0.0        ],
    [-0.5, -SQRT_3_ON_2],
    [ 0.5, -SQRT_3_ON_2],
];

#[cfg_attr(rustfmt, rustfmt_skip)]
pub const HEXAGON_VERTS: &[f32] = &[
     0.0,  0.0,
     1.0,  0.0,
     0.5,  SQRT_3_ON_2,
    -0.5,  SQRT_3_ON_2,
    -1.0,  0.0,
    -0.5, -SQRT_3_ON_2,
     0.5, -SQRT_3_ON_2,
];

#[cfg_attr(rustfmt, rustfmt_skip)]
pub const HEXAGON_INDICES: &[u16] = &[
    0, 1, 2,
    0, 2, 3,
    0, 3, 4,
    0, 4, 5,
    0, 5, 6,
    0, 6, 1,
];

#[cfg_attr(rustfmt, rustfmt_skip)]
pub const HEXAGONAL_PRISM_VERTS: &[f32] = &[
     0.0,  0.0,           0.0,
     1.0,  0.0,           0.0,
     0.5,  SQRT_3_ON_2,   0.0,
    -0.5,  SQRT_3_ON_2,   0.0,
    -1.0,  0.0,           0.0,
    -0.5, -SQRT_3_ON_2,   0.0,
     0.5, -SQRT_3_ON_2,   0.0,
     1.0,  0.0,         -12.0,
     0.5,  SQRT_3_ON_2, -12.0,
    -0.5,  SQRT_3_ON_2, -12.0,
    -1.0,  0.0,         -12.0,
    -0.5, -SQRT_3_ON_2, -12.0,
     0.5, -SQRT_3_ON_2, -12.0,
];

#[cfg_attr(rustfmt, rustfmt_skip)]
pub const HEXAGONAL_PRISM_INDICES: &[usize] = &[
    //////////////// Hexagon ////////////////
    0, 1, 2,
    0, 2, 3,
    0, 3, 4,
    0, 4, 5,
    0, 5, 6,
    0, 6, 1,
    //////////////// Prism ////////////////
    1,  7,  2,
    2,  7,  8,
    2,  8,  3,
    3,  8,  9,
    3,  9,  4,
    4,  9, 10,
    4, 10,  5,
    5, 10, 11,
    5, 11,  6,
    6, 11, 12,
    6, 12,  1,
    1, 12,  7,
];

#[cfg_attr(rustfmt, rustfmt_skip)]
pub const HEXAGONAL_PRISM: &[f32] = &[
     0.0,  0.0,           0.0,
     1.0,  0.0,           0.0,
     0.5,  SQRT_3_ON_2,   0.0,
     0.0,  0.0,           0.0,
     0.5,  SQRT_3_ON_2,   0.0,
    -0.5,  SQRT_3_ON_2,   0.0,
     0.0,  0.0,           0.0,
    -0.5,  SQRT_3_ON_2,   0.0,
    -1.0,  0.0,           0.0,
     0.0,  0.0,           0.0,
    -1.0,  0.0,           0.0,
    -0.5, -SQRT_3_ON_2,   0.0,
     0.0,  0.0,           0.0,
    -0.5, -SQRT_3_ON_2,   0.0,
     0.5, -SQRT_3_ON_2,   0.0,
     0.0,  0.0,           0.0,
     0.5, -SQRT_3_ON_2,   0.0,
     1.0,  0.0,           0.0,
     1.0,  0.0,           0.0,
     1.0,  0.0,         -12.0,
     0.5,  SQRT_3_ON_2,   0.0,
     0.5,  SQRT_3_ON_2,   0.0,
     1.0,  0.0,         -12.0,
     0.5,  SQRT_3_ON_2, -12.0,
     0.5,  SQRT_3_ON_2,   0.0,
     0.5,  SQRT_3_ON_2, -12.0,
    -0.5,  SQRT_3_ON_2,   0.0,
    -0.5,  SQRT_3_ON_2,   0.0,
     0.5,  SQRT_3_ON_2, -12.0,
    -0.5,  SQRT_3_ON_2, -12.0,
    -0.5,  SQRT_3_ON_2,   0.0,
    -0.5,  SQRT_3_ON_2, -12.0,
    -1.0,  0.0,           0.0,
    -1.0,  0.0,           0.0,
    -0.5,  SQRT_3_ON_2, -12.0,
    -1.0,  0.0,         -12.0,
    -1.0,  0.0,           0.0,
    -1.0,  0.0,         -12.0,
    -0.5, -SQRT_3_ON_2,   0.0,
    -0.5, -SQRT_3_ON_2,   0.0,
    -1.0,  0.0,         -12.0,
    -0.5, -SQRT_3_ON_2, -12.0,
    -0.5, -SQRT_3_ON_2,   0.0,
    -0.5, -SQRT_3_ON_2, -12.0,
     0.5, -SQRT_3_ON_2,   0.0,
     0.5, -SQRT_3_ON_2,   0.0,
    -0.5, -SQRT_3_ON_2, -12.0,
     0.5, -SQRT_3_ON_2, -12.0,
     0.5, -SQRT_3_ON_2,   0.0,
     0.5, -SQRT_3_ON_2, -12.0,
     1.0,  0.0,           0.0,
     1.0,  0.0,           0.0,
     0.5, -SQRT_3_ON_2, -12.0,
     1.0,  0.0,         -12.0,
];

#[cfg_attr(rustfmt, rustfmt_skip)]
pub const HEXAGONAL_PRISM_NORMALS: &[f32] = &[
    // Hexagon
    0.0, 0.0, 1.0,
    0.0, 0.0, 1.0,
    0.0, 0.0, 1.0,
    0.0, 0.0, 1.0,
    0.0, 0.0, 1.0,
    0.0, 0.0, 1.0,
    0.0, 0.0, 1.0,
    0.0, 0.0, 1.0,
    0.0, 0.0, 1.0,
    0.0, 0.0, 1.0,
    0.0, 0.0, 1.0,
    0.0, 0.0, 1.0,
    0.0, 0.0, 1.0,
    0.0, 0.0, 1.0,
    0.0, 0.0, 1.0,
    0.0, 0.0, 1.0,
    0.0, 0.0, 1.0,
    0.0, 0.0, 1.0,
    // Prism: first face, northeast
    SQRT_3_ON_2, 0.5, 0.0,
    SQRT_3_ON_2, 0.5, 0.0,
    SQRT_3_ON_2, 0.5, 0.0,
    SQRT_3_ON_2, 0.5, 0.0,
    SQRT_3_ON_2, 0.5, 0.0,
    SQRT_3_ON_2, 0.5, 0.0,
    // Prism: second face, north
    0.0, 1.0, 0.0,
    0.0, 1.0, 0.0,
    0.0, 1.0, 0.0,
    0.0, 1.0, 0.0,
    0.0, 1.0, 0.0,
    0.0, 1.0, 0.0,
    // Prism: third face, northwest
    -SQRT_3_ON_2, 0.5, 0.0,
    -SQRT_3_ON_2, 0.5, 0.0,
    -SQRT_3_ON_2, 0.5, 0.0,
    -SQRT_3_ON_2, 0.5, 0.0,
    -SQRT_3_ON_2, 0.5, 0.0,
    -SQRT_3_ON_2, 0.5, 0.0,
    // Prism: fourth face, southwest
    -SQRT_3_ON_2, -0.5, 0.0,
    -SQRT_3_ON_2, -0.5, 0.0,
    -SQRT_3_ON_2, -0.5, 0.0,
    -SQRT_3_ON_2, -0.5, 0.0,
    -SQRT_3_ON_2, -0.5, 0.0,
    -SQRT_3_ON_2, -0.5, 0.0,
    // Prism: fifth face, south
    0.0, -1.0, 0.0,
    0.0, -1.0, 0.0,
    0.0, -1.0, 0.0,
    0.0, -1.0, 0.0,
    0.0, -1.0, 0.0,
    0.0, -1.0, 0.0,
    // Prism: sixth face, southeast
    SQRT_3_ON_2, -0.5, 0.0,
    SQRT_3_ON_2, -0.5, 0.0,
    SQRT_3_ON_2, -0.5, 0.0,
    SQRT_3_ON_2, -0.5, 0.0,
    SQRT_3_ON_2, -0.5, 0.0,
    SQRT_3_ON_2, -0.5, 0.0,
];

#[cfg_attr(rustfmt, rustfmt_skip)]
pub const CUBE_VERTICES: &[f32] = &[
    -1.0,  1.0, -1.0,
    -1.0, -1.0, -1.0,
     1.0, -1.0, -1.0,
     1.0, -1.0, -1.0,
     1.0,  1.0, -1.0,
    -1.0,  1.0, -1.0,
    //
    -1.0, -1.0,  1.0,
    -1.0, -1.0, -1.0,
    -1.0,  1.0, -1.0,
    -1.0,  1.0, -1.0,
    -1.0,  1.0,  1.0,
    -1.0, -1.0,  1.0,
    //
    1.0, -1.0, -1.0,
    1.0, -1.0,  1.0,
    1.0,  1.0,  1.0,
    1.0,  1.0,  1.0,
    1.0,  1.0, -1.0,
    1.0, -1.0, -1.0,
    //
    -1.0, -1.0, 1.0,
    -1.0,  1.0, 1.0,
     1.0,  1.0, 1.0,
     1.0,  1.0, 1.0,
     1.0, -1.0, 1.0,
    -1.0, -1.0, 1.0,
    //
    -1.0, 1.0, -1.0,
     1.0, 1.0, -1.0,
     1.0, 1.0,  1.0,
     1.0, 1.0,  1.0,
    -1.0, 1.0,  1.0,
    -1.0, 1.0, -1.0,
    //
    -1.0, -1.0, -1.0,
    -1.0, -1.0,  1.0,
     1.0, -1.0, -1.0,
     1.0, -1.0, -1.0,
    -1.0, -1.0,  1.0,
     1.0, -1.0,  1.0,
];

impl CubeRing {
    /// Doesn't work when `radius == 0`.
    #[inline]
    pub fn new(center: CubeCoord, radius: usize) -> Self {
        Self {
            cube: center + na::Vector3::new(-1, 0, 1) * radius as isize,
            radius,
            i: 0,
            j: 0,
        }
    }

    #[inline]
    pub fn get_radius(&self) -> usize {
        self.radius
    }
}

impl Iterator for CubeRing {
    type Item = CubeCoord;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(i_dir) = HexDir::from_u8(self.i) {
            if self.j < self.radius {
                self.j += 1;
                let ret = self.cube;
                self.cube = cube_neighbor(self.cube, i_dir);

                Some(ret)
            } else {
                self.j = 1;
                self.i += 1;
                if let Some(i_dir) = HexDir::from_u8(self.i) {
                    let ret = self.cube;
                    self.cube = cube_neighbor(self.cube, i_dir);

                    Some(ret)
                } else {
                    None
                }
            }
        } else {
            None
        }
    }
}

impl HexDir {
    #[inline]
    pub fn from_u8(b: u8) -> Option<Self> {
        match b {
            n if n == HexDir::Southeast as u8 => Some(HexDir::Southeast),
            n if n == HexDir::Northeast as u8 => Some(HexDir::Northeast),
            n if n == HexDir::North as u8 => Some(HexDir::North),
            n if n == HexDir::Northwest as u8 => Some(HexDir::Northwest),
            n if n == HexDir::Southwest as u8 => Some(HexDir::Southwest),
            n if n == HexDir::South as u8 => Some(HexDir::South),
            _ => None,
        }
    }
}

#[inline]
pub fn cube_direction(dir: HexDir) -> na::Vector3<isize> {
    match dir {
        HexDir::Southeast => na::Vector3::new(1, -1, 0),
        HexDir::Northeast => na::Vector3::new(1, 0, -1),
        HexDir::North => na::Vector3::new(0, 1, -1),
        HexDir::Northwest => na::Vector3::new(-1, 1, 0),
        HexDir::Southwest => na::Vector3::new(-1, 0, 1),
        HexDir::South => na::Vector3::new(0, -1, 1),
    }
}

#[inline]
pub fn cube_neighbor(cc: CubeCoord, dir: HexDir) -> CubeCoord {
    cc + cube_direction(dir)
}

#[inline]
pub fn cube_to_indices(cc: CubeCoord, radius: usize) -> (usize, usize) {
    (
        cc[2] as usize,
        cc[0] as usize - radius.saturating_sub(cc[2] as usize),
    )
}

#[inline]
fn hex_round(cc: na::Point3<f32>) -> CubeCoord {
    let (x, y, z) = (cc[0], cc[1], cc[2]);

    let rx = x.round();
    let ry = y.round();
    let rz = z.round();

    unsafe {
        let x_diff = intrinsics::fsub_fast(rx, x).abs();
        let y_diff = intrinsics::fsub_fast(ry, y).abs();
        let z_diff = intrinsics::fsub_fast(rz, z).abs();

        if x_diff > y_diff && x_diff > z_diff {
            let (ry, rz) = (ry as isize, rz as isize);

            na::Point3::new(-(ry + rz), ry, rz)
        } else if y_diff > z_diff {
            let (rx, rz) = (rx as isize, rz as isize);

            na::Point3::new(rx, -(rx + rz), rz)
        } else {
            let (rx, ry) = (rx as isize, ry as isize);

            na::Point3::new(rx, ry, -(rx + ry))
        }
    }
}

#[inline]
pub fn pixel_to_cube(x: f32, y: f32) -> CubeCoord {
    const SQRT_3_ON_3: f32 = 0.577_350_26;
    const TWO_ON_3: f32 = 2.0 / 3.0;
    const NEG_1_ON_3: f32 = -1.0 / 3.0;

    unsafe {
        let q = intrinsics::fmul_fast(TWO_ON_3, x);
        let r = intrinsics::fmaf32(
            SQRT_3_ON_3,
            y,
            intrinsics::fmul_fast(NEG_1_ON_3, x),
        );

        hex_round(na::Point3::new(q, r, -intrinsics::fadd_fast(q, r)))
    }
}

#[inline]
pub fn axial_to_cartesian(q: f32, r: f32) -> (f32, f32) {
    unsafe {
        (
            intrinsics::fmul_fast(1.5, q),
            intrinsics::fmaf32(
                SQRT_3_ON_2,
                q,
                intrinsics::fmul_fast(SQRT_3, r),
            ),
        )
    }
}
