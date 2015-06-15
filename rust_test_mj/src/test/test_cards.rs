
#[cfg(test)]
mod test_cards {

use vo::cards::*;
use vo::card::*;
use config::configure::*; 


#[test]
fn test_new() {
  	let mut cards = Cards::new();
   	cards.addCardNum( DOLLAR_1 ); 
   	assert_eq!(  cards.size()  ,  1 );
}

#[test]
fn test_new2() {
  	let mut cards = Cards::new();
   	cards.addCardNum( DOLLAR_1 ); 
   	assert_eq!(  cards.size()  ,  1 );
}

}