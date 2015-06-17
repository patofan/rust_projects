
use config::configure::*;

pub struct Card {
	cardNum : u8 ,
	mark : bool,
	/**
	 * 1:明槓牌,2:暗槓牌,3: 碰牌,4:連序牌, 5:活牌 ,6:自己模進來的牌 , 7:另人打的牌
	 */
	state : u8,
}

impl Card {
	
	pub fn new(cardNum : u8 ) -> Card {
		Card{ cardNum : cardNum , mark : false , state : 6 }
	}
	 
	pub fn is_mark(&self) -> bool {
		self.mark
	}
	
	pub fn set_mark(&mut self , mark : bool ) {
		self.mark = mark;
	}
	
	pub fn get_card_num(&self ) -> u8 {
		self.cardNum
	}
	
	pub fn set_card_num(&mut self , cardNum : u8 ) {
		self.cardNum = cardNum
	}
	
//    pub fn to_string() -> &str {
//		if (CardUtility.isDollarCard(this)) {
//			return   this.cardNum   +   "萬"   +(  this.mark ? "*" : "" );
//		} else if (CardUtility.isLineCard(this)) {
//			return  (this.cardNum - 10)  + "索" +(  this.mark ? "*" : "" ) ;
//		} else if (CardUtility.isCircleCard(this)) {
//			return  (this.cardNum - 20)  + "筒" +(  this.mark ? "*" : "" ) ;
//		} else if (CardUtility.is_letterCard(this)) {
//			return  LetterTitle[this.cardNum - 31]  + (  this.mark ? "*" : "" ) ;
//		} else if (CardUtility.isFlowerCard(this)) {
//			return   FlowerTitle[this.cardNum - 41] + (  this.mark ? "*" : "" )   ;
//		}
//		return "Error Card number:" + this.cardNum;
//	}


	/**
	 * 萬子
	 * 
	 * @param cardNums
	 */
	pub fn is_dollar_card(&self )-> bool {
		return  self.is_dollar_card_num( self.get_card_num() );
	}

	pub fn  is_dollar_card_num(&self  ,cardNum : u8 ) -> bool{
		return ( cardNum >= DOLLAR_1 && cardNum <= DOLLAR_9 );
	}
	
	
	/**
	 * 索子
	 * 
	 * @param cardNum
	 * @return
	 */
	pub fn  is_line_card(&self ) -> bool{
		return self.is_line_card_num( self.cardNum );
	}
	
	pub fn  is_line_card_num( &self  , cardNum : u8 ) -> bool{
		return (cardNum >= LINE_1 && cardNum <= LINE_9);
	}
	

	/**
	 * 筒子
	 * 
	 * @param cardNum
	 * @return
	 */
	pub fn  is_circle_card(&self ) -> bool{
		return self.is_circle_card_num( self.cardNum );
	}
	
	pub fn  is_circle_card_num(&self  ,cardNum : u8 ) -> bool{
		return (cardNum >= CIRCLE_1 && cardNum <= CIRCLE_9 );
	}
	

	/**
	 * 字牌
	 * 
	 * @param cardNum
	 * @return
	 */
	pub fn  is_letter_card(&self ) -> bool{
		return self.is_letter_card_num(self.cardNum);
	}
	
	pub fn  is_letter_card_num(&self  ,cardNum : u8) -> bool{
		return (cardNum >= LETTER_EAST && cardNum <= LETTER_WHITE );
	}
	
	
	pub fn  is_letter_east_card(&self ) -> bool{
		return self.is_letter_east_card_num(self.cardNum);
	}
	
	pub fn  is_letter_east_card_num(&self  ,cardNum : u8 ) -> bool{
		return (cardNum == LETTER_EAST );
	}
	
	pub fn  is_letter_south_card(&self ) -> bool{
		return self.is_letter_south_card_num(self.cardNum);
	}
	
	pub fn  is_letter_south_card_num(&self  ,cardNum : u8) -> bool{
		return (cardNum == LETTER_SOUTH );
	}
	
	
	pub fn  is_letter_west_card(&self ) -> bool{
		return self.is_letter_west_card_num(self.cardNum);
	}
	
	pub fn  is_letter_west_card_num(&self  ,cardNum : u8) -> bool{
		return (cardNum == LETTER_WEST );
	}
	
	
	pub fn  is_letter_north_card(&self ) -> bool{
		return self.is_letter_north_card_num(self.cardNum);
	}
	
	pub fn  is_letter_north_card_num(&self  ,cardNum : u8) -> bool{
		return (cardNum == LETTER_NORTH );
	}
	
	
	//東、南、西、北、中、發、白 : 31 ~ 37        ,每一數四張。
	
	pub fn  is_letter_red_mid_card(&self ) -> bool{
		return self.is_letter_red_mid_card_num(self.cardNum);
	}
	
	pub fn  is_letter_red_mid_card_num(&self  ,cardNum : u8) -> bool{
		return (cardNum == LETTER_RED_MID );
	}
	
	
	pub fn  is_letter_fa_card(&self ) -> bool{
		return self.is_letter_fa_card_num( self.cardNum);
	}
	
	pub fn  is_letter_fa_card_num(&self  ,cardNum : u8) -> bool{
		return (cardNum == LETTER_FA );
	}
	
	
	pub fn  is_letter_white_card(&self ) -> bool{
		return self.is_letter_white_card_num(self.cardNum);
	}

	pub fn  is_letter_white_card_num(&self  ,cardNum : u8) -> bool{
		return (cardNum ==LETTER_WHITE );
	}
	
	
	
	pub fn  is_sequence_card(&self ) -> bool{
		return self.is_dollar_card() || self.is_line_card() || self.is_circle_card();
	}
	
	pub fn  is_sequence_card_num(&self  ,cardNum : u8) -> bool{
		return self.is_dollar_card_num( cardNum ) || self.is_line_card_num( cardNum ) || self.is_circle_card_num( cardNum );
	}
	
	
	/**
	 * 1 ~37
	 * 
	 * @param cardNum
	 * @return
	 */
	pub fn  is_normal_card(&self ) -> bool{
		return self.is_sequence_card( ) ||  self.is_letter_card();
	}
	
	pub fn  is_normal_card_num(&self  ,cardNum : u8 ) -> bool{
		return self.is_sequence_card_num( cardNum ) ||  self.is_letter_card_num( cardNum );
	}
	

	/**
	 * 花牌 松、蘭、竹、菊及春、夏、秋、冬八 : 41 ~ 48 ,各一張。
	 * 
	 * @param cardNum
	 * @return
	 */
	pub fn  is_flower_card(&self ) -> bool{
		return self.is_flower_card_num(self.cardNum  );
	}

	pub fn  is_flower_card_num(&self  ,cardNum : u8) -> bool{
		return (cardNum >= FLOWER_ME_1 ) &&  (cardNum <= FLOWER_WINTER_4 );
	}
	
	
	
}