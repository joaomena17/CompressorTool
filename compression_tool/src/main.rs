// Compression Tool project using Huffman's algorithm
// Following the guide: https://codingchallenges.substack.com/p/coding-challenge-3

use std::env;
use std::fs::File;

#[derive(Debug)]
#[derive(PartialEq)]
enum FuncMode {
    Encode,
    Decode,
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


fn main() {
    // accept file as input and return error if it is not valid

    // collect CLI arguments
    let args: Vec<String> = env::args().collect();

    // validate arguments
    let (_mode, source_name) = parse_args(args);

    // open the file
    let _source_file = match File::open(&source_name){
        Ok(file) => file,
        Err(err) => panic!("Unable to open the file: {:?}", err),
    };

    // determine the frequency of occurence of each character

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

#[cfg(test)]
mod tests {
    use crate::{parse_args, FuncMode};

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
}