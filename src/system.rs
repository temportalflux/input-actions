use crate::{
	Action, ActionId, Category, CategoryId, ControllerId, Event, EventSource, GamepadKind, Layout,
	User,
};
use std::{
	collections::HashMap,
	mem::MaybeUninit,
	sync::{Once, RwLock, RwLockReadGuard, RwLockWriteGuard},
};

struct Singleton(MaybeUninit<RwLock<System>>, Once);

impl Singleton {
	const fn uninit() -> Singleton {
		Singleton(MaybeUninit::uninit(), Once::new())
	}

	fn get(&mut self) -> &'static RwLock<System> {
		let rwlock = &mut self.0;
		let once = &mut self.1;
		once.call_once(|| unsafe { rwlock.as_mut_ptr().write(RwLock::new(System::new())) });
		unsafe { &*self.0.as_ptr() }
	}
}

type UserId = usize;

/// Contains the setup for a particular application.
pub struct System {
	gamepad_input: gilrs::Gilrs,
	users: Vec<User>,
	actions: HashMap<String, Action>,
	layouts: Vec<Layout>,
	categories: HashMap<Option<CategoryId>, Category>,
	controller_to_user: HashMap<ControllerId, UserId>,
	disconnected_controller_users: HashMap<ControllerId, UserId>,
}

impl System {
	fn new() -> Self {
		Self {
			gamepad_input: gilrs::Gilrs::new().unwrap(),
			users: Vec::new(),
			actions: HashMap::new(),
			layouts: Vec::new(),
			categories: HashMap::new(),
			controller_to_user: HashMap::new(),
			disconnected_controller_users: HashMap::new(),
		}
		.initialize_gamepads()
	}

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
	fn initialize_gamepads(mut self) -> Self {
		let existing_gamepad_ids = self
			.gamepad_input
			.gamepads()
			.map(|(id, _)| id)
			.collect::<Vec<_>>();
		for id in existing_gamepad_ids {
			self.connect_gamepad(id);
		}
		self
	}

	pub fn set_user_count(&mut self, count: usize) -> &mut Self {
		self.users.resize(count, User::default());
		if !self.users.is_empty() {
			self.assign_controller(ControllerId::Mouse, 0);
			self.assign_controller(ControllerId::Keyboard, 0);
		}
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

	fn assign_controller(&mut self, controller: ControllerId, user_id: UserId) {
		self.users[user_id].add_controller(controller);
		self.controller_to_user.insert(controller, user_id);
		if cfg!(feature = "log") {
			log::info!(target: "ReBound", "assigning {} to user {}", controller, user_id);
		}
	}

	fn unassign_controller(&mut self, controller: ControllerId) {
		if let Some(user_id) = self.controller_to_user.remove(&controller) {
			self.users[user_id].remove_controller(controller);
			self.disconnected_controller_users
				.insert(controller, user_id);
			if cfg!(feature = "log") {
				log::info!(target: "ReBound", "unassigning {} from user {}", controller, user_id);
			}
		}
	}

	fn get_gamepad_kind(&self, _id: &gilrs::GamepadId) -> GamepadKind {
		// GILRS seems to always provide "Xbox Controller" as the gamepad name (`name()` AND `os_name()`)
		// regardless of what kind of controller is actually is.
		// Until this can be addressed, assume all controllers are Dual-Axis-Gamepad.
		// let gamepad = self.gamepad_input.gamepad(id);
		GamepadKind::DualAxisGamepad
	}

	/// Connects a gamepad to user data.
	///
	/// If the gamepad previously disconnected from a user, it is automatically assigned to the same user
	/// (assuming the provided `id` is the same).
	///
	/// If user already has another gamepad or the gamepad was never previously connected,
	/// then it is assigned to the first user without a gamepad.
	fn connect_gamepad(&mut self, id: gilrs::GamepadId) {
		let gamepad_kind = self.get_gamepad_kind(&id);
		let controller = ControllerId::Gamepad(gamepad_kind, id);

		if let Some(user_id) = self.disconnected_controller_users.remove(&controller) {
			if !self.users[user_id].has_gamepad_controller() {
				self.assign_controller(controller, user_id);
				return;
			}
		}

		if let Some(user_id) = self
			.users
			.iter()
			.position(|user| !user.has_gamepad_controller())
		{
			self.assign_controller(controller, user_id);
		}
	}

	fn disconnect_gamepad(&mut self, id: gilrs::GamepadId) {
		let gamepad_kind = self.get_gamepad_kind(&id);
		let controller = ControllerId::Gamepad(gamepad_kind, id);
		self.unassign_controller(controller);
	}

	pub fn read_gamepad_events(&mut self) {
		use crate::{Axis, Button};
		use gilrs::{Event, EventType};
		use std::convert::TryFrom;
		while let Some(Event { id, event, time }) = self.gamepad_input.next_event() {
			let controller = ControllerId::Gamepad(self.get_gamepad_kind(&id), id);
			match event {
				// Gamepad has been connected. If gamepad's UUID doesn't match one of disconnected gamepads,
				// newly connected gamepad will get new ID.
				EventType::Connected => self.connect_gamepad(id),
				// Gamepad has been disconnected. Disconnected gamepad will not generate any new events.
				EventType::Disconnected => self.disconnect_gamepad(id),
				// There was an `Event`, but it was dropped by one of filters. You should ignore it.
				EventType::Dropped => {}
				// Some button on gamepad has been pressed.
				EventType::ButtonPressed(btn, _) => {
					if let Some(button) = Button::try_from(btn).ok() {
						self.process_event(
							controller,
							crate::Event::GamepadButtonState(
								button,
								crate::EventButtonState::Pressed,
							),
							time,
						);
					}
				}
				// Previously pressed button has been released.
				EventType::ButtonReleased(btn, _) => {
					if let Some(button) = Button::try_from(btn).ok() {
						self.process_event(
							controller,
							crate::Event::GamepadButtonState(
								button,
								crate::EventButtonState::Released,
							),
							time,
						);
					}
				}
				// This event can be generated by [`ev::Repeat`](filter/struct.Repeat.html) event filter.
				EventType::ButtonRepeated(_btn, _) => {}
				// Value of button has changed. Value can be in range [0.0, 1.0].
				EventType::ButtonChanged(btn, value, _) => {
					if let Some(button) = Button::try_from(btn).ok() {
						self.process_event(
							controller,
							crate::Event::GamepadButtonChanged(button, value),
							time,
						);
					}
				}
				// Value of axis has changed. Value can be in range [-1.0, 1.0].
				EventType::AxisChanged(axis, value, _) => {
					if let Some(axis) = Axis::try_from(axis).ok() {
						self.process_event(
							controller,
							crate::Event::GamepadAxisChanged(axis, value),
							time,
						);
					}
				}
			}
		}
	}

	pub fn send_event(&mut self, source: EventSource, event: Event) {
		self.process_event(
			match source {
				EventSource::Mouse => ControllerId::Mouse,
				EventSource::Keyboard => ControllerId::Keyboard,
			},
			event,
			std::time::SystemTime::now(),
		);
	}

	fn process_event(
		&mut self,
		controller: ControllerId,
		event: Event,
		time: std::time::SystemTime,
	) {
	}
}
