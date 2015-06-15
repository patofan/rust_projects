

 pub enum RecordState {
    RecordNoState,
    RecordUpdateState,
    RecordInsertState,
    RecordDeleteState,
}
 
 
 pub trait RecordHolder<T> {
 /**
     * Get status flag
     * @return RecordStatus
     */
	fn get_record_status(self) -> RecordState;
	
	/**
	 * Set status flag
	 * @param recordStatus recordStatus
	 */
	fn set_record_status( &mut self , recordStatus : RecordState);
	
		
	/**
	 * clear record status flag
	 */
	 fn  clear_record_status(&mut self);
	/**
	 * get dirty flag 
	 * @return + : is insert , * : is modified 
	 */
	 fn get_dirty_flag( &self) -> &str;
	
	/**
	 * if the status flage is a none changed state 
	 * @return  isNoneState
	 */
	 fn is_none_state(self) -> bool;
	/**
	 * if the status flag is a modified state 
	 * @return isUpdateState
	 */
	 fn is_update_state(self) -> bool;
	/**
	 * if the status flag is an inserting state 
	 * @return isInsertState
	 */
	 fn is_insert_state(self) -> bool ;
	/**
	 * if the status flag is a deleted state 
	 * @return isDeleteState
	 */
	 fn is_delete_state(self) -> bool;	 	
 	
 	

	/**
	 * Get proxy object of entity
	 * @return entity
	 */
	fn get_entity(self) -> T;
	
	/**
	 * build proxy object of entity
	 */
	fn set_entity(&mut self ,rawRecord : T );

	
	fn  get_old_entity(self ) -> T;

	fn  set_old_entity(&mut self ,oldEntity : T );
	
	
	/**
	 * Get entity , it's a none proxy object
	 * @return entity
	 */
	fn  get_unwrap(self ) -> T;
	
	/**
	 * Set entity , it's a none proxy object
	 *@param entity entity
	 */
	fn set_unwrap( &mut self ,entity : T );
	
} 