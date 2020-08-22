//! Quadtree that keeps track of centers of mass.
use crate::quadtree::BoundingBox2D;

fn l2(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    let dx: f32 = x2 - x1;
    let dy: f32 = y2 - y1;
    (dx * dx + dy * dy).sqrt()
}

#[derive(Debug)]
pub struct MassQuadtree2D {
    pub x: f32,
    pub y: f32,
    pub m: f32,
    pub children: Vec<Option<Box<Self>>>,
}

impl MassQuadtree2D {
    // Constructs a child with no children.
    pub fn empty() -> Self {
        Self {
            x: 0.,
            y: 0.,
            m: 0.,
            children: vec![None, None, None, None]
        }
    }
    
    /// Constructs a quadtree for the given bounds and list of points
    pub fn new(x: &Vec<f32>, y: &Vec<f32>, m: &Vec<f32>, bb: BoundingBox2D) -> Self {
        let mut root = Self::empty();
        for i in 0..x.len() {
            root.insert(x[i], y[i], m[i], bb);
        }
        root
    }
    
    /// If node x does not contain a body, put the new body b here.
    /// If node x is an internal node, update the center-of-mass and total mass of x.
    /// Recursively insert the body b in the appropriate quadrant.
    /// If node x is an external node, say containing a body named c,
    /// then there are two bodies b and c in the same region.
    /// Subdivide the region further by creating four children.
    /// Then, recursively insert both b and c into the appropriate quadrant(s).
    /// Since b and c may still end up in the same quadrant, there may be several subdivisions during a single insertion.
    /// Finally, update the center-of-mass and total mass of x.
    pub fn insert(&mut self, x: f32, y: f32, m: f32, bb: BoundingBox2D) {
        // If no body in this node, insert this point.
        if self.m == 0. {
            self.x = x;
            self.y = y;
            self.m = m;
            println!("Free node found, inserted ({}, {}) into {:?}", x, y, bb);
            return;
        }

        // Otherwise, insert the existing node and the new node under this parent.
        let cx: f32 = bb.cx();
        let cy: f32 = bb.cy();
        let mut points: Vec<(f32, f32)> = vec![(x, y)];
        if self.is_leaf() {
            points.push((self.x, self.y));
        }

        for &(x, y) in points.iter() {
            // Find the child and insert into it
            let x_bit = (x >= cx) as usize;
            let y_bit = (y >= cy) as usize;
            let quadrant: usize = x_bit + (y_bit << 1);
            println!("Inserting ({}, {}) into {:?}...", x, y, bb.child(quadrant));

            if self.children[quadrant].is_none() {
                self.children[quadrant] = Some(Box::new(Self::empty()));
            }

            let child: &mut Self = self.children[quadrant].as_mut().unwrap();
            child.insert(x, y, m, bb.child(quadrant));
        }
        
        // Update center of mass
        let total_m: f32 = self.m + m;
        self.x = (self.m * self.x + m * x) / total_m;
        self.y = (self.m * self.y + m * y) / total_m;
        self.m = total_m;
    }

    pub fn is_leaf(&self) -> bool {
        for child in &self.children {
            if child.is_some() {
                return false
            }
        }
        true
    }
}

pub struct MassQuadtree2DIterator<'a> {
    x: f32,
    y: f32,
    theta: f32,
    stack: Vec<(&'a Box<MassQuadtree2D>, BoundingBox2D)>
}

impl<'a> MassQuadtree2DIterator<'a> {
    pub fn new(x: f32, y: f32, theta: f32, tree: &'a Box<MassQuadtree2D>, bb: BoundingBox2D) -> Self {
        Self {
            x,
            y,
            theta,
            stack: vec![(&tree, bb)]
        }
    }
}

// Implement `Iterator` for `Fibonacci`.
// The `Iterator` trait only requires a method to be defined for the `next` element.
impl<'a> Iterator for MassQuadtree2DIterator<'a> {
    type Item = &'a Box<MassQuadtree2D>;

    // Whether a node is or isn't sufficiently far away from a body,
    // depends on the quotient s/d,
    // where s is the width of the region represented by the internal node,
    // and d is the distance between the body and the node's center of mass.
    // The node is sufficiently far away when this ratio is smaller than a threshold value θ.
    // The parameter θ determines the accuracy of the simulation;
    // larger values of θ increase the speed of the simulation but decreases its accuracy.
    // If θ = 0, no internal node is treated as a single body and the algorithm degenerates to a direct-sum algorithm.
    fn next(&mut self) -> Option<&'a Box<MassQuadtree2D>> {
        while !self.stack.is_empty() {
            let (node, bb) = self.stack.pop()?;
            
            let d: f32 = l2(node.x, node.y, self.x, self.y);
            let s: f32 = bb.width();
            if s / d < self.theta || node.is_leaf() {
                if node.is_leaf() {
                    println!("Leaf node: ({}, {})", node.x, node.y);
                } else {
                    println!("Node far enough away (s/d={}): ({}, {}) - ({}, {})",
                    s / d, node.x, node.y, self.x, self.y);
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
    let x: Vec<f32> = vec![100., 300., 400.];
    let y: Vec<f32> = vec![100., 300., 400.];
    let m: Vec<f32> = vec![10., 30., 10.];
    let bb: BoundingBox2D = BoundingBox2D{min_x: 0., max_x: 500., min_y: 0., max_y: 500.};
    let quadtree = MassQuadtree2D::new(&x, &y, &m, bb);
    println!("Tree: {:?}", quadtree);
    let boxed_quadtree = Box::new(quadtree);
    let theta: f32 = 0.5;
    let quadtree_iter = MassQuadtree2DIterator::new(100., 100., theta, &boxed_quadtree, bb);
    for node in quadtree_iter {
        println!("Node: ({}, {}, {})", node.x, node.y, node.m);
    }
}
