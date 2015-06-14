
use std::string::String;

struct StrToken<'a>  {
   s : &'a str,
   delemeters :  &'a str,
   cur_pos   : usize,
   skip_delemeters :bool,
   len : usize,
}


impl <'a> StrToken <'a> {

  fn new( source : &'a str , deles : &'a str , skip_delemeters :bool ) -> StrToken<'a> {
     StrToken{ s : source , delemeters : deles , skip_delemeters : skip_delemeters , cur_pos : 0 , len : source.len() }
  }
  
  fn is_dele_char(&self , ch : char ) -> bool {
     match self.delemeters.find( ch ) {
     	Some(_) => { return true; }
     	None => { return false;  }
     }
  }
  
  /**
  *  state = 0 , end
  *  state = 1 , normal char
  *  state = 2 , delemeter char
  */
  fn next_char( &mut self , state : &mut i8 ) -> char {
  	let mut ch : char = '\0';
  	*state = 0;
  	
  	if self.cur_pos < self.len {
   	  let ch_option =  self.s.chars().nth(  self.cur_pos );
      match  ch_option { 
        Some(x ) =>  { 
        	 ch = x;
        	 if  self.is_dele_char( ch ) {
        	 	*state = 2;
        	 } else {
        	 	*state = 1;
        	 } 
       	 }
        None => {}
      }
      self.cur_pos += 1;  
  	}
  	//println!( "valid={} , ch={}" , valid , ch );
  	return ch;
  }
  
  
  fn scan(&mut self , result : &mut String ) {
    let mut ch : char;
    let mut state : i8 = 0;
    ch = self.next_char( &mut state );
    match state {
    	1 => {
    		 // scan normal char;
    		 result.push( ch );
    		 ch = self.next_char( &mut state );
    		 while state == 1  {
    		 	result.push( ch );
    		 	ch = self.next_char( &mut state );
    		 }
    		 
    		 if ! self.skip_delemeters && state == 2 {
    		 	self.cur_pos -= 1;
    		 }
    		 
    		}
    	2 => {
    		  // delemeters char
    		  if ! self.skip_delemeters {
    		  	result.push( ch );
    		  }
    		}
    	_ => {}
    } 	 
  }
  
  
  fn next_token(&mut self) -> String {
     let mut s1 = String::new();
     if self.cur_pos < self.len  {
        self.scan( &mut s1 );	
     }
     return s1;
  }
  
  fn has_more_token(&self) -> bool {
     //println!( "cur_pos={}" , self.cur_pos );
     //println!( "len={}" , self.len );
  	
  	 return self.cur_pos < self.len;  
  }
   
}


fn main() {
   let s = "aa,,bb,cc;dd,ee"; 
   let dels = ",;";
   
   let mut stk = StrToken::new( s , dels , true );
   
//   println!( "{}" , stk.s );
//   println!( "{}" , stk.delemeters );
//   println!( "{}" , stk.skip_delemeters );
//   println!( "{}" , stk.cur_pos );
    let mut strtoken : String;  
    while stk.has_more_token() {
   	   strtoken = stk.next_token();
       println!( "token={}" , strtoken );
    } 


 
}