
const VERSION_MASK: u32 = 16383;

#[derive(Hash, Clone, Copy)]
pub struct Entity {
	id: u32,
}

impl Entity {
	
	pub(crate) fn new(index: u32, version: u32) -> Self {
		Self {
			id: (index << 18) + version,
		}
	}

	///
    /// Gets the index of the entity.
    /// 
    /// # Examples
    ///
    /// ```
	/// 
    /// let manager = rust_ecs::Manager::new();
    /// 
    /// let entity1 = manager.add_entity();
    /// let entity2 = manager.add_entity();
    /// 
    /// assert_eq!(0, entity1.get_index());
    /// assert_eq!(1, entity2.get_index());
    /// ```
	pub fn get_index(&self) -> u32 {
		self.id >> 18
	}

	///
    /// Gets the version of the entity.
    /// 
    /// # Examples
    ///
    /// ```
    /// let manager = rust_ecs::Manager::new();
    /// 
    /// let entity1 = manager.add_entity();
    /// let entity2 = manager.add_entity();
    /// 
    /// assert_eq!(0, entity1.get_version());
    /// assert_eq!(0, entity2.get_version());
    /// ```
	pub fn get_version(&self) -> u32 {
		self.id & VERSION_MASK
	}
}

impl std::ops::Deref for Entity {
	type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.id
    }
}

impl std::fmt::Debug for Entity {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Entity")
         .field("index", &self.get_index())
         .field("version", &self.get_version())
         .finish()
    }
}

impl PartialEq for Entity {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Entity {}

///
/// Submodule for handling of entity creation and deletion.
/// 
pub struct EntityHandler {
	entities: Vec<Entity>,
	head_index: u32,
	killed: usize,
}

impl EntityHandler {
	pub fn new() -> Self {
		Self {
			entities: Vec::new(),
			head_index: 0,
			killed: 0,
		}
	}

	pub fn new_entity(&mut self) -> Entity {
		if self.killed > 0 {
			let head_entity = *self.entities.get(self.head_index as usize).unwrap();
			let new_ident = Entity::new(self.head_index, head_entity.get_version());

			*self.entities.get_mut(self.head_index as usize).unwrap() = new_ident;
			self.head_index = head_entity.get_index();
			//Dear god this code...
			return new_ident;
		} else {
			let index = self.entities.len() as u32;
			let new_ident = Entity::new(index, 0);
			self.entities.push(new_ident);
			return new_ident;
		}
	}

	pub fn kill_entity(&mut self, entity: Entity) {
		if let Some(elem) = self.entities.get_mut(entity.get_index() as usize) {
			*elem = Entity::new(self.head_index, entity.get_version() + 1);
			self.head_index = entity.get_index();
			self.killed += 1;
		}
	}

	pub fn is_alive(&self, entity: Entity) -> bool {
		if let Some(identity) = self.entities.get(entity.get_index() as usize) {
			//Compares version, but inefficient to do to_version
			return *identity == entity;
		}
		return false;
	}
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn new_entity() {
		let mut handler = EntityHandler::new();
		let entity = handler.new_entity();
		assert_eq!(entity.get_index(), 0);
		assert_eq!(entity.get_version(), 0);
	}

	#[test]
	fn new_entity_25() {
		let mut handler = EntityHandler::new();
		for _ in 0..25 {
			handler.new_entity();
		}
		let entity = handler.new_entity();
		assert_eq!(entity.get_index(), 25);
		assert_eq!(entity.get_version(), 0);
	}

	#[test]
	fn kill_head_check() {
		let mut handler = EntityHandler::new();
		handler.new_entity();
		let e1 = handler.new_entity();
		let e2 = handler.new_entity();
		handler.new_entity();

		handler.kill_entity(e2);
		handler.kill_entity(e1);
		
		assert_eq!(handler.head_index, 1);
	}

	#[test]
	fn kill() {
		let mut handler = EntityHandler::new();
		let e1 = handler.new_entity();
		handler.kill_entity(e1);
		
		assert!(!handler.is_alive(e1));
	}

	

	#[test]
	fn respawn() {
		let mut handler = EntityHandler::new();
		handler.new_entity();
		let e1 = handler.new_entity();
		handler.kill_entity(e1);

		assert_eq!(handler.head_index, 1);
	}

	#[test]
	fn entity_ident_version() {
		let e = Entity::new(0, 20);

		assert_eq!(e.id, 20);
	}
}

#[cfg(all(feature = "unstable", test))]
mod benchmark {
	extern crate test;
	use self::test::Bencher;
	use super::*;

	#[bench]
	fn add_and_kill_entity(b: &mut Bencher) {
		let mut handler = EntityHandler::new();

		b.iter(|| {
			let entity = handler.new_entity();
			handler.kill_entity(entity);
		});
	}

	#[bench]
	fn is_alive(b: &mut Bencher) {
		let mut handler = EntityHandler::new();
		let entity = handler.new_entity();

		b.iter(|| {
			handler.is_alive(entity);
		});
	}
}