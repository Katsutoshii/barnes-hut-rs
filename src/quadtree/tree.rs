//! Quadtree that keeps track of centers of mass.
use super::BoundingBox2D;
use crate::vector::{Scalar, Vector3D};

const EPSILON: Scalar = 1e-4;

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
    /// Constructs a child with no children
    pub fn empty() -> Self {
        Self {
            x: 0.,
            y: 0.,
            m: 0.,
            children: vec![None, None, None, None]
        }
    }

    // Constructs a new child under a node
    pub fn new_child(&mut self, quadrant: usize, x: Scalar, y: Scalar, m: Scalar) {
        // println!("New child ({}, {}, {}) under ({}, {}, {}) in quad {}", x, y, m, self.x, self.y, self.m, quadrant);
        self.children[quadrant] = Some(Box::new(Self {
            x,
            y,
            m,
            children: vec![None, None, None, None]
        }))
    }
    
    /// Constructs a quadtree for the given bounds and list of points
    pub fn new(r: &Vec<Vector3D>, m: &Vec<Scalar>, bb: BoundingBox2D) -> Self {
        let mut root = Self::empty();
        for i in 0..r.len() {
            root.insert(r[i].x, r[i].y, m[i], bb);
        }
        root
    }

    // Updates the center of mass
    pub fn update_com(&mut self, x: Scalar, y: Scalar, m: Scalar) {
        let total_m: Scalar = self.m + m;
        self.x = (self.m * self.x + m * x) / total_m;
        self.y = (self.m * self.y + m * y) / total_m;
        self.m = total_m;
    }
    
    /// Inserts a point into the quadtree.
    pub fn insert(&mut self, x: Scalar, y: Scalar, m: Scalar, bb: BoundingBox2D) {
        // Edge cases: if inserting empty objects or inserting the first element of the tree
        if m == 0. { return }
        if self.m == 0. { self.x = x; self.y = y; self.m = m; return }

        // Find the parent to insert this node under
        let mut parent: &mut Self = self;
        let mut parent_bb: BoundingBox2D = bb;
        let mut quadrant: usize = parent_bb.quadrant(x, y);
        while let Some(_) = &mut parent.children[quadrant] {
            // Update the parent's center of mass
            parent.update_com(x, y, m);

            // Update the bounding box while searching for new parents deeper in the tree
            parent_bb = parent_bb.child(quadrant);
            parent = parent.children[quadrant].as_mut().unwrap();

            // Compute quadrant for next ieration
            quadrant = parent_bb.quadrant(x, y);
        }

        // Leaves must be re-inserted
        if parent.is_leaf() {
            let (px, py, pm) = (parent.x, parent.y, parent.m);

            // Edge case: if the parent is too close to the child, don't insert as child
            if (px - x).abs() < EPSILON && (py - y).abs() < EPSILON { return }

            // Find the center of mass between the two
            parent.update_com(x, y, m);
            let (cx, cy, cm) = (parent.x, parent.y, parent.m);

            // Then split until the parent and child are in separate cells
            let mut parent_quadrant = parent_bb.quadrant(px, py);
            while quadrant == parent_quadrant {
                // Create the cell containing both
                parent.new_child(quadrant, cx, cy, cm);
                parent = parent.children[quadrant].as_mut().unwrap();

                // Split the center and continue down
                parent_bb = parent_bb.child(quadrant);
                quadrant = parent_bb.quadrant(x, y);
                parent_quadrant = parent_bb.quadrant(px, py);
            }
            // Once the quadrants are different, insert the parent into its quadrant
            parent.new_child(parent_quadrant, px, py, pm);
        }
        
        // Insert the new child in the correct quadrant
        parent.new_child(quadrant, x, y, m);
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
            if s / d < self.theta || node.is_leaf() { return Some(node) }
            
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
    // x: 265.56293, y: 263.4189, m: 0.4261353
    // x: 250.0, y: 250.0, m: 5000000.0

    // Initialize the particles
    let r: Vec<Vector3D> = vec![
        Vector3D { x: 265.56293, y: 263.4189, z: 0. },
        Vector3D { x: 250.0, y: 250.0, z: 0. },
        // Vector3D { x: 400., y: 400., z: 0. },
    ];
    // And their masses
    let m: Vec<Scalar> = vec![
        0.4261353,
        5000000.0
        // 10.,
    ];

    // Create a quadtree in the bounding box (0,0),(500, 500)
    let bb: BoundingBox2D = BoundingBox2D{min_x: 0., max_x: 500., min_y: 0., max_y: 500.};
    let quadtree = MassQuadtree::new(&r, &m, bb);
    println!("Tree: {:?}", quadtree);

    // Pass the tree to the iterator in a box
    let boxed_quadtree = Box::new(quadtree);
    let theta: Scalar = 0.5;
    let quadtree_iter = MassQuadtreeIterator::new(250., 250., theta, &boxed_quadtree, bb);

    // Iterate over all contributing nodes of the tree
    for node in quadtree_iter {
        println!("Node: ({}, {}, {})", node.x, node.y, node.m);
    }
}
