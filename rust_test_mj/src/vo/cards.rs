
use vo::card::*;
use vo::two_index_vo::*;
use std::collections::BTreeMap;

pub struct Cards{
    //CARD_RESULT_GUN_CLOSE= 90 ( close ), 
	//CARD_RESULT_GUN_OPEN =  80 ( open ), 
 	//CARD_RESULT_PON =  70,
 	//CARD_RESULT_SEQ =  60,
	kind : u8 ,
	all_cards : Vec<Card> , 

}

impl Cards {
  
   pub fn new() ->Cards {
   	  Cards{  kind : 0 , all_cards : vec![] }
   }
   
   pub fn get_all_cards(&self) -> & Vec<Card> {
   	  & (self.all_cards )
   }

	pub fn  clear_mark( &mut self) {
		for c in &mut self.all_cards {
			c.set_mark( false) ;
		} 
	}

	pub fn  add_card_num(&mut self , card_num : u8 ) {
		let card = Card::new(card_num);
		self.all_cards.push( card);
	}
	

	pub fn  add_card(&mut self , card : Card) {
		self.all_cards.push( card );
	}

	pub fn  add_cards(&mut self , cards : &Cards) {
		for c in cards.get_all_cards() {
		   self.add_card_num( c.get_card_num() );
		}
	}


	pub fn index_of(&self , card : &Card) -> Option<usize> {
		let num = card.get_card_num();
		let mut index = 0usize;
		for c in & self.all_cards {
			if c.get_card_num() == num {
				return Some( index );
			} 
			index += 1;
		}
		return None;
	}

	pub fn get_card(&self , index : usize ) -> Option<&Card> {
		if index < self.size() {
		  return Some( &self.all_cards[ index ] )
		} 
	    return None;  
	}
	
	pub fn  size(&self) -> usize {
		return self.all_cards.len();
	}

//	pub fn Cards cloneAndClearMark() {
//		Cards newCards = new Cards(this.all_cards.size());
//		for (card : Card : this.all_cards) {
//			newCards.addCard(card.get_card_num());
//		}
//		return newCards;
//	}
	

	pub fn  clear(&mut self) {
		self.all_cards.clear();
	}

	pub fn  contain_card(&self , card : &Card) -> bool {
		 self.index_of( card ).is_some() 
	}
	


	pub fn  sort(&mut self) {
		//(& (self.all_cards) ).sort();
	}

	pub fn to_string(&self) -> String  {
		let mut buf = String::new();
		
		let mut index = 0;
		for card in &( self.all_cards)  {
			buf.push_str(& format!("{}.{} ", index , card.get_card_num() ));
			index += 1;
		}
		return buf;
	}

//	// pub fn Cards removePairCards(Card pair_card) {
//	// Cards newCards = cloneAndClearMark();
//	// card : Card;
//	// Iterator<Card> iter = newCards.all_cards.iterator();
//	// int cnt = 0;
//	// while (iter.hasNext()) {
//	// card = iter.next();
//	// if (card.equals( pair_card ) ) {
//	// iter.remove();
//	// cnt++;
//	// if (cnt == 2) {
//	// break;
//	// }
//	// }
//	// }
//	// return newCards;
//	// }

	pub fn  mark_pair_cards(&mut self , pair_card : &Card) {
		let mut cnt = 2;
		let pair_num = pair_card.get_card_num();
		
		for index in 0..self.all_cards.len() {
			
			let  mut found =  false ;
			{
				let card : &Card = &( self.all_cards[index]);
			    found = ( card.get_card_num() == pair_num && ! card.is_mark() && cnt > 0);
			}
			
			if found  {
				let card = &mut self.all_cards[index];
				card.set_mark( false) ;
				cnt -= 1;
			}
		}
		
	}
	

	pub fn find_all_match_cards(&self , match_cards : &Cards ) -> bool{
		let mut  found = false;
		
		for index in 0..match_cards.all_cards.len() {
			let card =& ( match_cards.get_all_cards()[index]);
			
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

	pub fn do_get_same_cards(&self , same_count : u8 ) ->  Cards {
		let mut result = Cards::new();
		let card_nums = self.get_all_cards();

		// card_num , cnt
		//Map<Card, Byte> card_num_to_cnt_map = new HashMap<Card, Byte>();
		let mut card_num_to_cnt_map : BTreeMap<u8 , u8> = BTreeMap::new();
        //a.insert(1, "a");
		
		let mut card_num : u8;
		let mut cnt;
		for card in  card_nums {
			card_num = card.get_card_num();
			match card_num_to_cnt_map.get( &card_num ) {
				Some( x ) => cnt = x + 1,
				None      => cnt = 1 
			}
			card_num_to_cnt_map.insert(card_num, cnt);
		}

        for ( & card_no, & card_cnt ) in card_num_to_cnt_map.iter() {
        	if card_cnt  >= same_count {
        		result.add_card_num( card_no );
        	}
        }

		
		if result.size() > 0  {
      	    result.sort();
   	    }

		return result;
	}

	pub fn  do_get_same_match_cards(&self , match_card : &Card , same_count : u8 ) -> Cards {
		let mut result = Cards::new();
		let card_nums = self.get_all_cards();

		let match_card_num = match_card.get_card_num(); 
		let mut cnt = 0;
		for card in  card_nums {
			if card.get_card_num() != match_card_num { 
				continue;
			}	
			cnt += 1;
		}

		if cnt >= same_count {
			result.add_card_num( match_card_num );
		}	
		return result;
	}

	pub fn find_cards(&self , test_cards : & Cards ) -> bool {
		let mut found = false;
		for  test_card in test_cards.get_all_cards() {
			found = self.contain_card(&test_card);
			if !found {
				break;
			}	
		}
		return found;
	}
	
	pub fn find_card_vec(&self , test_cards : &Vec<Card>  ) -> bool {
		let mut found = false;
		for  test_card in test_cards {
			found = self.contain_card(test_card);
			if !found {
				break;
			}	
		}
		return found;
	}
	

	pub fn  find_two_same_cards(&self , start_index : usize , first_card  : &Card ) -> TwoIndexVO {
		// check same 3 card
		let mut result = TwoIndexVO::new();

		// check same 3 card
		let match_index1 = self.find_one_same_card(start_index, first_card);
		match match_index1 {
			Some( index1 ) => {
				let match_index2 = self.find_one_same_card( index1 + 1 , first_card);
				match match_index2 {
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

	pub fn find_one_same_card(&self ,  start_index : usize , first_card : &Card ) -> Option<usize> {

        let cards =  self.get_all_cards();
        for index in 0..cards.len() {
        	let card = &( cards[index]);
			if ! card.is_mark() && card.get_card_num() == first_card.get_card_num() && index >= start_index {
				return Some(index);
			}	
        }
		return None;
	}


	pub fn remove_card(&mut self , card : &Card ) -> bool {
		let index = self.index( card );
		match index {
			Some( indexval ) => {
				self.all_cards.remove( indexval );
				return true;
			}
			None => {}
		}
	    return false;
	}

	pub fn remove_card_by_index(&mut self , index : usize )-> Card  {
		return self.all_cards.remove(index);
	}
	
	
	pub fn index( &self , card : &Card ) -> Option<usize> {
		for index in 0..self.all_cards.len() {
			let c = &(self.all_cards[index]);
			if c.get_card_num() == card.get_card_num() {
				return Some( index );
			}
		}
		return None;
	}

	pub fn find_two_next_sequence_cards(&self , start_index : usize , first_card : &Card ) -> TwoIndexVO  {
		let mut result = TwoIndexVO::new();
		let match_index1 = self.find_next_sequence_card(start_index, first_card);
		match match_index1 {
			Some( index1 ) => {
				let second_card = self.get_card(index1).unwrap();
				let match_index2 = self.find_next_sequence_card( index1 + 1 , &second_card);
				match match_index2 {
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

	pub fn find_next_sequence_card(&self , start_index : usize , first_card : &Card ) -> Option<usize> {
		// Cards test_cards = this;
		let  first_card_num = first_card.get_card_num();
		if  first_card.is_sequence_card()  {
			for index in start_index..self.all_cards.len()  {
				let card = self.get_card(index).unwrap();
				if card.is_sequence_card() && ! card.is_mark() && (card.get_card_num() == first_card_num + 1) {
					return Some(index);
				}	
			}
		}
		return None;
	}
	
	
	pub fn remove_all_flower_cards( &mut self) -> Cards  {
		let mut idexes = vec![];
		for index in 0..self.all_cards.len() {
			let c = &(self.all_cards[index]);
			if c.is_flower_card() {
				idexes.push( index );
			}
		}
		
		idexes.sort_by(|a, b| b.cmp(a) );
		
        let mut result = Cards::new();        	
        for remove_idex in idexes {
        	let remove_card = self.remove_card_by_index( remove_idex );
        	result.add_card( remove_card );
        }
		return result;
	}
	
	
}


