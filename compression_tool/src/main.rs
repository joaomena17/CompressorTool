// Compression Tool project using Huffman's algorithm
// Following the guide: https://codingchallenges.substack.com/p/coding-challenge-3

use std::env;
use std::fs::File;

enum FuncMode {
    Encode,
    Decode,
}


fn main() {
    // accept file as input and return error if it is not valid

    // collect CLI arguments
    let args: Vec<String> = env::args().collect();

    // validate arguments
    match args.len(){
        1 => {
            panic!("No arguments inserted");
        }

        2 => {
            panic!("Insufficent arguments");
        }

        3 => {
            match &args[1][..] {
                "encode" => {
                    let mode: FuncMode = FuncMode::Encode;
                }
                "decode" => {
                    let mode: FuncMode = FuncMode::Decode;
                }
                _ => {
                    panic!("Invalid mode argument");
                },
            }
        }

        _ => { panic!("Too many arguments"); }
    }

    // get source file name from arguments
    let source_name: String = args[2].clone();

    // open the file
    let source_file = match File::open(source_name){
        Ok(file) => file,
        Err(err) => panic!("Unable to open the file: {:?}", err),
    };

    // determine the occurence of each character

    // verify table: There are 333 occurrences of ‘X’ and 223000 of ‘t’

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

