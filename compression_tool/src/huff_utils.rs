use std::cmp::Ordering;

// Define trait for Huffman tree node implementation
pub trait HuffBaseNode {
    fn is_leaf(&self) -> bool;
    fn weight(&self) -> i32;
}

// Implementation of leaf node
pub struct HuffLeafNode {
    element: char, // Element for this node
    weight: i32,   // Weight for this node
}

// Methods for leaf node
impl HuffLeafNode {
    pub fn new(el: char, wt: i32) -> HuffLeafNode {
        HuffLeafNode {
            element: el,
            weight: wt,
        }
    }

    // Get the element value
    pub fn value(&self) -> char {
        self.element
    }
}

// Implement trait for leaf node
impl HuffBaseNode for HuffLeafNode {
    fn is_leaf(&self) -> bool {
        true
    }

    fn weight(&self) -> i32 {
        self.weight
    }
}

// Implementation of internal node
pub struct HuffInternalNode {
    weight: i32,
    left: Box<dyn HuffBaseNode>,   // Left child
    right: Box<dyn HuffBaseNode>,  // Right child
}

// Methods for internal node
impl HuffInternalNode {
    pub fn new(l: Box<dyn HuffBaseNode>, r: Box<dyn HuffBaseNode>, wt: i32) -> HuffInternalNode {
        HuffInternalNode {
            weight: wt,
            left: l,
            right: r,
        }
    }
}

// Implement trait for internal node
impl HuffBaseNode for HuffInternalNode {
    fn is_leaf(&self) -> bool {
        false
    }

    fn weight(&self) -> i32 {
        self.weight
    }
    
}

pub struct HuffTree {
    root: Box<dyn HuffBaseNode>, // Root node of the tree
}

// Implementation of Huffman tree
impl HuffTree {
    // Constructor for leaf node
    pub fn new_leaf(el: char, wt: i32) -> HuffTree {
        HuffTree {
            root: Box::new(HuffLeafNode::new(el, wt)),
        }
    }

    // Constructor for internal node
    pub fn new_internal(left: Box<dyn HuffBaseNode>, right: Box<dyn HuffBaseNode>, wt: i32) -> HuffTree {
        HuffTree {
            root: Box::new(HuffInternalNode::new(left, right, wt)),
        }
    }

    // Get the root node of the tree
    pub fn root(&self) -> &Box<dyn HuffBaseNode> {
        &self.root
    }

    // Get the weight of the tree, which is the weight of the root node
    pub fn weight(&self) -> i32 {
        self.root.weight()
    }
}

// Implement the Comparable trait for Huffman tree
impl PartialEq for HuffTree {
    fn eq(&self, other: &HuffTree) -> bool {
        self.weight() == other.weight()
    }
}

impl Eq for HuffTree {}

impl PartialOrd for HuffTree {
    fn partial_cmp(&self, other: &HuffTree) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HuffTree {
    fn cmp(&self, other: &HuffTree) -> Ordering {
        self.weight().cmp(&other.weight())
    }
}
/*
// Build a Huffman tree from a priority queue of Huffman trees
pub fn build_tree() -> HuffTree {
    let mut tmp1: HuffTree;
    let mut tmp2: HuffTree;
    let mut tmp3: HuffTree;

    // manipulate heap
    
    tmp3 // Return the final tree
}
 */