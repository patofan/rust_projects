


 use page::recordstatus::RecordStatus;
 
 pub trait RecordHolder<T> : RecordStatus {

	/**
	 * Get proxy object of entity
	 * @return entity
	 */
	fn get_entity() -> T;
	
	/**
	 * build proxy object of entity
	 */
	fn set_entity(rawRecord : T );

	
	fn  get_old_entity() -> T;

	fn  set_old_entity(oldEntity : T );
	
	
	/**
	 * Get entity , it's a none proxy object
	 * @return entity
	 */
	fn  get_unwrap() -> T;
	
	/**
	 * Set entity , it's a none proxy object
	 *@param entity entity
	 */
	fn set_unwrap( entity : T );
	
} 