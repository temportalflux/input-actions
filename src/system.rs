use crate::{
	action,
	binding::{self, ActionSet, ActionSetId, LayoutId},
	device::{self, GamepadKind},
	event,
	source::{self, Axis, Button},
	ArcLockUser, WeakLockUser,
};
use std::{
	collections::HashMap,
	sync::{Arc, RwLock, Weak},
};

pub type UserId = usize;

pub type ArcLockConfig = Arc<RwLock<Config>>;
pub type WeakLockConfig = Weak<RwLock<Config>>;
#[derive(Default, Clone, Debug)]
pub struct Config {
	actions: HashMap<action::Id, source::Kind>,
	layouts: Vec<LayoutId>,
	action_sets: HashMap<ActionSetId, ActionSet>,
}

impl Config {
	/// Adds an action to the list of actions the system supports.
	pub fn add_action(mut self, name: action::Id, action: source::Kind) -> Self {
		self.actions.insert(name, action);
		self
	}

	/// Adds a layout to the list of layouts the system supports.
	pub fn add_layout(mut self, layout: LayoutId) -> Self {
		self.layouts.push(layout);
		self
	}

	/// Associates an [`action set`](ActionSet) with an [`id`](ActionSetId).
	pub fn add_action_set(mut self, id: ActionSetId, set: ActionSet) -> Self {
		self.action_sets.insert(id, set);
		self
	}

	pub(crate) fn get_action_set(&self, id: &binding::ActionSetId) -> Option<&binding::ActionSet> {
		self.action_sets.get(&id)
	}
}

pub struct DeviceCache {
	gamepad_input: gilrs::Gilrs,
	consts: Arc<RwLock<Consts>>,
	unassigned_devices: Vec<device::Id>,
	assigned_devices: HashMap<device::Id, (WeakLockUser, event::InputSender)>,
	disconnected_devices: HashMap<device::Id, (WeakLockUser, event::InputSender)>,
	users: Vec<(WeakLockUser, Vec<device::Id>)>,
}

impl Default for DeviceCache {
	fn default() -> Self {
		Self {
			gamepad_input: gilrs::Gilrs::new().unwrap(),
			consts: Default::default(),
			unassigned_devices: vec![device::Id::Mouse, device::Id::Keyboard],
			assigned_devices: HashMap::new(),
			disconnected_devices: HashMap::new(),
			users: Vec::new(),
		}
		.initialize_gamepads()
	}
}

impl DeviceCache {
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

	fn get_gamepad_kind(_id: &gilrs::GamepadId) -> GamepadKind {
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
		let device_id = device::Id::Gamepad(Self::get_gamepad_kind(&id), id.into());

		if let Some((weak_user, _)) = self.disconnected_devices.remove(&device_id) {
			if let Some(arc_user) = weak_user.upgrade() {
				self.assign_device(&arc_user, device_id);
				return;
			}
		}

		self.unassigned_devices.push(device_id);
	}

	/// Unassigns a gilrs gamepad from an user it may be assigned to.
	fn disconnect_gamepad(&mut self, id: gilrs::GamepadId) {
		let device_id = device::Id::Gamepad(Self::get_gamepad_kind(&id), id.into());
		if let Some(owner) = self.assigned_devices.remove(&device_id) {
			for (weak, device_ids) in self.users.iter_mut() {
				if weak.ptr_eq(&owner.0) {
					device_ids.retain(|&id| id != device_id);
					break;
				}
			}
			self.disconnected_devices.insert(device_id, owner);
		} else {
			self.unassigned_devices.retain(|&id| id != device_id);
		}
	}

	fn assign_device(&mut self, arc_user: &ArcLockUser, device_id: device::Id) {
		let input_sender = arc_user.read().unwrap().input_sender().clone();
		let weak_user = Arc::downgrade(&arc_user);
		for (weak, device_ids) in self.users.iter_mut() {
			if weak.ptr_eq(&weak_user) {
				device_ids.push(device_id);
				break;
			}
		}
		self.assigned_devices
			.insert(device_id, (weak_user, input_sender));
	}

	/// Iterates over all unassigned devices and attempts to assign them to users.
	/// Used predominately to assign devices to users on initialization
	/// (where users are added after the system queries all the gamepads).
	fn assign_unused_devices(&mut self) {
		let unused_devices = self.unassigned_devices.drain(..).collect::<Vec<_>>();
		'iterDevices: for device in unused_devices {
			match device {
				// Mouse and Keyboard devices should always go to the first user
				device::Id::Mouse | device::Id::Keyboard => {
					for (weak_user, _device_ids) in self.users.iter_mut() {
						if let Some(arc_user) = weak_user.upgrade() {
							self.assign_device(&arc_user, device);
							continue 'iterDevices;
						}
					}
				}
				// Assign gamepads to users without gamepads
				device::Id::Gamepad(_, _) => {
					for (weak_user, device_ids) in self.users.iter_mut() {
						if let Some(arc_user) = weak_user.upgrade() {
							let has_gamepad = device_ids.iter().any(|id| match id {
								device::Id::Gamepad(_, _) => true,
								_ => false,
							});
							if !has_gamepad {
								self.assign_device(&arc_user, device);
								continue 'iterDevices;
							}
						}
					}
				}
			}
			self.unassigned_devices.push(device);
		}
	}

	pub fn add_user(&mut self, user: WeakLockUser) {
		self.users.push((user, Vec::new()));
	}

	/// Queries the gilrs system to get all gamepad input events.
	/// Sends relevant events to `process_event` (or connects/disconnects the gamepad if required).
	pub fn update(&mut self) {
		self.prune_users();
		self.assign_unused_devices();
		self.read_events();
	}

	pub fn users(&self) -> Vec<WeakLockUser> {
		self.users.iter().map(|(user, _)| user.clone()).collect()
	}

	fn prune_users(&mut self) {
		// Remove all users who've been dropped.
		// Can use `Vec::drain_filter` when that api stabilizes.
		// O(n) performance where `n` is the number of loaded chunks
		let mut i = 0;
		while i < self.users.len() {
			if self.users[i].0.strong_count() == 0 {
				let (_, mut device_ids) = self.users.remove(i);
				self.unassigned_devices.append(&mut device_ids);
			} else {
				i += 1;
			}
		}
	}

	fn read_events(&mut self) {
		use gilrs::EventType;
		use std::convert::TryFrom;
		while let Some(gilrs::Event {
			id,
			event, /*system time*/
			..
		}) = self.gamepad_input.next_event()
		{
			let gamepad_kind = Self::get_gamepad_kind(&id);
			let device = device::Id::Gamepad(gamepad_kind, id.into());
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
						self.send_device_event((
							device,
							binding::Source::Gamepad(
								gamepad_kind,
								binding::Gamepad::Button(button),
							),
							event::State::ButtonState(event::ButtonState::Pressed),
						));
					}
				}
				// Previously pressed button has been released.
				EventType::ButtonReleased(btn, _) => {
					if let Some(button) = Button::try_from(btn).ok() {
						self.send_device_event((
							device,
							binding::Source::Gamepad(
								gamepad_kind,
								binding::Gamepad::Button(button),
							),
							event::State::ButtonState(event::ButtonState::Released),
						));
					}
				}
				// This event can be generated by [`ev::Repeat`](filter/struct.Repeat.html) event filter.
				EventType::ButtonRepeated(_btn, _) => {}
				// Value of button has changed. Value can be in range [0.0, 1.0].
				EventType::ButtonChanged(btn, value, _) => {
					if let Some(button) = Button::try_from(btn).ok() {
						self.send_device_event((
							device,
							binding::Source::Gamepad(
								gamepad_kind,
								binding::Gamepad::Button(button),
							),
							event::State::ValueChanged(value),
						));
					}
				}
				// Value of axis has changed. Value can be in range [-1.0, 1.0].
				EventType::AxisChanged(axis, value, _) => {
					if let Some(axis) = Axis::try_from(axis).ok() {
						self.send_device_event((
							device,
							binding::Source::Gamepad(gamepad_kind, binding::Gamepad::Axis(axis)),
							event::State::ValueChanged(value),
						));
					}
				}
			}
		}
	}

	pub fn consts(&self) -> Weak<RwLock<Consts>> {
		Arc::downgrade(&self.consts)
	}

	/// Sends an input event to the system.
	/// Use with caution! Gamepad events are already handled/built-in,
	/// but Mouse and Keyboard events should come from the relevant feature/extension (like winit).
	pub fn send_event(&mut self, event: event::Event) {
		match event {
			event::Event::Window(event::WindowEvent::ResolutionChanged(width, height)) => {
				let mut consts = self.consts.write().unwrap();
				consts.screen_size = (
					(width as f64) / consts.scale_factor,
					(height as f64) / consts.scale_factor,
				);
				return;
			}
			event::Event::Window(event::WindowEvent::ScaleFactorChanged(
				width,
				height,
				scale_factor,
			)) => {
				let mut consts = self.consts.write().unwrap();
				consts.scale_factor = scale_factor;
				consts.screen_size = (
					(width as f64) / consts.scale_factor,
					(height as f64) / consts.scale_factor,
				);
				return;
			}
			event::Event::Input(device_source, binding_source, state) => {
				self.send_device_event((device_source, binding_source, state));
			}
		}
	}

	fn send_device_event(&self, event: (device::Id, binding::Source, event::State)) {
		for (device, binding, event) in self.parse_input_event(event) {
			if let Some((_user, sender)) = self.assigned_devices.get(&device) {
				let _ = sender.try_send((binding, event));
			}
		}
	}

	// Based on the platform and the event, we may need to split the event into multiple events.
	// For example: the bottom and right face buttons on a gamepad may need to also trigger
	// `Button::VirtualConfirm` or `Button::VirtualDeny` in addition to the original button.
	fn parse_input_event(
		&self,
		event: (device::Id, binding::Source, event::State),
	) -> Vec<(device::Id, binding::Source, event::State)> {
		let mut events = vec![event.clone()];
		if let (
			device_id,
			binding::Source::Gamepad(kind, binding::Gamepad::Button(Button::FaceBottom)),
			state,
		) = event
		{
			events.push((
				device_id,
				binding::Source::Gamepad(kind, binding::Gamepad::Button(Button::VirtualConfirm)),
				state,
			));
		}
		if let (
			device_id,
			binding::Source::Gamepad(kind, binding::Gamepad::Button(Button::FaceRight)),
			state,
		) = event
		{
			events.push((
				device_id,
				binding::Source::Gamepad(kind, binding::Gamepad::Button(Button::VirtualDeny)),
				state,
			));
		}
		events
	}
}

pub struct Consts {
	pub(crate) screen_size: (f64, f64),
	scale_factor: f64,
}

impl Default for Consts {
	fn default() -> Self {
		Self {
			screen_size: (0.0, 0.0),
			scale_factor: 1.0,
		}
	}
}
