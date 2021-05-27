use crate::{
	action,
	binding::{self, CategoryId, Layout, LayoutBindings},
	device::{self, GamepadKind},
	event,
	source::{Axis, Button},
	User,
};
use std::{
	collections::HashMap,
	mem::MaybeUninit,
	sync::{Once, RwLock, RwLockReadGuard, RwLockWriteGuard},
	time::Instant,
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

pub type UserId = usize;

/// Contains the setup for a particular application.
pub struct System {
	gamepad_input: gilrs::Gilrs,
	users: Vec<User>,
	actions: HashMap<action::Id, action::Action>,
	layouts: Vec<Layout>,
	categories: HashMap<CategoryId, LayoutBindings>,
	unassigned_devices: Vec<device::Id>,
	device_to_user: HashMap<device::Id, UserId>,
	disconnected_device_users: HashMap<device::Id, UserId>,
}

impl System {
	fn new() -> Self {
		Self {
			gamepad_input: gilrs::Gilrs::new().unwrap(),
			users: Vec::new(),
			actions: HashMap::new(),
			layouts: Vec::new(),
			categories: HashMap::new(),
			unassigned_devices: vec![device::Id::Mouse, device::Id::Keyboard],
			device_to_user: HashMap::new(),
			disconnected_device_users: HashMap::new(),
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
	/// Grabs all gamepads from gilrs and attempts to connect them (or cache them if there are no users).
	/// User internally when constructing the singleton.
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

	/// Adds an amount of users to the system,
	/// connecting any unassigned devices to the new users as available.
	pub fn add_users(&mut self, count: usize) -> &mut Self {
		self.users.extend((0..count).map(|_| User::default()));
		self.assign_unused_devices();
		self
	}

	/// Adds an action to the list of actions the system supports.
	pub fn add_action(&mut self, name: action::Id, action: action::Action) -> &mut Self {
		self.actions.insert(name, action);
		self
	}

	/// Adds a layout to the list of layouts the system supports.
	pub fn add_layout(&mut self, layout: Layout) -> &mut Self {
		self.layouts.push(layout);
		self
	}

	/// Adds a mapping of category & layout to the list of bindings for set of actions.
	pub fn add_map_category(&mut self, id: CategoryId, category: LayoutBindings) -> &mut Self {
		self.categories.insert(id, category);
		self
	}

	/// Sets the layout of a user.
	/// If not called, all user's start with a `None` layout (default layout).
	pub fn set_user_layout(&mut self, user_id: UserId, layout: Layout) -> &mut Self {
		if let Some(user) = self.users.get_mut(user_id) {
			user.set_layout(layout, &self.actions);
		}
		self
	}

	/// Enables and disables a provided category for a given user.
	/// When enabled, a user will receive input events for the actions in that category,
	/// until the category is disabled (or until [`System::update`] stops being called).
	pub fn set_category_enabled(
		&mut self,
		user_id: UserId,
		category_id: CategoryId,
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

	/// Enables a category for all existing users. See [`System::set_category_enabled`] for further details.
	pub fn enable_category_for_all(&mut self, category_id: CategoryId) -> &mut Self {
		if let Some(category) = self.categories.get(&category_id) {
			for user in self.users.iter_mut() {
				user.enable_category(category_id, category, &self.actions);
			}
		}
		self
	}

	/// Iterates over all unassigned devices and attempts to assign them to users.
	/// Used predominately to assign devices to users on initialization
	/// (where users are added after the system queries all the gamepads).
	fn assign_unused_devices(&mut self) {
		let unused_devices = self.unassigned_devices.drain(..).collect::<Vec<_>>();
		for device in unused_devices {
			match device {
				// Mouse and Keyboard devices should always go to the first user
				device::Id::Mouse | device::Id::Keyboard => {
					if let Some(first_user_id) = self.users.iter().position(|_| true) {
						self.assign_device(device, first_user_id);
					} else {
						self.unassigned_devices.push(device);
					}
				}
				// Assign gamepads to users without gamepads
				device::Id::Gamepad(_, _) => {
					if let Some(user_id) = self
						.users
						.iter()
						.position(|user| !user.has_gamepad_device())
					{
						self.assign_device(device, user_id);
					} else {
						self.unassigned_devices.push(device);
					}
				}
			}
		}
	}

	/// Assigns a device to a specific user - does not validate if this operation is actually desired.
	fn assign_device(&mut self, device: device::Id, user_id: UserId) {
		self.users[user_id].add_device(device);
		self.device_to_user.insert(device, user_id);
		if cfg!(feature = "log") {
			log::info!(target: "ReBound", "assigning {} to user {}", device, user_id);
		}
	}

	/// Unassigns a device from a user it may belong to.
	/// The device is put in `unassigned_devices` if it had no user,
	/// or in `disconnected_device_users` if it had a user so that we can track the device
	/// should it become available again.
	fn unassign_device(&mut self, device: device::Id) {
		if let Some(user_id) = self.device_to_user.remove(&device) {
			self.users[user_id].remove_device(device);
			self.disconnected_device_users.insert(device, user_id);
			if cfg!(feature = "log") {
				log::info!(target: "ReBound", "unassigning {} from user {}", device, user_id);
			}
		} else {
			self.unassigned_devices.push(device);
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
		let device = device::Id::Gamepad(self.get_gamepad_kind(&id), id);

		if let Some(user_id) = self.disconnected_device_users.remove(&device) {
			if !self.users[user_id].has_gamepad_device() {
				self.assign_device(device, user_id);
				return;
			}
		}

		if let Some(user_id) = self
			.users
			.iter()
			.position(|user| !user.has_gamepad_device())
		{
			self.assign_device(device, user_id);
			return;
		}

		self.unassigned_devices.push(device);
	}

	/// Unassigns a gilrs gamepad from an user it may be assigned to.
	fn disconnect_gamepad(&mut self, id: gilrs::GamepadId) {
		self.unassign_device(device::Id::Gamepad(self.get_gamepad_kind(&id), id));
	}

	/// Queries the gilrs system to get all gamepad input events.
	/// Sends relevant events to `process_event` (or connects/disconnects the gamepad if required).
	fn read_gamepad_events(&mut self) {
		use gilrs::EventType;
		use std::convert::TryFrom;
		while let Some(gilrs::Event {
			id,
			event, /*system time*/
			..
		}) = self.gamepad_input.next_event()
		{
			let time = Instant::now();
			let gamepad_kind = self.get_gamepad_kind(&id);
			let device = device::Id::Gamepad(gamepad_kind, id);
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
							device,
							event::Event {
								source: binding::Source::Gamepad(
									gamepad_kind,
									binding::Gamepad::Button(button),
								),
								state: event::State::ButtonState(event::ButtonState::Pressed),
							},
							time,
						);
					}
				}
				// Previously pressed button has been released.
				EventType::ButtonReleased(btn, _) => {
					if let Some(button) = Button::try_from(btn).ok() {
						self.process_event(
							device,
							event::Event {
								source: binding::Source::Gamepad(
									gamepad_kind,
									binding::Gamepad::Button(button),
								),
								state: event::State::ButtonState(event::ButtonState::Released),
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
							device,
							event::Event {
								source: binding::Source::Gamepad(
									gamepad_kind,
									binding::Gamepad::Button(button),
								),
								state: event::State::ValueChanged(value),
							},
							time,
						);
					}
				}
				// Value of axis has changed. Value can be in range [-1.0, 1.0].
				EventType::AxisChanged(axis, value, _) => {
					if let Some(axis) = Axis::try_from(axis).ok() {
						self.process_event(
							device,
							event::Event {
								source: binding::Source::Gamepad(
									gamepad_kind,
									binding::Gamepad::Axis(axis),
								),
								state: event::State::ValueChanged(value),
							},
							time,
						);
					}
				}
			}
		}
	}

	/// Sends an input event to the system.
	/// Use with caution! Gamepad events are already handled/built-in,
	/// but Mouse and Keyboard events should come from the relevant feature/extension (like winit).
	pub fn send_event(&mut self, source: event::Source, event: event::Event) {
		self.process_event(
			match source {
				event::Source::Mouse => device::Id::Mouse,
				event::Source::Keyboard => device::Id::Keyboard,
			},
			event,
			Instant::now(),
		);
	}

	/// Parses and processes a provided input event from a device.
	fn process_event(&mut self, device: device::Id, event: event::Event, time: Instant) {
		for event in self.parse_event(event) {
			self.update_user_actions(device, event, time);
		}
	}

	// Based on the platform and the event, we may need to split the event into multiple events.
	// For example: the bottom and right face buttons on a gamepad may need to also trigger
	// `Button::VirtualConfirm` or `Button::VirtualDeny` in addition to the original button.
	fn parse_event(&mut self, event: event::Event) -> Vec<event::Event> {
		let mut events = vec![event.clone()];
		if let event::Event {
			source: binding::Source::Gamepad(kind, binding::Gamepad::Button(Button::FaceBottom)),
			..
		} = event
		{
			events.push(event::Event {
				source: binding::Source::Gamepad(
					kind,
					binding::Gamepad::Button(Button::VirtualConfirm),
				),
				..event
			});
		}
		if let event::Event {
			source: binding::Source::Gamepad(kind, binding::Gamepad::Button(Button::FaceRight)),
			..
		} = event
		{
			events.push(event::Event {
				source: binding::Source::Gamepad(
					kind,
					binding::Gamepad::Button(Button::VirtualDeny),
				),
				..event
			});
		}
		events
	}

	/// Processes an event for a specific user based on the device.
	fn update_user_actions(&mut self, device: device::Id, event: event::Event, time: Instant) {
		if let Some(user_id) = self.device_to_user.get(&device) {
			if let Some(user) = self.users.get_mut(*user_id) {
				user.process_event(&event, &time);
			}
		}
	}

	/// Collects gamepad input and updates relevant actions.
	pub fn update(&mut self) {
		self.read_gamepad_events();
		let time = Instant::now();
		for user in self.users.iter_mut() {
			user.update(&time);
		}
	}

	/// Returns a list of active user ids.
	pub fn get_user_ids(&self) -> Vec<UserId> {
		(0..self.users.len()).collect()
	}

	/// Returns the state for an action on a given user.
	/// If the action is invalid or is not enabled for the user's layout
	/// or list of enabled categories, `None` will be returned.
	pub fn get_user_action(
		&self,
		user_id: UserId,
		action_id: action::Id,
	) -> Option<&action::State> {
		self.users
			.get(user_id)
			.map(|user| user.get_action(action_id))
			.flatten()
	}
}
