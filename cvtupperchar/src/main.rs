
use std::env;

use std::string::String;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::ascii::AsciiExt;

fn convert_str( s : String ) -> String {
	
	let mut target = String::new();
	for ch in s.chars() {
	  match ch {
		'A'...'Z' => {
				 target.push( '_' );
				 target.push( ch.to_ascii_lowercase( ) ); 
		}
		_ => target.push( ch )
		
		
	  }
	}
    println!( "{}" , target );
	return target;
	
}


fn convert( from_file_path : &str , to_file_path : &str ) {
	println!( "Convert upper char , from {} to {} ." , from_file_path , to_file_path );
	
	let from_path = Path::new( from_file_path );
	
	// Open the path in read-only mode, returns `io::Result<File>`
    let mut in_file = match File::open(&from_path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", from_file_path ,Error::description(&why)),
        Ok(file) => file,
    };
    


	let to_path = Path::new( to_file_path );

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut out_file = match File::create(&to_path) {
        Err(why) => panic!("couldn't create {}: {}",
                           to_file_path,
                           Error::description(&why)),
        Ok(file) => file,
    };
    
    
    let reader = BufReader::new(in_file);
    
    
    for line in reader.lines() {
        // Handle any errors that may arise
        match line {
            Ok(ln) => {  let s = convert_str( ln );   out_file.write_all( s.as_bytes() ); out_file.write_all( "\n".as_bytes() );  }
            Err(error) => print!("{}", error) ,
        }
    }
    
} 

fn help() {
	println!( "cvtupperchar from_file to_file" );
}

fn main() {

    // cvtupperchar C:\mydoc\\Cards.java   C:\mydoc\Cards_out.java
   //convert( "C:\\mydoc\\Cards.java" ,  "C:\\mydoc\\Cards_out.java" );
   // C:\Users\patofan\git\rust_projects\cvtupperchar\target\debug\cvtupperchar C:\mydoc\\Cards.java   C:\mydoc\Cards_out.java 


   let args: Vec<String> = env::args().collect();
   
//   for arg in args {
//     println!( "arg:{}" , arg );
//   }  
   
   if args.len() < 3 {
   	 help();
   } else {
   	 convert( &args[1] , &args[2] );
   }
    
}
