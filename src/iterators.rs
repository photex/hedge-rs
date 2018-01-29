//! Iterators for simple or common mesh traversal patterns.

use super::*;

////////////////////////////////////////////////////////////////////////////////

pub struct VertexFnIterator<'mesh> {
    enumerator: VertexEnumerator<'mesh>,
    mesh: &'mesh Mesh,
}

impl<'mesh> VertexFnIterator<'mesh> {
    pub fn new(mesh: &'mesh Mesh) -> VertexFnIterator<'mesh> {
        VertexFnIterator {
            enumerator: mesh.kernel.enumerate_vertices(mesh.next_tag()),
            mesh,
        }
    }
}

impl<'mesh> Iterator for VertexFnIterator<'mesh> {
    type Item = VertexFn<'mesh>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((index, vert)) = self.enumerator.next_element() {
            debug!("Found vertex {:?} - {:?}", index, vert);
            return Some(VertexFn::from_index_and_item(index, vert, self.mesh));
        }
        return None;
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct FaceFnIterator<'mesh> {
    enumerator: FaceEnumerator<'mesh>,
    mesh: &'mesh Mesh,
}

impl<'mesh> FaceFnIterator<'mesh> {
    pub fn new(mesh: &'mesh Mesh) -> FaceFnIterator<'mesh> {
        FaceFnIterator {
            enumerator: mesh.kernel.enumerate_faces(mesh.next_tag()),
            mesh,
        }
    }
}

impl<'mesh> Iterator for FaceFnIterator<'mesh> {
    type Item = FaceFn<'mesh>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((index, face)) = self.enumerator.next_element() {
            debug!("Found face {:?} - {:?}", index, face);
            return Some(FaceFn::from_index_and_item(index, face, self.mesh));
        }
        return None;
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct EdgeFnIterator<'mesh> {
    enumerator: EdgeEnumerator<'mesh>,
    mesh: &'mesh Mesh,
}

impl<'mesh> EdgeFnIterator<'mesh> {
    pub fn new(mesh: &'mesh Mesh) -> EdgeFnIterator<'mesh> {
        EdgeFnIterator {
            enumerator: mesh.kernel.enumerate_edges(mesh.next_tag()),
            mesh,
        }
    }
}

impl<'mesh> Iterator for EdgeFnIterator<'mesh> {
    type Item = EdgeFn<'mesh>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((index, edge)) = self.enumerator.next_element() {
            debug!("Found edge {:?} - {:?}", index, edge);
            return Some(EdgeFn::from_index_and_item(index, edge, self.mesh));
        }
        return None;
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct PointIterator<'mesh> {
    enumerator: PointEnumerator<'mesh>,
    mesh: &'mesh Mesh,
}

impl<'mesh> PointIterator<'mesh> {
    pub fn new(mesh: &'mesh Mesh) -> PointIterator<'mesh> {
        PointIterator {
            enumerator: mesh.kernel.enumerate_points(mesh.next_tag()),
            mesh,
        }
    }
}

impl<'mesh> Iterator for PointIterator<'mesh> {
    type Item = &'mesh Point;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((index, edge)) = self.enumerator.next_element() {
            debug!("Found edge {:?} - {:?}", index, edge);
            return Some(self.mesh.point(index));
        }
        return None;
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct FaceEdges<'mesh> {
    tag: Tag,
    edge: EdgeFn<'mesh>,
}

impl<'mesh> FaceEdges<'mesh> {
    pub fn new(tag: Tag, edge: EdgeFn<'mesh>) -> FaceEdges<'mesh> {
        FaceEdges { tag, edge }
    }
}

impl<'mesh> Iterator for FaceEdges<'mesh> {
    type Item = EdgeFn<'mesh>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.edge.element.props().tag.get() == self.tag {
            return None;
        } else {
            self.edge.element.props().tag.set(self.tag);
            let result = Some(self.edge);
            self.edge = self.edge.next();
            return result;
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct FaceVertices<'mesh> {
    edges: FaceEdges<'mesh>,
}

impl<'mesh> FaceVertices<'mesh> {
    pub fn new(edges: FaceEdges<'mesh>) -> FaceVertices<'mesh> {
        FaceVertices { edges }
    }
}

impl<'mesh> Iterator for FaceVertices<'mesh> {
    type Item = VertexFn<'mesh>;

    fn next(&mut self) -> Option<Self::Item> {
        self.edges.next().map(|e| e.vertex())
    }
}

////////////////////////////////////////////////////////////////////////////////

pub enum CirculatorDirection {
    /// Counter Clock Wise
    CCW,
    /// Clock Wise
    CW,
}

pub struct VertexCirculator<'mesh> {
    tag: Tag,
    direction: CirculatorDirection,
    vertex: VertexFn<'mesh>,
    current_edge: EdgeFn<'mesh>,
}

impl<'mesh> VertexCirculator<'mesh> {
    pub fn new(tag: Tag, vertex: VertexFn<'mesh>) -> VertexCirculator<'mesh> {
        let direction = CirculatorDirection::CCW;
        let current_edge = vertex.edge();
        VertexCirculator {
            tag,
            direction,
            vertex,
            current_edge,
        }
    }
}

impl<'mesh> Iterator for VertexCirculator<'mesh> {
    type Item = EdgeFn<'mesh>;

    fn next(&mut self) -> Option<Self::Item> {
        use CirculatorDirection::*;

        if self.current_edge.element.props().tag.get() == self.tag {
            return None;
        } else {
            self.current_edge.element.props().tag.set(self.tag);
            let result = Some(self.current_edge);

            match self.direction {
                CCW => if self.current_edge.is_boundary() {
                    self.direction = CW;
                    self.current_edge = self.vertex.edge().twin().next();
                } else {
                    self.current_edge = self.current_edge.prev().twin();
                },
                CW => {
                    if self.current_edge.is_boundary() {
                        self.current_edge = self.vertex.edge(); // should terminate iterator
                    } else {
                        self.current_edge = self.current_edge.twin().next();
                    }
                }
            }

            return result;
        }
    }
}
