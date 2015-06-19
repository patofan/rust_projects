
use vo::cards::Cards;
use vo::card::Card;
use config::configure::*;


const RESERVER_CNT : usize =  16;

pub struct CircularSplitCards {
	vecs : Vec<Cards>,
	cnt : usize,
}
	
impl CircularSplitCards {

    pub fn new()-> 	CircularSplitCards {
    	CircularSplitCards{ vecs : vec![] , cnt :0  }
    }
	
	pub fn  put_split_cards( &mut self , cards : Cards ) {
		self.cnt += cards.size();
		self.vecs.push( cards );
	}
	
	pub fn clear(&mut self) {
		self.vecs.clear();
		self.cnt = 0;
	}
	
	
	pub fn start_put_cards( &mut self , cards : Vec<Card> ) {
		self.clear();
		
		for index in  0..cards.len() / 2 {
			let mut split_cards = Cards::new();
			for cnt in 0..2 {
				split_cards.add_card_num(  cards[index * 2 + cnt].get_card_num() );
			}
			self.put_split_cards(split_cards);
		}
		
	}
	
    pub fn do_final_reset_head_tail(&mut self , skip : usize ) {
    	let mut cards :Cards;
    	for _ in 0..skip {
    		cards = self.vecs.remove(0); 
    		self.put_split_cards( cards );
    	}
	}
	
	
	pub fn do_final_reset_head_tail_by_wnd_skip(&mut self , wnd :u8 ,  skip : usize) {
		let mut total_skip = 0;
		if wnd == LETTER_EAST {
			total_skip = skip; 
		} else if wnd == LETTER_SOUTH  {
			total_skip =  18 * 3 +  skip;
		}else if wnd == LETTER_WEST {
			total_skip =  18 * 2 +  skip;
		}else if wnd == LETTER_NORTH  {
			total_skip =  18 * 1 +  skip;
		}
		self.do_final_reset_head_tail(total_skip); 
	}	
	
	pub fn pull_card_from_head(&mut self , card_count : usize ) -> Cards {
		let mut result = Cards::new();
		
		
		for _ in 0..card_count {
			if self.cnt > RESERVER_CNT {   
			  let mut split_cards = &mut self.vecs[0];
			  let card = split_cards.remove_card_by_index(0);
			  result.add_card( card );
			  self.cnt -= 1; 
			}
		}
		return result;
	}
	
	pub fn pull_one_card_from_head(&mut self )-> Option<Card> {
		let mut cards = self.pull_card_from_head( 1 );
		if cards.size() > 0 {
			return Some( cards.remove_card_by_index(0) );
		}
		return None;
	}
	
	
	pub fn  pull_card_from_tail(&mut self , card_count : usize ) -> Cards {
		let mut result = Cards::new();
		
		for _ in 0..card_count {
			if self.cnt > 0{
			  let index = self.vecs.len() -1;
			  let mut split_cards = &mut self.vecs[index];
			  let card = split_cards.remove_card_by_index(0);
			  result.add_card( card );
			  self.cnt -= 1;
			} 
		}
		return result;
	}
	
	pub fn pull_one_card_from_tail(&mut self ) -> Option<Card>  {
		let mut cards = self.pull_card_from_tail( 1 );
		if cards.size() > 0 {
			return Some( cards.remove_card_by_index(0) );
		}
		return None;
	}
	
}