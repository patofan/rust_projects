
use vo::cards::Cards;

pub struct CardsResult<'a> {
	 cards : &'a Cards,
	// 100 :  hu , 90 : listener ,  85 : hidden gun ,  80 : open gun ,  70 : pon , 60 :eat
	 result : u8,
}

impl <'a> CardsResult<'a> {
	
	pub fn  get_cards(&self ) -> &'a Cards{
		return self.cards;
	}

	pub fn set_cards(&mut self , cards: &'a Cards) {
		self.cards = cards;
	}

	pub fn get_result(&self ) -> u8 {
		return self.result;
	}

	pub fn set_result(&mut self , result : u8) {
		self.result = result;
	}
	
}

