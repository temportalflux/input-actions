use crate::{
	binding::Binding, Action, ActionId, Axis, Button, Category, CategoryId, ControllerId, Event,
	EventButtonState, EventSource, EventState, GamepadKind, Layout, User,
};
use std::{
	collections::HashMap,
	mem::MaybeUninit,
	sync::{Once, RwLock, RwLockReadGuard, RwLockWriteGuard},
	time::SystemTime,
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
	actions: HashMap<ActionId, Action>,
	layouts: Vec<Layout>,
	categories: HashMap<Option<CategoryId>, Category>,
	unassigned_controllers: Vec<ControllerId>,
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
			unassigned_controllers: vec![ControllerId::Mouse, ControllerId::Keyboard],
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

	pub fn add_users(&mut self, count: usize) -> &mut Self {
		self.users.extend((0..count).map(|_| User::default()));
		self.assign_unused_controllers();
		self
	}

	pub fn add_action(&mut self, name: ActionId, action: Action) -> &mut Self {
		self.actions.insert(name, action);
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

	pub fn set_user_layout(&mut self, user_id: UserId, layout: Option<Layout>) -> &mut Self {
		if let Some(user) = self.users.get_mut(user_id) {
			user.set_layout(layout, &self.actions);
		}
		self
	}

	pub fn set_category_enabled(
		&mut self,
		user_id: UserId,
		category_id: Option<CategoryId>,
		enabled: bool,
	) -> &mut Self {
		if let Some(user) = self.users.get_mut(user_id) {
			if enabled {
				if let Some(category) = self.categories.get(&category_id) {
					user.enable_category(category_id, category, &self.actions);
				}
			} else {
				user.disable_category(category_id);
			}
		}
		self
	}

	pub fn enable_category_for_all(&mut self, category_id: Option<CategoryId>) -> &mut Self {
		if let Some(category) = self.categories.get(&category_id) {
			for user in self.users.iter_mut() {
				user.enable_category(category_id, category, &self.actions);
			}
		}
		self
	}

	fn assign_unused_controllers(&mut self) {
		let unused_controllers = self.unassigned_controllers.drain(..).collect::<Vec<_>>();
		for controller in unused_controllers {
			match controller {
				ControllerId::Mouse | ControllerId::Keyboard => {
					if let Some(first_user_id) = self.users.iter().position(|_| true) {
						self.assign_controller(controller, first_user_id);
					} else {
						self.unassigned_controllers.push(controller);
					}
				}
				ControllerId::Gamepad(_, _) => {
					if let Some(user_id) = self
						.users
						.iter()
						.position(|user| !user.has_gamepad_controller())
					{
						self.assign_controller(controller, user_id);
					} else {
						self.unassigned_controllers.push(controller);
					}
				}
			}
		}
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
		let controller = ControllerId::Gamepad(self.get_gamepad_kind(&id), id);

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
			return;
		}

		self.unassigned_controllers.push(controller);
	}

	fn disconnect_gamepad(&mut self, id: gilrs::GamepadId) {
		self.unassign_controller(ControllerId::Gamepad(self.get_gamepad_kind(&id), id));
	}

	fn read_gamepad_events(&mut self) {
		use gilrs::EventType;
		use std::convert::TryFrom;
		while let Some(gilrs::Event { id, event, time }) = self.gamepad_input.next_event() {
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
							Event {
								binding: Binding::GamepadButton(button),
								state: EventState::ButtonState(crate::EventButtonState::Pressed),
							},
							time,
						);
					}
				}
				// Previously pressed button has been released.
				EventType::ButtonReleased(btn, _) => {
					if let Some(button) = Button::try_from(btn).ok() {
						self.process_event(
							controller,
							Event {
								binding: Binding::GamepadButton(button),
								state: EventState::ButtonState(EventButtonState::Released),
							},
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
							Event {
								binding: Binding::GamepadButton(button),
								state: EventState::ValueChanged(value),
							},
							time,
						);
					}
				}
				// Value of axis has changed. Value can be in range [-1.0, 1.0].
				EventType::AxisChanged(axis, value, _) => {
					if let Some(axis) = Axis::try_from(axis).ok() {
						self.process_event(
							controller,
							Event {
								binding: Binding::GamepadAxis(axis),
								state: EventState::ValueChanged(value),
							},
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
			SystemTime::now(),
		);
	}

	fn process_event(&mut self, controller: ControllerId, event: Event, time: SystemTime) {
		for event in self.parse_event(event) {
			self.update_user_actions(controller, event, time);
		}
	}

	fn parse_event(&mut self, event: Event) -> Vec<Event> {
		// Based on the platform and the event, we may need to split the event into multiple events.
		// For example: the bottom and right face buttons on a gamepad may need to also trigger
		// `Button::VirtualConfirm` or `Button::VirtualDeny` in addition to the original button.
		let mut events = vec![event.clone()];
		if let Event {
			binding: Binding::GamepadButton(Button::FaceBottom),
			..
		} = event
		{
			events.push(Event {
				binding: Binding::GamepadButton(Button::VirtualConfirm),
				..event
			});
		}
		if let Event {
			binding: Binding::GamepadButton(Button::FaceRight),
			..
		} = event
		{
			events.push(Event {
				binding: Binding::GamepadButton(Button::VirtualDeny),
				..event
			});
		}
		events
	}

	fn update_user_actions(&mut self, controller: ControllerId, event: Event, time: SystemTime) {
		if let Some(user_id) = self.controller_to_user.get(&controller) {
			if let Some(user) = self.users.get_mut(*user_id) {
				user.process_event(controller, &event, &time);
			}
		}
	}

	pub fn update(&mut self) {
		self.read_gamepad_events();
		let time = SystemTime::now();
		for user in self.users.iter_mut() {
			user.update(&time);
		}
	}
}
