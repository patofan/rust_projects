pub struct TwoIndexVO {
    pub index1 : usize,
	pub index2 : usize,
	pub found  : bool,	
	
}

impl TwoIndexVO {
	pub fn new() -> TwoIndexVO {
		TwoIndexVO{ index1 : 0 , index2 : 0 , found : false }
	}
}