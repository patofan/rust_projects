
pub enum RecordState {
    RecordNoState,
    RecordUpdateState,
    RecordInsertState,
    RecordDeleteState,
}

pub trait RecordStatus {

    /**
     * Get status flag
     * @return RecordStatus
     */
	fn get_record_status() -> RecordState;
	
	/**
	 * Set status flag
	 * @param recordStatus recordStatus
	 */
	fn set_record_status( recordStatus : RecordState);
	
		
	/**
	 * clear record status flag
	 */
	 fn  clear_record_status();
	/**
	 * get dirty flag 
	 * @return + : is insert , * : is modified 
	 */
	 fn get_dirty_flag( ) -> String;
	
	/**
	 * if the status flage is a none changed state 
	 * @return  isNoneState
	 */
	 fn is_none_state() -> bool;
	/**
	 * if the status flag is a modified state 
	 * @return isUpdateState
	 */
	 fn is_update_state() -> bool;
	/**
	 * if the status flag is an inserting state 
	 * @return isInsertState
	 */
	 fn is_insert_state() -> bool ;
	/**
	 * if the status flag is a deleted state 
	 * @return isDeleteState
	 */
	 fn is_delete_state() -> bool;	
	
} 

