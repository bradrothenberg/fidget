//! Standard library of shapes and transforms
use crate::context::Tree;
use facet::Facet;

mod vec;
pub use vec::{Vec2, Vec3, Vec4};

////////////////////////////////////////////////////////////////////////////////
// 2D shapes

/// 2D circle
#[derive(Clone, Facet)]
pub struct Circle {
    /// Center of the circle (in XY)
    pub center: Vec2,
    /// Circle radius
    pub radius: f64,
}

impl From<Circle> for Tree {
    fn from(v: Circle) -> Self {
        let (x, y, _) = Tree::axes();
        ((x - v.center.x).square() + (y - v.center.y).square()).sqrt()
            - v.radius
    }
}

/// Axis-aligned rectangle
#[derive(Clone, Facet)]
pub struct Rect {
    /// Center of the rectangle (in XY)
    pub center: Vec2,
    /// Half-size of the rectangle on each axis
    pub half_size: Vec2,
}

impl From<Rect> for Tree {
    fn from(v: Rect) -> Self {
        let (x, y, _) = Tree::axes();
        let dx = (x - v.center.x).abs() - v.half_size.x;
        let dy = (y - v.center.y).abs() - v.half_size.y;
        let ox = dx.max(0.0);
        let oy = dy.max(0.0);
        (ox.square() + oy.square()).sqrt() + dx.max(dy).min(0.0)
    }
}

////////////////////////////////////////////////////////////////////////////////
// 3D shapes

/// 3D sphere
#[derive(Clone, Facet)]
pub struct Sphere {
    /// Center of the circle (in XYZ)
    pub center: Vec3,
    /// Sphere radius
    pub radius: f64,
}

impl From<Sphere> for Tree {
    fn from(v: Sphere) -> Self {
        let (x, y, z) = Tree::axes();
        ((x - v.center.x).square()
            + (y - v.center.y).square()
            + (z - v.center.z).square())
        .sqrt()
            - v.radius
    }
}

/// Axis-aligned box
#[derive(Clone, Facet)]
pub struct Cuboid {
    /// Center of the box (in XYZ)
    pub center: Vec3,
    /// Half-size of the box on each axis
    pub half_size: Vec3,
}

impl From<Cuboid> for Tree {
    fn from(v: Cuboid) -> Self {
        let (x, y, z) = Tree::axes();
        let dx = (x - v.center.x).abs() - v.half_size.x;
        let dy = (y - v.center.y).abs() - v.half_size.y;
        let dz = (z - v.center.z).abs() - v.half_size.z;
        let ox = dx.max(0.0);
        let oy = dy.max(0.0);
        let oz = dz.max(0.0);
        (ox.square() + oy.square() + oz.square()).sqrt()
            + dx.max(dy.max(dz)).min(0.0)
    }
}

/// Finite cylinder aligned with the Y axis
#[derive(Clone, Facet)]
pub struct Cylinder {
    /// Center of the cylinder (in XYZ)
    pub center: Vec3,
    /// Cylinder radius
    pub radius: f64,
    /// Half-height of the cylinder
    pub half_height: f64,
}

impl From<Cylinder> for Tree {
    fn from(v: Cylinder) -> Self {
        let (x, y, z) = Tree::axes();
        let px = x - v.center.x;
        let py = y - v.center.y;
        let pz = z - v.center.z;
        let dx = (px.square() + pz.square()).sqrt() - v.radius;
        let dy = py.abs() - v.half_height;
        let ox = dx.max(0.0);
        let oy = dy.max(0.0);
        (ox.square() + oy.square()).sqrt() + dx.max(dy).min(0.0)
    }
}

/// Torus aligned with the Y axis
#[derive(Clone, Facet)]
pub struct Torus {
    /// Center of the torus (in XYZ)
    pub center: Vec3,
    /// Major radius of the torus
    pub major_radius: f64,
    /// Radius of the tube
    pub tube_radius: f64,
}

impl From<Torus> for Tree {
    fn from(v: Torus) -> Self {
        let (x, y, z) = Tree::axes();
        let px = x - v.center.x;
        let py = y - v.center.y;
        let pz = z - v.center.z;
        let q = ((px.square() + pz.square()).sqrt() - v.major_radius).square()
            + py.square();
        q.sqrt() - v.tube_radius
    }
}

////////////////////////////////////////////////////////////////////////////////
// CSG operations

/// Take the union of a set of shapes
///
/// If the input is empty, returns an constant empty tree (at +∞)
#[derive(Clone, Facet)]
pub struct Union {
    /// List of shapes to merge
    pub input: Vec<Tree>,
}

impl From<Union> for Tree {
    fn from(v: Union) -> Self {
        if v.input.is_empty() {
            // XXX should this be an error instead?
            Tree::constant(f64::INFINITY)
        } else {
            fn recurse(s: &[Tree]) -> Tree {
                match s.len() {
                    1 => s[0].clone(),
                    n => recurse(&s[..n / 2]).min(recurse(&s[n / 2..])),
                }
            }
            recurse(&v.input)
        }
    }
}

/// Take the intersection of a set of shapes
///
/// If the input is empty, returns a constant full tree (at -∞)
#[derive(Clone, Facet)]
pub struct Intersection {
    /// List of shapes to intersect
    pub input: Vec<Tree>,
}

impl From<Intersection> for Tree {
    fn from(v: Intersection) -> Self {
        if v.input.is_empty() {
            // XXX should this be an error instead?
            Tree::constant(-f64::INFINITY)
        } else {
            fn recurse(s: &[Tree]) -> Tree {
                match s.len() {
                    1 => s[0].clone(),
                    n => recurse(&s[..n / 2]).max(recurse(&s[n / 2..])),
                }
            }
            recurse(&v.input)
        }
    }
}

/// Computes the inverse of a shape
#[derive(Clone, Facet)]
pub struct Inverse {
    /// Shape to invert
    pub shape: Tree,
}

impl From<Inverse> for Tree {
    fn from(v: Inverse) -> Self {
        -v.shape
    }
}

/// Take the difference of two shapes
#[derive(Clone, Facet)]
pub struct Difference {
    /// Original shape
    pub shape: Tree,
    /// Shape to be subtracted from the original
    pub cutout: Tree,
}

impl From<Difference> for Tree {
    fn from(v: Difference) -> Self {
        v.shape.max(-v.cutout)
    }
}

////////////////////////////////////////////////////////////////////////////////
// Morphological operations

/// Uniformly round (or offset) a shape
#[derive(Clone, Facet)]
pub struct Round {
    /// Shape to round
    pub shape: Tree,
    /// Amount to round the surface
    pub radius: f64,
}

impl From<Round> for Tree {
    fn from(v: Round) -> Self {
        v.shape - v.radius
    }
}

/// Form a shell of constant thickness around a shape
#[derive(Clone, Facet)]
pub struct Onion {
    /// Base shape to offset
    pub shape: Tree,
    /// Thickness of the shell
    pub thickness: f64,
}

impl From<Onion> for Tree {
    fn from(v: Onion) -> Self {
        v.shape.abs() - v.thickness
    }
}

/// Repeat a shape with the given periodicity
#[derive(Clone, Facet)]
pub struct Repeat {
    /// Shape to repeat
    pub shape: Tree,
    /// Spacing of the repeating cell
    pub cell: Vec3,
}

impl From<Repeat> for Tree {
    fn from(v: Repeat) -> Self {
        let (x, y, z) = Tree::axes();
        let hx = v.cell.x * 0.5;
        let hy = v.cell.y * 0.5;
        let hz = v.cell.z * 0.5;
        let px = (x + hx).modulo(v.cell.x) - hx;
        let py = (y + hy).modulo(v.cell.y) - hy;
        let pz = (z + hz).modulo(v.cell.z) - hz;
        v.shape.remap_xyz(px, py, pz)
    }
}

/// Twist a shape around the Y axis
#[derive(Clone, Facet)]
pub struct Twist {
    /// Shape to twist
    pub shape: Tree,
    /// Twist factor (angle per unit height)
    pub k: f64,
}

impl From<Twist> for Tree {
    fn from(v: Twist) -> Self {
        let (x, y, z) = Tree::axes();
        let angle = y.clone() * v.k;
        let sin_a = angle.sin();
        let cos_a = angle.cos();
        let rx = x.clone() * cos_a.clone() - z.clone() * sin_a.clone();
        let rz = x * sin_a + z * cos_a;
        v.shape.remap_xyz(rx, y, rz)
    }
}

////////////////////////////////////////////////////////////////////////////////
// Transforms

/// Move a shape
#[derive(Clone, Facet)]
pub struct Move {
    /// Shape to move
    pub shape: Tree,
    /// Position offset
    pub offset: Vec3,
}

impl From<Move> for Tree {
    fn from(v: Move) -> Self {
        v.shape.remap_affine(nalgebra::convert(
            nalgebra::Translation3::<f64>::new(
                -v.offset.x,
                -v.offset.y,
                -v.offset.z,
            ),
        ))
    }
}

/// Non-uniform scaling
#[derive(Clone, Facet)]
pub struct Scale {
    /// Shape to scale
    pub shape: Tree,
    /// Scale to apply on each axis
    pub scale: Vec3,
}

impl From<Scale> for Tree {
    fn from(v: Scale) -> Self {
        v.shape
            .remap_affine(nalgebra::convert(nalgebra::Scale3::<f64>::new(
                1.0 / v.scale.x,
                1.0 / v.scale.y,
                1.0 / v.scale.z,
            )))
    }
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod test {
    use super::*;
    use crate::{shape::EzShape, vm::VmShape};
    use approx::assert_relative_eq;

    #[test]
    fn circle_docstring() {
        assert_eq!(Circle::SHAPE.doc, &[" 2D circle"]);
    }

    #[test]
    fn rect_docstring() {
        assert_eq!(Rect::SHAPE.doc, &[" Axis-aligned rectangle"]);
    }

    #[test]
    fn cuboid_docstring() {
        assert_eq!(Cuboid::SHAPE.doc, &[" Axis-aligned box"]);
    }

    #[test]
    fn cylinder_docstring() {
        assert_eq!(
            Cylinder::SHAPE.doc,
            &[" Finite cylinder aligned with the Y axis"]
        );
    }

    #[test]
    fn torus_docstring() {
        assert_eq!(Torus::SHAPE.doc, &[" Torus aligned with the Y axis"]);
    }

    #[test]
    fn round_docstring() {
        assert_eq!(Round::SHAPE.doc, &[" Uniformly round (or offset) a shape"]);
    }

    #[test]
    fn onion_docstring() {
        assert_eq!(
            Onion::SHAPE.doc,
            &[" Form a shell of constant thickness around a shape"]
        );
    }

    #[test]
    fn repeat_docstring() {
        assert_eq!(
            Repeat::SHAPE.doc,
            &[" Repeat a shape with the given periodicity"]
        );
    }

    #[test]
    fn rect_sdf() {
        let rect = Rect {
            center: Vec2 { x: 0.0, y: 0.0 },
            half_size: Vec2 { x: 1.0, y: 2.0 },
        };
        let shape = VmShape::from(Tree::from(rect));
        let tape = shape.ez_point_tape();
        let mut eval = VmShape::new_point_eval();
        assert_relative_eq!(
            eval.eval(&tape, 0.0f32, 0.0, 0.0).unwrap().0,
            -1.0
        );
        assert_relative_eq!(eval.eval(&tape, 2.0, 0.0, 0.0).unwrap().0, 1.0);
        assert_relative_eq!(eval.eval(&tape, 0.0, 3.0, 0.0).unwrap().0, 1.0);
    }

    #[test]
    fn cuboid_sdf() {
        let cuboid = Cuboid {
            center: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            half_size: Vec3 {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            },
        };
        let shape = VmShape::from(Tree::from(cuboid));
        let tape = shape.ez_point_tape();
        let mut eval = VmShape::new_point_eval();
        assert_relative_eq!(eval.eval(&tape, 0.0, 0.0, 0.0).unwrap().0, -1.0);
        assert_relative_eq!(eval.eval(&tape, 2.0, 0.0, 0.0).unwrap().0, 1.0);
        assert_relative_eq!(eval.eval(&tape, 0.0, 0.0, 4.0).unwrap().0, 1.0);
    }

    #[test]
    fn cylinder_sdf() {
        let cyl = Cylinder {
            center: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            radius: 1.0,
            half_height: 2.0,
        };
        let shape = VmShape::from(Tree::from(cyl));
        let tape = shape.ez_point_tape();
        let mut eval = VmShape::new_point_eval();
        assert_relative_eq!(eval.eval(&tape, 0.0, 0.0, 0.0).unwrap().0, -1.0);
        assert_relative_eq!(eval.eval(&tape, 2.0, 0.0, 0.0).unwrap().0, 1.0);
        assert_relative_eq!(eval.eval(&tape, 0.0, 3.0, 0.0).unwrap().0, 1.0);
        assert_relative_eq!(
            eval.eval(&tape, 2.0, 3.0, 0.0).unwrap().0,
            1.4142135
        );
    }

    #[test]
    fn torus_sdf_values() {
        let torus = Torus {
            center: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            major_radius: 3.0,
            tube_radius: 1.0,
        };
        let shape = VmShape::from(Tree::from(torus));
        let tape = shape.ez_point_tape();
        let mut eval = VmShape::new_point_eval();
        assert_relative_eq!(eval.eval(&tape, 4.0, 0.0, 0.0).unwrap().0, 0.0);
        assert_relative_eq!(eval.eval(&tape, 3.0, 0.0, 0.0).unwrap().0, -1.0);
        assert_relative_eq!(eval.eval(&tape, 0.0, 0.0, 0.0).unwrap().0, 2.0);
    }

    #[test]
    fn round_sdf() {
        let sphere = Sphere {
            center: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            radius: 1.0,
        };
        let round = Round {
            shape: Tree::from(sphere).into(),
            radius: 0.5,
        };
        let shape = VmShape::from(Tree::from(round));
        let tape = shape.ez_point_tape();
        let mut eval = VmShape::new_point_eval();
        assert_relative_eq!(eval.eval(&tape, 0.0, 0.0, 0.0).unwrap().0, -1.5);
        assert_relative_eq!(eval.eval(&tape, 1.5, 0.0, 0.0).unwrap().0, 0.0);
    }

    #[test]
    fn onion_sdf() {
        let sphere = Sphere {
            center: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            radius: 1.0,
        };
        let onion = Onion {
            shape: Tree::from(sphere).into(),
            thickness: 0.1,
        };
        let shape = VmShape::from(Tree::from(onion));
        let tape = shape.ez_point_tape();
        let mut eval = VmShape::new_point_eval();
        assert_relative_eq!(eval.eval(&tape, 0.0, 0.0, 0.0).unwrap().0, 0.9);
        assert_relative_eq!(eval.eval(&tape, 1.0, 0.0, 0.0).unwrap().0, -0.1);
    }

    #[test]
    fn repeat_sdf() {
        let sphere = Sphere {
            center: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            radius: 1.0,
        };
        let repeat = Repeat {
            shape: Tree::from(sphere).into(),
            cell: Vec3 {
                x: 4.0,
                y: 4.0,
                z: 4.0,
            },
        };
        let shape = VmShape::from(Tree::from(repeat));
        let tape = shape.ez_point_tape();
        let mut eval = VmShape::new_point_eval();
        assert_relative_eq!(eval.eval(&tape, 0.0, 0.0, 0.0).unwrap().0, -1.0);
        assert_relative_eq!(eval.eval(&tape, 3.0, 0.0, 0.0).unwrap().0, 0.0);
    }

    #[test]
    fn twist_docstring() {
        assert_eq!(Twist::SHAPE.doc, &[" Twist a shape around the Y axis"]);
    }
}
