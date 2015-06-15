
use vo::card::*;
use vo::two_index_vo::*;
use std::collections::BTreeMap;

pub struct Cards{
    //CARD_RESULT_GUN_CLOSE= 90 ( close ), 
	//CARD_RESULT_GUN_OPEN =  80 ( open ), 
 	//CARD_RESULT_PON =  70,
 	//CARD_RESULT_SEQ =  60,
	kind : u8 ,
	allCards : Vec<Card> , 

}

impl Cards {
  
   pub fn new() ->Cards {
   	  Cards{  kind : 0 , allCards : vec![] }
   }
   
   pub fn getAllCards(&self) -> & Vec<Card> {
   	  & (self.allCards )
   }

	pub fn  clearMark( &mut self) {
		for c in &mut self.allCards {
			c.setMark( false) ;
		} 
	}

	pub fn  addCardNum(&mut self , cardNum : u8 ) {
		let card = Card::new(cardNum);
		self.allCards.push( card);
	}
	

	pub fn  addCard(&mut self , card : Card) {
		self.allCards.push( card );
	}

	pub fn  addCards(&mut self , cards : &Cards) {
		for c in cards.getAllCards() {
		   self.addCardNum( c.getCardNum() );
		}
	}

//	// pub fn List<Card> getAllCards() {
//	// return allCards;
//	// }
//

	pub fn indexOf(&self , card : &Card) -> Option<usize> {
		let num = card.getCardNum();
		let mut index = 0usize;
		for c in & self.allCards {
			if c.getCardNum() == num {
				return Some( index );
			} 
			index += 1;
		}
		return None;
	}

	pub fn getCard(&self , index : usize ) -> Option<&Card> {
		if index < self.size() {
		  return Some( &self.allCards[ index ] )
		} 
	    return None;  
	}
	
	pub fn  size(&self) -> usize {
		return self.allCards.len();
	}

//	pub fn Cards cloneAndClearMark() {
//		Cards newCards = new Cards(this.allCards.size());
//		for (card : Card : this.allCards) {
//			newCards.addCard(card.getCardNum());
//		}
//		return newCards;
//	}
	

	pub fn  clear(&mut self) {
		self.allCards.clear();
	}

	pub fn  containCard(&self , card : &Card) -> bool {
		 self.indexOf( card ).is_some() 
	}
	


	pub fn  sort(&mut self) {
	}

	pub fn toString(&self) -> String  {
		let mut buf = String::new();
		
		let mut index = 0;
		for card in &( self.allCards)  {
			buf.push_str(& format!("{}.{} ", index , card.getCardNum() ));
			index += 1;
		}
		return buf;
	}

//	// pub fn Cards removePairCards(Card pairCard) {
//	// Cards newCards = cloneAndClearMark();
//	// card : Card;
//	// Iterator<Card> iter = newCards.allCards.iterator();
//	// int cnt = 0;
//	// while (iter.hasNext()) {
//	// card = iter.next();
//	// if (card.equals( pairCard ) ) {
//	// iter.remove();
//	// cnt++;
//	// if (cnt == 2) {
//	// break;
//	// }
//	// }
//	// }
//	// return newCards;
//	// }

	pub fn  markPairCards(&mut self , pairCard : &Card) {
		let mut cnt = 2;
		let pairNum = pairCard.getCardNum();
		
		for index in 0..self.allCards.len() {
			
			let  mut found =  false ;
			{
				let card : &Card = &( self.allCards[index]);
			    found = ( card.getCardNum() == pairNum && ! card.isMark() && cnt > 0);
			}
			if found  {
				let card = &mut self.allCards[index];
				card.setMark( false) ;
				cnt -= 1;
			}
		}
		
	}
	

	pub fn findAllMatchCards(&self , matchCards : &Cards ) -> bool{
		let mut  found = false;
		
		for index in 0..matchCards.allCards.len() {
			let card =& ( matchCards.getAllCards()[index]);
			
			found = self.containCard( card);
			if ! found {
				break; 
			}
		}
		return found;
	}
	
	pub fn doGetPairCards(&self )-> Cards  {
		return self.doGetSameCards( 2);
	}

	pub fn doGetThreeSameCards(&self ) -> Cards  {
		return self.doGetSameCards( 3);
	}

	pub fn doGetSameCards(&self , sameCount : u8 ) ->  Cards {
		let mut result = Cards::new();
		let cardNums = self.getAllCards();

		// cardNum , cnt
		//Map<Card, Byte> cardNumToCntMap = new HashMap<Card, Byte>();
		let mut cardNumToCntMap : BTreeMap<u8 , u8> = BTreeMap::new();
        //a.insert(1, "a");
		
		let mut cardNum : u8;
		let mut cnt;
		for card in  cardNums {
			cardNum = card.getCardNum();
			match cardNumToCntMap.get( &cardNum ) {
				Some( x ) => cnt = x + 1,
				None      => cnt = 1 
			}
			cardNumToCntMap.insert(cardNum, cnt);
		}

        for ( & cardNo, & cardCnt ) in cardNumToCntMap.iter() {
        	if cardCnt  >= sameCount {
        		result.addCardNum( cardNo );
        	}
        }

		
		if result.size() > 0  {
      	    result.sort();
   	    }

		return result;
	}

	pub fn  doGetSameMatchCards(&self , matchCard : &Card , sameCount : u8 ) -> Cards {
		let mut result = Cards::new();
		let cardNums = self.getAllCards();

		let matchCardNum = matchCard.getCardNum(); 
		let mut cnt = 0;
		for card in  cardNums {
			if card.getCardNum() != matchCardNum { 
				continue;
			}	
			cnt += 1;
		}

		if cnt >= sameCount {
			result.addCardNum( matchCardNum );
		}	
		return result;
	}

	pub fn findCards(&self , testCards : & Cards ) -> bool {
		let mut found = false;
		for  testCard in testCards.getAllCards() {
			found = self.containCard(testCard);
			if !found {
				break;
			}	
		}
		return found;
	}

	pub fn  findTwoSameCards(&self , startIndex : usize , firstCard  : &Card ) -> TwoIndexVO {
		// check same 3 card
		let mut result = TwoIndexVO::new();

		// check same 3 card
		let matchIndex1 = self.findOneSameCard(startIndex, firstCard);
		match matchIndex1 {
			Some( index1 ) => {
				let matchIndex2 = self.findOneSameCard( index1 + 1 , firstCard);
				match matchIndex2 {
					Some( index2 ) => {
						result.found = true;
						result.index1 = index1;
						result.index2 = index2;
					}
					None => {}
				}
				
			} 
			None => {}
		}
		return result;
	}

	pub fn findOneSameCard(&self ,  startIndex : usize , firstCard : &Card ) -> Option<usize> {

        let cards =  self.getAllCards();
        for index in 0..cards.len() {
        	let card = &( cards[index]);
			if (! card.isMark() && card.getCardNum() == firstCard.getCardNum() ){
				return Some(index);
			}	
        }
		return None;
	}


	pub fn removeCard(&mut self , card : &Card ) -> bool {
		let index = self.index( card );
		match index {
			Some( indexval ) => {
				self.allCards.remove( indexval );
				return true;
			}
			None => {}
		}
	    return false;
	}

	pub fn removeCardByIndex(&mut self , index : usize )-> Card  {
		return self.allCards.remove(index);
	}
	
	
	pub fn index( &self , card : &Card ) -> Option<usize> {
		for index in 0..self.allCards.len() {
			let c = &(self.allCards[index]);
			if c.getCardNum() == card.getCardNum() {
				return Some( index );
			}
		}
		return None;
	}

	pub fn findTwoNextSequenceCards(&self , startIndex : usize , firstCard : &Card ) -> TwoIndexVO  {
		let mut result = TwoIndexVO::new();
		let matchIndex1 = self.findNextSequenceCard(startIndex, firstCard);
		match matchIndex1 {
			Some( index1 ) => {
				let secondCard = self.getCard(index1).unwrap();
				let matchIndex2 = self.findNextSequenceCard( index1 + 1 , &secondCard);
				match matchIndex2 {
					Some( index2 ) => {
						result.found = true; 
						result.index1 = index1;
						result.index2 = index2;
					}
					None => {}
				}
				
			}
			None => {} 
		}
		return result;
	}

	pub fn findNextSequenceCard(&self , startIndex : usize , firstCard : &Card ) -> Option<usize> {
		// Cards testCards = this;
		let  firstCardNum = firstCard.getCardNum();
		if ( firstCard.isSequenceCard() ) {
			for index in startIndex..self.allCards.len()  {
				let card = self.getCard(index).unwrap();
				if ( card.isSequenceCard() && ! card.isMark() && (card.getCardNum() == firstCardNum + 1)) {
					return Some(index);
				}	
			}
		}
		return None;
	}
	

//	fn indexCard( v : & Vec<Card> , card : &Card ) -> Option<usize> {
//		
//		for index in 0..v.len() {
//			let c = & (v[index]);
//			if c.getCardNum() == card.getCardNum() {
//				return Some( index );
//			}
//		}
//		return None;
//	}
	

	pub fn removeAllFlowerCards( &mut self) -> Cards  {
		let mut idexes = vec![];
		for index in 0..self.allCards.len() {
			let c = &(self.allCards[index]);
			if c.isFlowerCard() {
				idexes.push( index );
			}
		}

        let mut result = Cards::new();        	
        for removeIdex in idexes {
        	let removeCard = self.removeCardByIndex( removeIdex );
        	result.addCard( removeCard );
        }
		return result;
	}
	
}


