
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
   
   pub fn get_allCards(&self) -> & Vec<Card> {
   	  & (self.allCards )
   }

	pub fn  clear_mark( &mut self) {
		for c in &mut self.allCards {
			c.set_mark( false) ;
		} 
	}

	pub fn  add_card_num(&mut self , cardNum : u8 ) {
		let card = Card::new(cardNum);
		self.allCards.push( card);
	}
	

	pub fn  add_card(&mut self , card : Card) {
		self.allCards.push( card );
	}

	pub fn  add_cards(&mut self , cards : &Cards) {
		for c in cards.get_all_cards() {
		   self.add_card_num( c.get_card_num() );
		}
	}

	 pub fn get_all_cards(&self) ->  &Vec<Card> {
	     return & self.allCards;
	 }


	pub fn index_of(&self , card : &Card) -> Option<usize> {
		let num = card.get_card_num();
		let mut index = 0usize;
		for c in & self.allCards {
			if c.get_card_num() == num {
				return Some( index );
			} 
			index += 1;
		}
		return None;
	}

	pub fn get_card(&self , index : usize ) -> Option<&Card> {
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
//			newCards.addCard(card.get_card_num());
//		}
//		return newCards;
//	}
	

	pub fn  clear(&mut self) {
		self.allCards.clear();
	}

	pub fn  contain_card(&self , card : &Card) -> bool {
		 self.index_of( card ).is_some() 
	}
	


	pub fn  sort(&mut self) {
		//(& (self.allCards) ).sort();
	}

	pub fn to_string(&self) -> String  {
		let mut buf = String::new();
		
		let mut index = 0;
		for card in &( self.allCards)  {
			buf.push_str(& format!("{}.{} ", index , card.get_card_num() ));
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

	pub fn  mark_pair_cards(&mut self , pairCard : &Card) {
		let mut cnt = 2;
		let pairNum = pairCard.get_card_num();
		
		for index in 0..self.allCards.len() {
			
			let  mut found =  false ;
			{
				let card : &Card = &( self.allCards[index]);
			    found = ( card.get_card_num() == pairNum && ! card.is_mark() && cnt > 0);
			}
			if found  {
				let card = &mut self.allCards[index];
				card.set_mark( false) ;
				cnt -= 1;
			}
		}
		
	}
	

	pub fn find_all_match_cards(&self , matchCards : &Cards ) -> bool{
		let mut  found = false;
		
		for index in 0..matchCards.allCards.len() {
			let card =& ( matchCards.get_all_cards()[index]);
			
			found = self.contain_card( card);
			if ! found {
				break; 
			}
		}
		return found;
	}
	
	pub fn do_get_pair_cards(&self )-> Cards  {
		return self.do_get_same_cards( 2);
	}

	pub fn do_get_three_same_cards(&self ) -> Cards  {
		return self.do_get_same_cards( 3);
	}

	pub fn do_get_same_cards(&self , sameCount : u8 ) ->  Cards {
		let mut result = Cards::new();
		let cardNums = self.get_all_cards();

		// cardNum , cnt
		//Map<Card, Byte> cardNumToCntMap = new HashMap<Card, Byte>();
		let mut cardNumToCntMap : BTreeMap<u8 , u8> = BTreeMap::new();
        //a.insert(1, "a");
		
		let mut cardNum : u8;
		let mut cnt;
		for card in  cardNums {
			cardNum = card.get_card_num();
			match cardNumToCntMap.get( &cardNum ) {
				Some( x ) => cnt = x + 1,
				None      => cnt = 1 
			}
			cardNumToCntMap.insert(cardNum, cnt);
		}

        for ( & cardNo, & cardCnt ) in cardNumToCntMap.iter() {
        	if cardCnt  >= sameCount {
        		result.add_card_num( cardNo );
        	}
        }

		
		if result.size() > 0  {
      	    result.sort();
   	    }

		return result;
	}

	pub fn  do_get_same_match_cards(&self , matchCard : &Card , sameCount : u8 ) -> Cards {
		let mut result = Cards::new();
		let cardNums = self.get_all_cards();

		let matchCardNum = matchCard.get_card_num(); 
		let mut cnt = 0;
		for card in  cardNums {
			if card.get_card_num() != matchCardNum { 
				continue;
			}	
			cnt += 1;
		}

		if cnt >= sameCount {
			result.add_card_num( matchCardNum );
		}	
		return result;
	}

	pub fn find_cards(&self , testCards : & Cards ) -> bool {
		let mut found = false;
		for  testCard in testCards.get_all_cards() {
			found = self.contain_card(&testCard);
			if !found {
				break;
			}	
		}
		return found;
	}
	
	pub fn find_card_vec(&self , testCards : &Vec<Card>  ) -> bool {
		let mut found = false;
		for  testCard in testCards {
			found = self.contain_card(testCard);
			if !found {
				break;
			}	
		}
		return found;
	}
	

	pub fn  find_two_same_cards(&self , startIndex : usize , firstCard  : &Card ) -> TwoIndexVO {
		// check same 3 card
		let mut result = TwoIndexVO::new();

		// check same 3 card
		let matchIndex1 = self.find_one_same_card(startIndex, firstCard);
		match matchIndex1 {
			Some( index1 ) => {
				let matchIndex2 = self.find_one_same_card( index1 + 1 , firstCard);
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

	pub fn find_one_same_card(&self ,  startIndex : usize , firstCard : &Card ) -> Option<usize> {

        let cards =  self.get_all_cards();
        for index in 0..cards.len() {
        	let card = &( cards[index]);
			if (! card.is_mark() && card.get_card_num() == firstCard.get_card_num() ){
				return Some(index);
			}	
        }
		return None;
	}


	pub fn remove_card(&mut self , card : &Card ) -> bool {
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
			if c.get_card_num() == card.get_card_num() {
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
				let secondCard = self.get_card(index1).unwrap();
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
		let  firstCardNum = firstCard.get_card_num();
		if ( firstCard.is_sequence_card() ) {
			for index in startIndex..self.allCards.len()  {
				let card = self.get_card(index).unwrap();
				if ( card.is_sequence_card() && ! card.is_mark() && (card.get_card_num() == firstCardNum + 1)) {
					return Some(index);
				}	
			}
		}
		return None;
	}
	
	
	pub fn remove_all_flower_cards( &mut self) -> Cards  {
		let mut idexes = vec![];
		for index in 0..self.allCards.len() {
			let c = &(self.allCards[index]);
			if c.is_flower_card() {
				idexes.push( index );
			}
		}
		
		idexes.sort_by(|a, b| b.cmp(a) );
		
        let mut result = Cards::new();        	
        for removeIdex in idexes {
        	let remove_card = self.removeCardByIndex( removeIdex );
        	result.add_card( remove_card );
        }
		return result;
	}
	
	
}


