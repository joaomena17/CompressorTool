// Compression Tool project using Huffman's algorithm
// Following the guide: https://codingchallenges.substack.com/p/coding-challenge-3

extern crate iterate_text;
use iterate_text::file::characters::IterateFileCharacters;

use std::env;
use std::collections::HashMap;
use std::cmp::Ordering;

#[derive(Debug)]
#[derive(PartialEq)]
enum FuncMode {
    Encode,
    Decode,
}

// Define trait for Huffman tree node implementation
trait HuffBaseNode {
    fn is_leaf(&self) -> bool;
    fn weight(&self) -> i32;
}

// Implementation of leaf node
struct HuffLeafNode {
    element: char, // Element for this node
    weight: i32,   // Weight for this node
}

// Methods for leaf node
impl HuffLeafNode {
    fn new(el: char, wt: i32) -> HuffLeafNode {
        HuffLeafNode {
            element: el,
            weight: wt,
        }
    }

    // Get the element value
    fn value(&self) -> char {
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
struct HuffInternalNode {
    weight: i32,
    left: Box<dyn HuffBaseNode>,   // Left child
    right: Box<dyn HuffBaseNode>,  // Right child
}

// Methods for internal node
impl HuffInternalNode {
    fn new(l: Box<dyn HuffBaseNode>, r: Box<dyn HuffBaseNode>, wt: i32) -> HuffInternalNode {
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

struct HuffTree {
    root: Box<dyn HuffBaseNode>, // Root node of the tree
}

// Implementation of Huffman tree
impl HuffTree {
    // Constructor for leaf node
    fn new_leaf(el: char, wt: i32) -> HuffTree {
        HuffTree {
            root: Box::new(HuffLeafNode::new(el, wt)),
        }
    }

    // Constructor for internal node
    fn new_internal(left: Box<dyn HuffBaseNode>, right: Box<dyn HuffBaseNode>, wt: i32) -> HuffTree {
        HuffTree {
            root: Box::new(HuffInternalNode::new(left, right, wt)),
        }
    }

    // Get the root node of the tree
    fn root(&self) -> &Box<dyn HuffBaseNode> {
        &self.root
    }

    // Get the weight of the tree, which is the weight of the root node
    fn weight(&self) -> i32 {
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

fn parse_args(args: Vec<String>) -> (FuncMode, String){
    match args.len(){
        1 => {
            panic!("No arguments inserted");
        }

        2 => {
            panic!("Insufficent arguments");
        }

        3 => {
            // get source file name from arguments
            let source_name: String = args[2].clone();

            match &args[1][..] {
                "encode" => {
                    let func_mode = FuncMode::Encode;
                    (func_mode, source_name)
                }
                "decode" => {
                    let  func_mode = FuncMode::Decode;
                    (func_mode, source_name)
                }
                _ => {
                    panic!("Invalid mode argument");
                },
            }
        }
        _ => { panic!("Too many arguments"); }
    }
}

fn frequency_of_occurency(path: String) -> HashMap<String, i32> {
    let mut frequency_table: HashMap<String, i32> = HashMap::new();
    let iterator = IterateFileCharacters::new(path);
    for character in iterator {
        let curr_char_counter = frequency_table.entry(character.to_string().clone()).or_insert(0);
        *curr_char_counter += 1;
    }

    frequency_table
}


fn main() {
    // accept file as input and return error if it is not valid

    // collect CLI arguments
    let args: Vec<String> = env::args().collect();

    // validate arguments
    let (_mode, source_name) = parse_args(args);

    // open the file
    // determine the frequency of occurence of each character
    let _frequency_table: HashMap<String, i32> = frequency_of_occurency(source_name);

    // Build a binary tree with these frequencies
    // https://opendsa-server.cs.vt.edu/ODSA/Books/CS3/html/Huffman.html

    // unit test

    // use the tree to generate the prefix-code table
    // https://opendsa-server.cs.vt.edu/ODSA/Books/CS3/html/Huffman.html

    // write a header section to the output file

    // encode the text using the code table
    
    // write the compressed content of the source text to the output file, appending it after the header

    // read in the header of the encoded file and rebuild the prefix table to decode the text

    // read the body and decode it using the prefix table
    
    // write the decoded text back to the specified output file

    // test everything

    // look to add new features or refactor

    return;
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{parse_args, FuncMode, frequency_of_occurency};

    #[test]
    #[should_panic(expected = "No arguments inserted")]
    fn parse_no_args_fail() {
        let mut args = Vec::new();
        args.push("compression_tool/target/debug".to_string());
        parse_args(args);
    }

    #[test]
    #[should_panic(expected = "Insufficent arguments")]
    fn parse_one_args_fail() {
        let mut args = Vec::new();
        args.push("compression_tool/target/debug".to_string());
        args.push("encode".to_string());
        parse_args(args);
    }

    #[test]
    #[should_panic(expected = "Too many arguments")]
    fn parse_too_many_args_fail() {
        let mut args = Vec::new();
        args.push("compression_tool/target/debug".to_string());
        args.push("encode".to_string());
        args.push("../book.txt".to_string());
        args.push("something_extra".to_string());
        parse_args(args);
    }

    #[test]
    fn parse_two_args_pass() {
        let mut args = Vec::new();
        args.push("compression_tool/target/debug".to_string());
        args.push("encode".to_string());
        args.push("../book.txt".to_string());
        assert_eq!(parse_args(args), (FuncMode::Encode, String::from("../book.txt")));
    }

    // verify table: There are 333 occurrences of ‘X’ and 223000 of ‘t’
    #[test]
    fn frequency_of_upper_x_and_t(){
        let table: HashMap<String, i32> = frequency_of_occurency("../book.txt".to_string());
        let freq_of_upper_x: i32 = table.get("X").unwrap().clone();
        let freq_of_t: i32 = table.get("t").unwrap().clone();
        assert_eq!(freq_of_upper_x, 333);
        assert_eq!(freq_of_t, 223000);
    }
}