//
//struct Node {
//	left  :   Option< Box<Node>>,
//    right  : Option<Box<Node>>,
//    val :  i32,
//}
//
//impl  Node  {
//	fn new( key : i32 ) -> Node {
//		Node{  left : None , right : None , val : key }
//	}
//	
//	fn insert( &mut self,  new_val: i32 ) {
//	 	
//        if self.val == new_val {
//            return;
//        }
//        
//        let target_node = if new_val < self.val { &mut self.left } else { &mut self.right };
//        
//        match target_node {
//            &mut Some(ref mut subnode) => subnode.insert(new_val),
//            &mut None => {
//                let new_node = Node::new(  new_val );
//                let boxed_node = Some(Box::new(new_node));
//                *target_node = boxed_node;
//            }
//        }
//    }
//	
//	
////	fn insert2( &mut self,  bnode :  Box<Node> ) {
////	 	
////        if self.val == bnode.val  {
////            return;
////        }
////        
////        let target_node = if bnode.val < self.val { &mut self.left } else { &mut self.right };
////        
////        match target_node {
////            &mut Some(ref mut subnode) => subnode.insert2(  bnode  ),
////            &mut None => {
////                let some_node = Some( bnode );
////                *target_node = some_node;
////            }
////        }
////    }
//	
//	
//	fn print( &self ) {
//		
//		let  lnode = & self.left;
//		
//		match lnode {
//			& Some( ref node )  => { node.print(); }
//			& None => {}
//		} 
//		println!( "{}" , self.val );
//		
//		let rnode = & self.right;
//		match rnode  {
//			& Some( ref node )  => { node.print(); }
//			& None => {}
//		} 
//		
//	}
//
//}
 

// mod mystd;
extern crate treenode;

use treenode::mystd::node::Node; 

fn main() {
    println!("Hello, world!");
    let mut root =  Node::new( 30 );
    root.insert(  20 );
    root.insert(  12 );
    root.insert(  40 );
    root.insert(  12 );
    
    root.print();
    
    //println!( "rootval={}" , root.val );
    //println!( "root.left.val={}" , root.left.unwrap().val );
    
}
