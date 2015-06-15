
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
	 
	pub fn isMark(&self) -> bool {
		self.mark
	}
	
	pub fn setMark(&mut self , mark : bool ) {
		self.mark = mark;
	}
	
	pub fn getCardNum(&self ) -> u8 {
		self.cardNum
	}
	
	pub fn setCardNum(&mut self , cardNum : u8 ) {
		self.cardNum = cardNum
	}
	
//    pub fn to_string() -> &str {
//		if (CardUtility.isDollarCard(this)) {
//			return   this.cardNum   +   "萬"   +(  this.mark ? "*" : "" );
//		} else if (CardUtility.isLineCard(this)) {
//			return  (this.cardNum - 10)  + "索" +(  this.mark ? "*" : "" ) ;
//		} else if (CardUtility.isCircleCard(this)) {
//			return  (this.cardNum - 20)  + "筒" +(  this.mark ? "*" : "" ) ;
//		} else if (CardUtility.isLetterCard(this)) {
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
	pub fn isDollarCard(&self )-> bool {
		return  self.isDollarCardNum( self.getCardNum() );
	}

	pub fn  isDollarCardNum(&self  ,cardNum : u8 ) -> bool{
		return ( cardNum >= DOLLAR_1 && cardNum <= DOLLAR_9 );
	}
	
	
	/**
	 * 索子
	 * 
	 * @param cardNum
	 * @return
	 */
	pub fn  isLineCard(&self ) -> bool{
		return self.isLineCardNum( self.cardNum );
	}
	
	pub fn  isLineCardNum( &self  , cardNum : u8 ) -> bool{
		return (cardNum >= LINE_1 && cardNum <= LINE_9);
	}
	

	/**
	 * 筒子
	 * 
	 * @param cardNum
	 * @return
	 */
	pub fn  isCircleCard(&self ) -> bool{
		return self.isCircleCardNum( self.cardNum );
	}
	
	pub fn  isCircleCardNum(&self  ,cardNum : u8 ) -> bool{
		return (cardNum >= CIRCLE_1 && cardNum <= CIRCLE_9 );
	}
	

	/**
	 * 字牌
	 * 
	 * @param cardNum
	 * @return
	 */
	pub fn  isLetterCard(&self ) -> bool{
		return self.isLetterCardNum(self.cardNum);
	}
	
	pub fn  isLetterCardNum(&self  ,cardNum : u8) -> bool{
		return (cardNum >= LETTER_EAST && cardNum <= LETTER_WHITE );
	}
	
	
	pub fn  isLetterEastCard(&self ) -> bool{
		return self.isLetterEastCardNum(self.cardNum);
	}
	
	pub fn  isLetterEastCardNum(&self  ,cardNum : u8 ) -> bool{
		return (cardNum == LETTER_EAST );
	}
	
	pub fn  isLetterSouthCard(&self ) -> bool{
		return self.isLetterSouthCardNum(self.cardNum);
	}
	
	pub fn  isLetterSouthCardNum(&self  ,cardNum : u8) -> bool{
		return (cardNum == LETTER_SOUTH );
	}
	
	
	pub fn  isLetterWestCard(&self ) -> bool{
		return self.isLetterWestCardNum(self.cardNum);
	}
	
	pub fn  isLetterWestCardNum(&self  ,cardNum : u8) -> bool{
		return (cardNum == LETTER_WEST );
	}
	
	
	pub fn  isLetterNorthCard(&self ) -> bool{
		return self.isLetterNorthCardNum(self.cardNum);
	}
	
	pub fn  isLetterNorthCardNum(&self  ,cardNum : u8) -> bool{
		return (cardNum == LETTER_NORTH );
	}
	
	
	//東、南、西、北、中、發、白 : 31 ~ 37        ,每一數四張。
	
	pub fn  isLetterRedMidCard(&self ) -> bool{
		return self.isLetterRedMidCardNum(self.cardNum);
	}
	
	pub fn  isLetterRedMidCardNum(&self  ,cardNum : u8) -> bool{
		return (cardNum == LETTER_RED_MID );
	}
	
	
	pub fn  isLetterFaCard(&self ) -> bool{
		return self.isLetterFaCardNum( self.cardNum);
	}
	
	pub fn  isLetterFaCardNum(&self  ,cardNum : u8) -> bool{
		return (cardNum == LETTER_FA );
	}
	
	
	pub fn  isLetterWhiteCard(&self ) -> bool{
		return self.isLetterWhiteCardNum(self.cardNum);
	}

	pub fn  isLetterWhiteCardNum(&self  ,cardNum : u8) -> bool{
		return (cardNum ==LETTER_WHITE );
	}
	
	
	
	pub fn  isSequenceCard(&self ) -> bool{
		return self.isDollarCard() || self.isLineCard() || self.isCircleCard();
	}
	
	pub fn  isSequenceCardNum(&self  ,cardNum : u8) -> bool{
		return self.isDollarCardNum( cardNum ) || self.isLineCardNum( cardNum ) || self.isCircleCardNum( cardNum );
	}
	
	
	/**
	 * 1 ~37
	 * 
	 * @param cardNum
	 * @return
	 */
	pub fn  isNormalCard(&self ) -> bool{
		return self.isSequenceCard( ) ||  self.isLetterCard();
	}
	
	pub fn  isNormalCardNum(&self  ,cardNum : u8 ) -> bool{
		return self.isSequenceCardNum( cardNum ) ||  self.isLetterCardNum( cardNum );
	}
	

	/**
	 * 花牌 松、蘭、竹、菊及春、夏、秋、冬八 : 41 ~ 48 ,各一張。
	 * 
	 * @param cardNum
	 * @return
	 */
	pub fn  isFlowerCard(&self ) -> bool{
		return self.isFlowerCardNum(self.cardNum  );
	}

	pub fn  isFlowerCardNum(&self  ,cardNum : u8) -> bool{
		return (cardNum >= FLOWER_ME_1 ) &&  (cardNum <= FLOWER_WINTER_4 );
	}
	
	
	
}