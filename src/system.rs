use crate::{Action, ActionId, Category, CategoryId, Layout, User};
use std::{
	collections::HashMap,
	mem::MaybeUninit,
	sync::{Once, RwLock, RwLockReadGuard, RwLockWriteGuard},
};

/// Contains the setup for a particular application.
pub struct System {
	users: Vec<User>,
	actions: HashMap<String, Action>,
	layouts: Vec<Layout>,
	categories: HashMap<Option<CategoryId>, Category>,
}

impl Default for System {
	fn default() -> Self {
		Self {
			users: Vec::new(),
			actions: HashMap::new(),
			layouts: Vec::new(),
			categories: HashMap::new(),
		}
	}
}

impl System {
	pub fn get() -> &'static RwLock<Self> {
		static mut INSTANCE: Singleton = Singleton::uninit();
		unsafe { INSTANCE.get() }
	}

	pub fn read() -> RwLockReadGuard<'static, Self> {
		Self::get().read().unwrap()
	}

	pub fn write() -> RwLockWriteGuard<'static, Self> {
		Self::get().write().unwrap()
	}
}

impl System {
	pub fn set_user_count(&mut self, count: usize) -> &mut Self {
		self.users.resize(count, User::default());
		self
	}

	pub fn user(&mut self, id: usize) -> Option<&mut User> {
		self.users.get_mut(id)
	}

	pub fn add_action(&mut self, name: ActionId, action: Action) -> &mut Self {
		self.actions.insert(name.to_owned(), action);
		self
	}

	pub fn add_layout(&mut self, layout: Layout) -> &mut Self {
		self.layouts.push(layout);
		self
	}

	pub fn add_map_category(&mut self, id: Option<CategoryId>, category: Category) -> &mut Self {
		self.categories.insert(id, category);
		self
	}
}

struct Singleton(MaybeUninit<RwLock<System>>, Once);

impl Singleton {
	const fn uninit() -> Singleton {
		Singleton(MaybeUninit::uninit(), Once::new())
	}

	fn get(&mut self) -> &'static RwLock<System> {
		let rwlock = &mut self.0;
		let once = &mut self.1;
		once.call_once(|| unsafe { rwlock.as_mut_ptr().write(RwLock::new(System::default())) });
		unsafe { &*self.0.as_ptr() }
	}
}
