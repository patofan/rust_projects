
use config::configure;
use vo::cards::Cards;
use vo::card::Card;


pub struct CardUtility;

//牌
//萬子 1萬~9萬 : 1 ~ 9                         ,每一數四張。
//索子 1索~9索 : 11 ~ 19                        ,每一數四張。
//筒子 1筒~9筒 : 21 ~ 29                       ,每一數四張。
//字牌 東、南、西、北、中、發、白 : 31 ~ 37        ,每一數四張。
//花牌 梅、蘭、竹、菊及春、夏、秋、冬八 : 41 ~ 48   ,各一張。
impl CardUtility {

	pub fn makAllDollarCards( result : &mut Cards  ){
		for  num in 1..10 {
			result.addCard( CardUtility::makeDollarCard( num )  );
		}
	}
	
	pub fn makAllLineCards( result : &mut Cards ){
		for  num in 1..10 {
			result.addCard( CardUtility::makeLineCard( num )  );
		}
	}
	
	pub fn makAllCircleCards( result : &mut Cards ){
		for  num in 1..10 {
			result.addCard( CardUtility::makeCircleCard( num )  );
		}
	}
	
//	
//	fn makeEastCard() -> Card {
//		Card::new(  LETTER_EAST  );
//	}
//	
//	fn Card makeSouthCard()-> Card{
//		return new Card( CardConfigure.LETTER_SOUTH  );
//	}
//	
//	fn Card makeWestCard()-> Card{
//		return new Card(  CardConfigure.LETTER_WEST  );
//	}
//	
//	fn Card makeNorthCard()-> Card{
//		return new Card( CardConfigure.LETTER_NORTH );
//	}
//	
//	fn Card makeRedMidCard()-> Card{
//		return new Card( CardConfigure.LETTER_RED_MID  );
//	}
//	
//	fn Card makeFaCard()-> Card{
//		return new Card( CardConfigure.LETTER_FA  );
//	}
//	
//	fn Card makeWhiteCard()-> Card{
//		return new Card( CardConfigure.LETTER_WHITE  );
//	}
//	
	pub fn  makeDollarCard( dollarNum : u8 )-> Card{
		if( dollarNum > 9 ){
			panic!( "Make line card must be <= 9");
		}	
		return Card::new( dollarNum  );
	}
	
	
	pub fn  makeLineCard(  lineNum: u8 )-> Card{
		if( lineNum > 9 ) {
			panic!( "Make line card must be <= 9");
		}	
		return Card::new(  10 +   lineNum  );
	}
	
	pub fn  makeCircleCard( circleNum: u8 )-> Card{
		if( circleNum > 9 ) {
			panic!( "Make circle card must be <= 9");
		}	
		return Card::new( 20 +   circleNum );
	}
//	
//
//	/**
//	 * 萬子
//	 * 
//	 * @param cardNums
//	 */
//	fn boolean isDollarCard(Card card )-> bool {
//		byte cardNum = card.getCardNum();
//		return  isDollarCard(cardNum );
//	}
//
//	fn boolean isDollarCard(byte cardNum ) -> bool{
//		return (cardNum >= CardConfigure.DOLLAR_1 && cardNum <= CardConfigure.DOLLAR_9 );
//	}
//	
//	
//	/**
//	 * 索子
//	 * 
//	 * @param cardNum
//	 * @return
//	 */
//	fn boolean isLineCard(Card card ) -> bool{
//		byte cardNum = card.getCardNum();
//		return isLineCard( cardNum );
//	}
//	
//	fn boolean isLineCard( byte cardNum ) -> bool{
//		return (cardNum >= CardConfigure.LINE_1 && cardNum <= CardConfigure.LINE_9);
//	}
//	
//
//	/**
//	 * 筒子
//	 * 
//	 * @param cardNum
//	 * @return
//	 */
//	fn boolean isCircleCard(Card card ) -> bool{
//		byte cardNum = card.getCardNum();
//		return isCircleCard(cardNum);
//	}
//	
//	fn boolean isCircleCard(byte cardNum ) -> bool{
//		return (cardNum >= CardConfigure.CIRCLE_1 && cardNum <= CardConfigure.CIRCLE_9 );
//	}
//	
//
//	/**
//	 * 字牌
//	 * 
//	 * @param cardNum
//	 * @return
//	 */
//	fn boolean isLetterCard(Card card ) -> bool{
//		byte cardNum = card.getCardNum();
//		return isLetterCard(cardNum);
//	}
//	
//	fn boolean isLetterCard(byte cardNum) -> bool{
//		return (cardNum >= CardConfigure.LETTER_EAST && cardNum <= CardConfigure.LETTER_WHITE );
//	}
//	
//	
//	fn boolean isLetterEastCard(Card card ) -> bool{
//		byte cardNum = card.getCardNum();
//		return isLetterEastCard(cardNum);
//	}
//	
//	fn boolean isLetterEastCard(byte cardNum ) -> bool{
//		return (cardNum == CardConfigure.LETTER_EAST );
//	}
//	
//	fn boolean isLetterSouthCard(Card card ) -> bool{
//		byte cardNum = card.getCardNum();
//		return isLetterSouthCard(cardNum);
//	}
//	
//	fn boolean isLetterSouthCard(byte cardNum) -> bool{
//		return (cardNum == CardConfigure.LETTER_SOUTH );
//	}
//	
//	
//	fn boolean isLetterWestCard(Card card ) -> bool{
//		byte cardNum = card.getCardNum();
//		return isLetterWestCard(cardNum);
//	}
//	
//	fn boolean isLetterWestCard(byte cardNum) -> bool{
//		return (cardNum == CardConfigure.LETTER_WEST );
//	}
//	
//	
//	fn boolean isLetterNorthCard(Card card ) -> bool{
//		byte cardNum = card.getCardNum();
//		return isLetterNorthCard(cardNum);
//	}
//	
//	fn boolean isLetterNorthCard(byte cardNum) -> bool{
//		return (cardNum == CardConfigure.LETTER_NORTH );
//	}
//	
//	
//	//東、南、西、北、中、發、白 : 31 ~ 37        ,每一數四張。
//	
//	fn boolean isLetterRedMidCard(Card card ) -> bool{
//		byte cardNum = card.getCardNum();
//		return isLetterRedMidCard(cardNum);
//	}
//	
//	fn boolean isLetterRedMidCard(byte cardNum) -> bool{
//		return (cardNum == CardConfigure.LETTER_RED_MID );
//	}
//	
//	
//	fn boolean isLetterFaCard(Card card ) -> bool{
//		byte cardNum = card.getCardNum();
//		return isLetterFaCard( cardNum);
//	}
//	
//	fn boolean isLetterFaCard(byte cardNum) -> bool{
//		return (cardNum == CardConfigure.LETTER_FA );
//	}
//	
//	
//	fn boolean isLetterWhiteCard(Card card ) -> bool{
//		byte cardNum = card.getCardNum();
//		return isLetterWhiteCard(cardNum);
//	}
//
//	fn boolean isLetterWhiteCard(byte cardNum) -> bool{
//		return (cardNum ==CardConfigure.LETTER_WHITE );
//	}
//	
//	
//	
//	fn boolean isSequenceCard(Card card ) -> bool{
//		return isDollarCard( card ) || isLineCard( card ) || isCircleCard( card );
//	}
//	
//	fn boolean isSequenceCard(byte cardNum) -> bool{
//		return isDollarCard( cardNum ) || isLineCard( cardNum ) || isCircleCard( cardNum );
//	}
//	
//	
//	/**
//	 * 1 ~37
//	 * 
//	 * @param cardNum
//	 * @return
//	 */
//	fn boolean isNormalCard(Card card ) -> bool{
//		return isSequenceCard( card ) ||  isLetterCard( card );
//	}
//	
//	fn boolean isNormalCard(byte cardNum ) -> bool{
//		return isSequenceCard( cardNum ) ||  isLetterCard( cardNum );
//	}
//	
//
//	/**
//	 * 花牌 松、蘭、竹、菊及春、夏、秋、冬八 : 41 ~ 48 ,各一張。
//	 * 
//	 * @param cardNum
//	 * @return
//	 */
//	fn boolean isFlowerCard(Card card ) -> bool{
//		byte cardNum = card.getCardNum();
//		return isFlowerCard( cardNum );
//	}
//
//	fn boolean isFlowerCard(byte cardNum) -> bool{
//		return (cardNum >= CardConfigure.FLOWER_ME_1 ) &&  (cardNum <= CardConfigure.FLOWER_WINTER_4 );
//	}
	
	
}