
//!
//! An index based half-edge mesh implementation.
//!

#[macro_use]
extern crate log;

extern crate cgmath;

use std::fmt;
use std::sync::atomic::{AtomicUsize, Ordering};

pub use core::*;
pub use function_sets::*;
pub use iterators::*;

pub mod core;
pub mod function_sets;
pub mod iterators;


/// Storage interface for Mesh types
#[derive(Debug, Default)]
pub struct Kernel {
    edge_buffer: ElementBuffer<Edge>,
    face_buffer: ElementBuffer<Face>,
    vertex_buffer: ElementBuffer<Vertex>,
    point_buffer: ElementBuffer<Point>,
}

impl Kernel {
}

////////////////////////////////////////////////////////////////////////////////////////////////////


pub struct Mesh {
    kernel: Kernel,
    tag: AtomicUsize,
}

impl fmt::Debug for Mesh {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Half-Edge Mesh {{ {} points, {} vertices, {} edges, {} faces }}",
               self.point_count(), self.vertex_count(),
               self.edge_count(), self.face_count())
    }
}

impl Mesh {
    /// Creates a new Mesh with an initial component added to each Vec.
    ///
    /// The idea behind having a single invalid component at the front of each
    /// Vec comes from the blog http://ourmachinery.com/post/defaulting-to-zero/
    pub fn new() -> Mesh {
        Mesh {
            kernel: Kernel::default(),
            tag: AtomicUsize::new(1),
        }
    }

    fn next_tag(&self) -> usize {
        self.tag.fetch_add(1, Ordering::SeqCst)
    }

    /// Returns a `FaceFn` for the given index.
    pub fn face(&self, index: FaceIndex) -> FaceFn {
        FaceFn::new(index, &self)
    }

    pub fn face_count(&self) -> usize {
        self.kernel.face_buffer.len() - 1
    }

    //pub fn faces<'mesh>(&self) -> FaceFnIterator<'mesh> {
    pub fn faces<'mesh>(&'mesh self) -> FaceFnIterator<'mesh> {
        let current_tag = self.tag.fetch_add(1, Ordering::SeqCst);
        FaceFnIterator::new(current_tag, self.kernel.face_buffer.enumerate(), &self)
    }

    pub fn vertices<'mesh>(&'mesh self) -> VertexFnIterator<'mesh> {
        //let current_tag = self.tag.fetch_add(1, Ordering::SeqCst);
        VertexFnIterator::new(&self)
    }

    /// Returns an `EdgeFn` for the given index.
    pub fn edge(&self, index: EdgeIndex) -> EdgeFn {
        EdgeFn::new(index, &self)
    }

    pub fn edge_count(&self) -> usize {
        self.kernel.edge_buffer.len() - 1
    }

    /// Returns a `VertexFn` for the given index.
    pub fn vertex(&self, index: VertexIndex) -> VertexFn {
        VertexFn::new(index, &self)
    }

    pub fn vertex_count(&self) -> usize {
        self.kernel.vertex_buffer.len() - 1
    }

    pub fn point(&self, index: PointIndex) -> &Point {
        self.kernel.point_buffer.get(&index)
    }

    pub fn point_count(&self) -> usize {
        self.kernel.point_buffer.len() - 1
    }
}

////////////////////////////////////////////////////////////////////////////////
// Adding elements

impl AddElement<Vertex> for Mesh {
    fn add(&mut self, vertex: Vertex) -> VertexIndex {
        self.kernel.vertex_buffer.add(vertex)
    }
}

impl AddElement<Edge> for Mesh {
    fn add(&mut self, edge: Edge) -> EdgeIndex {
        self.kernel.edge_buffer.add(edge)
    }
}

impl AddElement<Face> for Mesh {
    fn add(&mut self, face: Face) -> FaceIndex {
        self.kernel.face_buffer.add(face)
    }
}

impl AddElement<Point> for Mesh {
    fn add(&mut self, point: Point) -> PointIndex {
        self.kernel.point_buffer.add(point)
    }
}

////////////////////////////////////////////////////////////////////////////////
// Removing elements

impl RemoveElement<Vertex> for Mesh {
    fn remove(&mut self, index: VertexIndex) {
        self.kernel.vertex_buffer.remove(index);
    }
}

impl RemoveElement<Edge> for Mesh {
    fn remove(&mut self, index: EdgeIndex) {
        self.kernel.edge_buffer.remove(index);
    }
}

impl RemoveElement<Face> for Mesh {
    fn remove(&mut self, index: FaceIndex) {
        self.kernel.face_buffer.remove(index);
    }
}

impl RemoveElement<Point> for Mesh {
    fn remove(&mut self, index: PointIndex) {
        self.kernel.point_buffer.remove(index);
    }
}

////////////////////////////////////////////////////////////////////////////////
// Get immutable references

impl GetElement<Vertex> for Mesh {
    fn get(&self, index: &VertexIndex) -> &Vertex {
        self.kernel.vertex_buffer.get(index)
    }
}

impl GetElement<Edge> for Mesh {
    fn get(&self, index: &EdgeIndex) -> &Edge {
        self.kernel.edge_buffer.get(index)
    }
}

impl GetElement<Face> for Mesh {
    fn get(&self, index: &FaceIndex) -> &Face {
        self.kernel.face_buffer.get(index)
    }
}

impl GetElement<Point> for Mesh {
    fn get(&self, index: &PointIndex) -> &Point {
        self.kernel.point_buffer.get(index)
    }
}

////////////////////////////////////////////////////////////////////////////////
// Get mutable references

impl GetElementMut<Vertex> for Mesh {
    fn get_mut(&mut self, index: &VertexIndex) -> Option<&mut Vertex> {
        self.kernel.vertex_buffer.get_mut(index)
    }
}

impl GetElementMut<Edge> for Mesh {
    fn get_mut(&mut self, index: &EdgeIndex) -> Option<&mut Edge> {
        self.kernel.edge_buffer.get_mut(index)
    }
}

impl GetElementMut<Face> for Mesh {
    fn get_mut(&mut self, index: &FaceIndex) -> Option<&mut Face> {
        self.kernel.face_buffer.get_mut(index)
    }
}

impl GetElementMut<Point> for Mesh {
    fn get_mut(&mut self, index: &PointIndex) -> Option<&mut Point> {
        self.kernel.point_buffer.get_mut(index)
    }
}


#[cfg(test)]
mod tests;
