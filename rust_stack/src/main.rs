pub struct Stack<E> {
	vec : Vec<E> ,
}

impl <E> Stack<E> {
	fn new() -> Stack<E> {
		Stack{ vec : vec![]  }
	}
	
	fn push(&mut self ,  e : E ) {
	   self.vec.push( e );	
	}
	
	fn pop(&mut self ) -> Option<E> {
		return self.vec.pop();
	}
	
    fn peek(&self) -> Option<&E> {
        return self.vec.last();
    }
    
    fn is_empty(&self) -> bool {
    	self.vec.is_empty()
    }

}

fn main() {
	let mut s = Stack::new();
	s.push(1);
	s.push(2);
	s.push(3);
	
	{
	let t = s.peek();
	match t {
		Some(  ref x ) => println!( "peek={}" ,  x ) ,
		_ => {}
	}
	
	}
	
	
	while ! s.is_empty() {
		let e = s.pop();
		match e {
			Some( x ) => println!( "{}" ,  x ) ,
			_ => {}
		}
	}
	
	
}
