
use config::configure::*;

pub struct Card {
	card_num : u8 ,
	mark : bool,
	/**
	 * 1:明槓牌,2:暗槓牌,3: 碰牌,4:連序牌, 5:活牌 ,6:自己模進來的牌 , 7:另人打的牌
	 */
	state : u8,
}

impl Card {
	
	pub fn new(card_num : u8 ) -> Card {
		Card{ card_num : card_num , mark : false , state : 6 }
	}
	 
	pub fn is_mark(&self) -> bool {
		self.mark
	}
	
	pub fn set_mark(&mut self , mark : bool ) {
		self.mark = mark;
	}
	
	pub fn get_card_num(&self ) -> u8 {
		self.card_num
	}
	
	pub fn set_card_num(&mut self , card_num : u8 ) {
		self.card_num = card_num
	}
	
//    pub fn to_string() -> &str {
//		if (CardUtility.isDollarCard(this)) {
//			return   this.card_num   +   "萬"   +(  this.mark ? "*" : "" );
//		} else if (CardUtility.isLineCard(this)) {
//			return  (this.card_num - 10)  + "索" +(  this.mark ? "*" : "" ) ;
//		} else if (CardUtility.isCircleCard(this)) {
//			return  (this.card_num - 20)  + "筒" +(  this.mark ? "*" : "" ) ;
//		} else if (CardUtility.is_letterCard(this)) {
//			return  LetterTitle[this.card_num - 31]  + (  this.mark ? "*" : "" ) ;
//		} else if (CardUtility.isFlowerCard(this)) {
//			return   FlowerTitle[this.card_num - 41] + (  this.mark ? "*" : "" )   ;
//		}
//		return "Error Card number:" + this.card_num;
//	}


	/**
	 * 萬子
	 * 
	 * @param cardNums
	 */
	pub fn is_dollar_card(&self )-> bool {
		return  self.is_dollar_card_num( self.get_card_num() );
	}

	pub fn  is_dollar_card_num(&self  ,card_num : u8 ) -> bool{
		return  card_num >= DOLLAR_1 && card_num <= DOLLAR_9;
	}
	
	
	/**
	 * 索子
	 * 
	 * @param card_num
	 * @return
	 */
	pub fn  is_line_card(&self ) -> bool{
		return self.is_line_card_num( self.card_num );
	}
	
	pub fn  is_line_card_num( &self  , card_num : u8 ) -> bool{
		return card_num >= LINE_1 && card_num <= LINE_9;
	}
	

	/**
	 * 筒子
	 * 
	 * @param card_num
	 * @return
	 */
	pub fn  is_circle_card(&self ) -> bool{
		return self.is_circle_card_num( self.card_num );
	}
	
	pub fn  is_circle_card_num(&self  ,card_num : u8 ) -> bool{
		return card_num >= CIRCLE_1 && card_num <= CIRCLE_9 ;
	}
	

	/**
	 * 字牌
	 * 
	 * @param card_num
	 * @return
	 */
	pub fn  is_letter_card(&self ) -> bool{
		return self.is_letter_card_num(self.card_num);
	}
	
	pub fn  is_letter_card_num(&self  ,card_num : u8) -> bool{
		return card_num >= LETTER_EAST && card_num <= LETTER_WHITE ;
	}
	
	
	pub fn  is_letter_east_card(&self ) -> bool{
		return self.is_letter_east_card_num(self.card_num);
	}
	
	pub fn  is_letter_east_card_num(&self  ,card_num : u8 ) -> bool{
		return card_num == LETTER_EAST ;
	}
	
	pub fn  is_letter_south_card(&self ) -> bool{
		return self.is_letter_south_card_num(self.card_num);
	}
	
	pub fn  is_letter_south_card_num(&self  ,card_num : u8) -> bool{
		return card_num == LETTER_SOUTH ;
	}
	
	
	pub fn  is_letter_west_card(&self ) -> bool{
		return self.is_letter_west_card_num(self.card_num);
	}
	
	pub fn  is_letter_west_card_num(&self  ,card_num : u8) -> bool{
		return card_num == LETTER_WEST;
	}
	
	
	pub fn  is_letter_north_card(&self ) -> bool{
		return self.is_letter_north_card_num(self.card_num);
	}
	
	pub fn  is_letter_north_card_num(&self  ,card_num : u8) -> bool{
		return card_num == LETTER_NORTH ;
	}
	
	
	//東、南、西、北、中、發、白 : 31 ~ 37        ,每一數四張。
	
	pub fn  is_letter_red_mid_card(&self ) -> bool{
		return self.is_letter_red_mid_card_num(self.card_num);
	}
	
	pub fn  is_letter_red_mid_card_num(&self  ,card_num : u8) -> bool{
		return card_num == LETTER_RED_MID;
	}
	
	
	pub fn  is_letter_fa_card(&self ) -> bool{
		return self.is_letter_fa_card_num( self.card_num);
	}
	
	pub fn  is_letter_fa_card_num(&self  ,card_num : u8) -> bool{
		return card_num == LETTER_FA;
	}
	
	
	pub fn  is_letter_white_card(&self ) -> bool{
		return self.is_letter_white_card_num(self.card_num);
	}

	pub fn  is_letter_white_card_num(&self  ,card_num : u8) -> bool{
		return card_num ==LETTER_WHITE;
	}
	
	
	
	pub fn  is_sequence_card(&self ) -> bool{
		return self.is_dollar_card() || self.is_line_card() || self.is_circle_card();
	}
	
	pub fn  is_sequence_card_num(&self  ,card_num : u8) -> bool{
		return self.is_dollar_card_num( card_num ) || self.is_line_card_num( card_num ) || self.is_circle_card_num( card_num );
	}
	
	
	/**
	 * 1 ~37
	 * 
	 * @param card_num
	 * @return
	 */
	pub fn  is_normal_card(&self ) -> bool{
		return self.is_sequence_card( ) ||  self.is_letter_card();
	}
	
	pub fn  is_normal_card_num(&self  ,card_num : u8 ) -> bool{
		return self.is_sequence_card_num( card_num ) ||  self.is_letter_card_num( card_num );
	}
	

	/**
	 * 花牌 松、蘭、竹、菊及春、夏、秋、冬八 : 41 ~ 48 ,各一張。
	 * 
	 * @param card_num
	 * @return
	 */
	pub fn  is_flower_card(&self ) -> bool{
		return self.is_flower_card_num(self.card_num  );
	}

	pub fn  is_flower_card_num(&self  ,card_num : u8) -> bool{
		return (card_num >= FLOWER_ME_1 ) &&  (card_num <= FLOWER_WINTER_4 );
	}
	
	
	
}