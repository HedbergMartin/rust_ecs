
use crate::ecs::Entity;

const ENTITY_MASK: u32 = 4294950912;
const VERSION_MASK: u32 = 16383;

pub struct EntityHandler {
	entities: Vec<Entity>,
	head_id: usize,
	killed: usize,
}

impl EntityHandler {
	pub fn new() -> Self {
		Self {
			entities: Vec::new(),
			head_id: 0,
			killed: 0,
		}
	}

	pub fn new_entity(&mut self) -> Entity {
		/* if (self.pending > 0) {
			//Clean a pending entity and return it.
			return 0;
		} else  */ 
		if self.killed > 0 {
			let head_entity = *self.entities.get(self.head_id).unwrap();
			let head_vers = EntityHandler::to_version(head_entity);
			let new_ident = EntityHandler::to_ident(self.head_id, head_vers + 1);

			*self.entities.get_mut(self.head_id).unwrap() = new_ident;
			self.head_id = EntityHandler::to_entityid(head_entity);
			//Dear god this code...
			return new_ident;
		} else {
			let id = self.entities.len();
			let new_ident = EntityHandler::to_ident(id, 0);
			self.entities.push(new_ident);
			return new_ident;
		}
	}

	pub fn kill_entity(&mut self, entity: Entity) {
		//TODO Benchmark if if-statement should be removed.
		if self.killed != 0 {
			//This line sucks, sorry
			*self.entities.get_mut(EntityHandler::to_entityid(entity)).unwrap() = EntityHandler::to_ident(self.head_id, EntityHandler::to_version(entity));
		}
			
		self.head_id = EntityHandler::to_entityid(entity);
		self.killed += 1;
	}

	fn to_ident(id: usize, version: usize) -> Entity {
		id << 18 + version
	}

	fn to_entityid(entity: Entity) -> usize {
		entity >> 18
	}

	fn to_version(entity: Entity) -> usize {
		entity & VERSION_MASK as usize
	}

	pub fn print_entity(entity: Entity) {
		print!("Entity {{ id {}, version: {} }}", EntityHandler::to_entityid(entity), EntityHandler::to_version(entity));
	}
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn new_entity() {
		let mut handler = EntityHandler::new();
		let entity = handler.new_entity();
		assert_eq!(EntityHandler::to_entityid(entity), 0);
		assert_eq!(EntityHandler::to_version(entity), 0);
	}

	#[test]
	fn new_entity_25() {
		let mut handler = EntityHandler::new();
		for _ in 0..25 {
			handler.new_entity();
		}
		let entity = handler.new_entity();
		assert_eq!(EntityHandler::to_entityid(entity), 25);
		assert_eq!(EntityHandler::to_version(entity), 0);
	}

	#[test]
	fn kill() {
		let mut handler = EntityHandler::new();
		handler.new_entity();
		let e1 = handler.new_entity();
		let e2 = handler.new_entity();
		handler.new_entity();

		handler.kill_entity(e2);
		handler.kill_entity(e1);
		
		assert_eq!(handler.head_id, 1);
	}

}