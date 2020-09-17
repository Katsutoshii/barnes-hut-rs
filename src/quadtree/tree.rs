//! Quadtree that keeps track of centers of mass.
use super::BoundingBox2D;
use crate::vector::{Scalar, Vector3D};
const EPSILON: Scalar = 0.0001;

/// Computes the l2 norm of a 2d vector represented by x1, y1, x2, y2
fn l2(x1: Scalar, y1: Scalar, x2: Scalar, y2: Scalar) -> Scalar {
    let dx: Scalar = x2 - x1;
    let dy: Scalar = y2 - y1;
    (dx * dx + dy * dy).sqrt()
}

/// Definition of the mass quadtree
#[derive(Debug)]
pub struct MassQuadtree {
    pub x: Scalar,
    pub y: Scalar,
    pub m: Scalar,
    pub children: Vec<Option<Box<Self>>>,
}

/// Implementation for the mass quadtree
impl MassQuadtree {
    /// Constructs a child with no children.
    pub fn empty() -> Self {
        Self {
            x: 0.,
            y: 0.,
            m: 0.,
            children: vec![None, None, None, None]
        }
    }
    
    /// Constructs a quadtree for the given bounds and list of points
    pub fn new(r: &Vec<Vector3D>, m: &Vec<Scalar>, bb: BoundingBox2D) -> Self {
        let mut root = Self::empty();
        for i in 0..r.len() {
            // println!();
            root.insert(r[i].x, r[i].y, m[i], bb);
        }
        root
    }
    
    /// Inserts into the quadtree according to the following procedure:
    /// 
    /// If node x does not contain a body, put the new body b here.
    /// If node x is an internal node, update the center-of-mass and total mass of x.
    /// Recursively insert the body b in the appropriate quadrant.
    /// If node x is an external node, say containing a body named c,
    /// then there are two bodies b and c in the same region.
    /// Subdivide the region further by creating four children.
    /// Then, recursively insert both b and c into the appropriate quadrant(s).
    /// Since b and c may still end up in the same quadrant, there may be several subdivisions during a single insertion.
    /// Finally, update the center-of-mass and total mass of x.
    pub fn insert(&mut self, x: Scalar, y: Scalar, m: Scalar, bb: BoundingBox2D) {
        // If no body in this node, insert this point.
        if self.m == 0. {
            self.x = x;
            self.y = y;
            self.m = m;
            // println!("Free node found, inserted ({}, {}) into {:?}", x, y, bb);
            return;
        }

        // Otherwise, insert the existing node and the new node under this parent.
        let cx: Scalar = bb.cx();
        let cy: Scalar = bb.cy();
        let mut points: Vec<(Scalar, Scalar, Scalar)> = vec![(x, y, m)];

        // Insert the parent into itself for leaves
        if self.is_leaf() {
            // Only insert separate points if parent + child aren't too close
            if (self.x - x).abs() > EPSILON && (self.y - y).abs() > EPSILON {
                points.push((self.x, self.y, self.m));
            } else {
                // When too close, just insert the sum of the two points
                points[0] = ((x + self.x) / 2., (y + self.y) / 2., m + self.m);
            }
        }

        for &(x, y, m) in points.iter() {
            // Find the child and insert into it
            let x_bit = (x >= cx) as usize;
            let y_bit = (y >= cy) as usize;
            let quadrant: usize = x_bit + (y_bit << 1);
            // println!("Inserting ({}, {}, {}) into {:?}...", x, y, m, bb.child(quadrant));

            if self.children[quadrant].is_none() {
                self.children[quadrant] = Some(Box::new(Self::empty()));
            }

            let child: &mut Self = self.children[quadrant].as_mut().unwrap();
            child.insert(x, y, m, bb.child(quadrant));
        }
        
        // Update center of mass
        let total_m: Scalar = self.m + m;
        self.x = (self.m * self.x + m * x) / total_m;
        self.y = (self.m * self.y + m * y) / total_m;
        self.m = total_m;
    }

    /// Checks if this node is a leaf
    pub fn is_leaf(&self) -> bool {
        for child in &self.children {
            if child.is_some() {
                return false
            }
        }
        true
    }
}

/// Iterator for iterating over all nearby nodes of the tree
pub struct MassQuadtreeIterator<'a> {
    x: Scalar,
    y: Scalar,
    theta: Scalar,
    stack: Vec<(&'a Box<MassQuadtree>, BoundingBox2D)>
}

/// Implementation of the constructor for the mass quadtree iterator.
impl<'a> MassQuadtreeIterator<'a> {
    /// Constructs a new iterator with the stack initialized to the root.
    pub fn new(x: Scalar, y: Scalar, theta: Scalar, tree: &'a Box<MassQuadtree>, bb: BoundingBox2D) -> Self {
        Self {
            x,
            y,
            theta,
            stack: vec![(&tree, bb)]
        }
    }
}

/// Implements the iterator
impl<'a> Iterator for MassQuadtreeIterator<'a> {
    type Item = &'a Box<MassQuadtree>;

    /// Gets the next node that should count towards the force calculation for the current particle.
    /// 
    /// Whether a node is or isn't sufficiently far away from a body,
    /// depends on the quotient s/d,
    /// where s is the width of the region represented by the internal node,
    /// and d is the distance between the body and the node's center of mass.
    /// The node is sufficiently far away when this ratio is smaller than a threshold value θ.
    /// The parameter θ determines the accuracy of the simulation;
    /// larger values of θ increase the speed of the simulation but decreases its accuracy.
    /// If θ = 0, no internal node is treated as a single body and the algorithm degenerates to a direct-sum algorithm.
    fn next(&mut self) -> Option<&'a Box<MassQuadtree>> {
        while !self.stack.is_empty() {
            let (node, bb) = self.stack.pop()?;
            
            let d: Scalar = l2(node.x, node.y, self.x, self.y);
            let s: Scalar = bb.width();
            if s / d < self.theta || node.is_leaf() {
                if node.is_leaf() {
                    // println!("Leaf node: ({}, {})", node.x, node.y);
                } else {
                    // println!("Node far enough away (s/d={}): ({}, {}) - ({}, {})",
                    //     s / d, node.x, node.y, self.x, self.y);
                }
                return Some(node);
            }
            
            // If not far enough away, add children to the stack.
            for (quadrant, child) in node.children.iter().enumerate() {
                match child {
                    Some(child) => self.stack.push((child, bb.child(quadrant))),
                    None => (),
                }
            }
        }
        None
    }
}

#[test]
fn test_quadtree() {
    // Initialize the particles
    let r: Vec<Vector3D> = vec![
        Vector3D { x: 100., y: 100., z: 0. },
        Vector3D { x: 100., y: 100., z: 0. },
        Vector3D { x: 300., y: 300., z: 0. },
        Vector3D { x: 400., y: 400., z: 0. },
    ];
    // And their masses
    let m: Vec<Scalar> = vec![
        10.,
        10.,
        30.,
        10.,
    ];

    // Create a quadtree in the bounding box (0,0),(500, 500)
    let bb: BoundingBox2D = BoundingBox2D{min_x: 0., max_x: 500., min_y: 0., max_y: 500.};
    let quadtree = MassQuadtree::new(&r, &m, bb);
    println!("Tree: {:?}", quadtree);

    // Pass the tree to the iterator in a box
    let boxed_quadtree = Box::new(quadtree);
    let theta: Scalar = 0.5;
    let quadtree_iter = MassQuadtreeIterator::new(100., 100., theta, &boxed_quadtree, bb);

    // Iterate over all contributing nodes of the tree
    for node in quadtree_iter {
        println!("Node: ({}, {}, {})", node.x, node.y, node.m);
    }
}
