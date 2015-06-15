
#[cfg(test)]
mod test_cards {

use vo::cards::*;
use vo::card::*;
use config::configure::*;
use utility::card_utility::*; 


	fn setUp() -> Cards {
		let mut cards = Cards::new();
		cards.addCard( CardUtility::makeDollarCard( 1 ) );
		cards.addCard( CardUtility::makeDollarCard( 2 ) );
		cards.addCard( CardUtility::makeDollarCard( 3 ) );
		
		cards.addCard( CardUtility::makeDollarCard(  1 ) );
		cards.addCard( CardUtility::makeDollarCard(  2 ) );
		cards.addCard( CardUtility::makeDollarCard(  3 ) );

		cards.addCard( CardUtility::makeLineCard(  1 ) );
		cards.addCard( CardUtility::makeLineCard(  2 ) );
		cards.addCard( CardUtility::makeLineCard(  3 ) );
		
		cards.addCard( CardUtility::makeLineCard(  1 ) );
		cards.addCard( CardUtility::makeLineCard(  2 ) );
		cards.addCard( CardUtility::makeLineCard(  3 ) );
		
		cards.addCard( CardUtility::makeDollarCard(   5) );
		cards.addCard( CardUtility::makeDollarCard(   5) );
		cards.addCard( CardUtility::makeDollarCard(   5) );
		cards.addCard( CardUtility::makeDollarCard(   6) );
		cards.addCard( CardUtility::makeDollarCard(   4) );
		return cards;
	}



#[test]
fn test_new() {
  	let mut cards = Cards::new();
   	cards.addCardNum( DOLLAR_1 ); 
   	assert_eq!(  cards.size()  ,  1 );
}

#[test]
fn test_addCard() {
  	let mut cards = Cards::new();
  	let card = Card::new( DOLLAR_2 );
  	cards.addCard( card );
   	assert_eq!(  cards.size()  ,  1 );
}

#[test]
fn test_FindAllMatchCards() {
	    let cards = setUp();
	    
	    let mut testCards = Cards::new();
		testCards.addCard( CardUtility::makeDollarCard(   5) );
		testCards.addCard( CardUtility::makeDollarCard(   5) );
		testCards.addCard( CardUtility::makeDollarCard(   5) );
		testCards.addCard( CardUtility::makeDollarCard(   6) );
		testCards.addCard( CardUtility::makeDollarCard(   4) );
		assert!( cards.findAllMatchCards( &testCards  ) );
		
		testCards = Cards::new();
		testCards.addCard( CardUtility::makeDollarCard(   5) );
		testCards.addCard( CardUtility::makeDollarCard(   5) );
		testCards.addCard( CardUtility::makeDollarCard(   5) );
		testCards.addCard( CardUtility::makeDollarCard(   6) );
		testCards.addCard( CardUtility::makeDollarCard(   7) );
		assert!( ! cards.findAllMatchCards( &testCards  ) );
	}


#[test]
fn test_DoGetPairCards() {
	    let cards = setUp();
		let testCards =  cards.doGetPairCards();
		
		let mut target = Cards::new();
		target.addCard(  CardUtility::makeDollarCard(   1) );
		target.addCard(  CardUtility::makeDollarCard(   2) );
		target.addCard(  CardUtility::makeDollarCard(   3) );
		
		target.addCard(  CardUtility::makeLineCard(   1) );
		target.addCard(  CardUtility::makeLineCard(   2) );
		target.addCard(  CardUtility::makeLineCard(   3) );
		
		target.addCard(  CardUtility::makeDollarCard(   5) );
		
		assert!( testCards.findCards( &target ) );
}

#[test]
fn test_DoGetPairCards2() {
	    let cards = setUp();
		let testCards =  cards.doGetPairCards();
		
		let target = vec![
		  CardUtility::makeDollarCard(   1) ,
		  CardUtility::makeDollarCard(   2) ,
		  CardUtility::makeDollarCard(   3) ,
		  CardUtility::makeLineCard(   1) ,
		  CardUtility::makeLineCard(   2) ,
		  CardUtility::makeLineCard(   3) ,
		  CardUtility::makeDollarCard(   5) 
	    ];
		
		assert!( testCards.findCardVec( &target ) );
}





}