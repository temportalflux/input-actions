use crate::{
	binding, device, event,
	source::{Key, MouseButton},
};
use std::convert::{TryFrom, TryInto};
use winit::event::VirtualKeyCode;

// TODO: Winit gamepad support is still in progress https://github.com/rust-windowing/winit/issues/944
pub fn parse_winit_event<'a, T>(event: &winit::event::Event<'a, T>) -> Result<event::Event, ()> {
	use winit::event::{DeviceEvent, ElementState, KeyboardInput};
	match event {
		// resolution changed
		winit::event::Event::WindowEvent {
			event: winit::event::WindowEvent::Resized(physical_size),
			..
		} => Ok(event::Event::Window(event::WindowEvent::ResolutionChanged(
			physical_size.width,
			physical_size.height,
		))),
		// dpi changed
		winit::event::Event::WindowEvent {
			event:
				winit::event::WindowEvent::ScaleFactorChanged {
					scale_factor,
					new_inner_size,
				},
			..
		} => Ok(event::Event::Window(
			event::WindowEvent::ScaleFactorChanged(
				new_inner_size.width,
				new_inner_size.height,
				*scale_factor,
			),
		)),
		winit::event::Event::DeviceEvent {
			event: DeviceEvent::Motion { axis, value },
			..
		} => {
			match axis {
				// Mouse X is axis 0
				0 => Ok(event::Event::Input(
					device::Id::Mouse,
					binding::Source::Mouse(binding::Mouse::Move(binding::MouseAxis::MouseX)),
					event::State::MouseMove(*value),
				)),
				// Mouse Y is axis 1
				1 => Ok(event::Event::Input(
					device::Id::Mouse,
					binding::Source::Mouse(binding::Mouse::Move(binding::MouseAxis::MouseY)),
					event::State::MouseMove(*value),
				)),
				_ => Err(()), // NO-OP
			}
		}
		/* TODO: Scroll wheel
		winit::event::Event::DeviceEvent {
			event:
				DeviceEvent::MouseWheel {
					delta: MouseScrollDelta::LineDelta(horizontal, vertical),
				},
			..
		} => {
			let mut events = Vec::with_capacity(2);
			if *horizontal > std::f32::EPSILON {
				events.push(event::Event {
					source: binding::Source::Mouse(binding::Mouse::Move(
						binding::MouseAxis::MouseX,
					)),
					state: event::State::MouseScroll(*horizontal),
				});
			}
			if *vertical > std::f32::EPSILON {
				events.push(event::Event {
					source: binding::Source::Mouse(binding::Mouse::Move(
						binding::MouseAxis::MouseY,
					)),
					state: event::State::MouseScroll(*vertical),
				});
			}
			Ok((event::Source::Mouse, events))
		}
		*/
		winit::event::Event::DeviceEvent {
			event: DeviceEvent::Button { button, state },
			..
		} => MouseButton::try_from(*button)
			.map(|button_enum| {
				event::Event::Input(
					device::Id::Mouse,
					binding::Source::Mouse(binding::Mouse::Button(button_enum)),
					event::State::ButtonState(match state {
						ElementState::Pressed => event::ButtonState::Pressed,
						ElementState::Released => event::ButtonState::Released,
					}),
				)
			})
			.map_err(|id| {
				println!("ERROR failed to parse button id {:?}", id);
				()
			}),
		winit::event::Event::DeviceEvent {
			event:
				DeviceEvent::Key(KeyboardInput {
					state,
					virtual_keycode: Some(keycode),
					..
				}),
			..
		} => (*keycode)
			.try_into()
			.map(|keycode| {
				event::Event::Input(
					device::Id::Keyboard,
					binding::Source::Keyboard(keycode),
					event::State::ButtonState(match state {
						ElementState::Pressed => event::ButtonState::Pressed,
						ElementState::Released => event::ButtonState::Released,
					}),
				)
			})
			.map_err(|_| ()),
		_ => Err(()),
	}
}

impl TryFrom<winit::event::ButtonId> for MouseButton {
	type Error = winit::event::ButtonId;
	fn try_from(id: winit::event::ButtonId) -> Result<Self, Self::Error> {
		match id {
			1 => Ok(MouseButton::Left),
			2 => Ok(MouseButton::Center),
			3 => Ok(MouseButton::Right),
			_ => Err(id),
		}
	}
}

impl TryFrom<VirtualKeyCode> for Key {
	type Error = ();
	fn try_from(winit: VirtualKeyCode) -> Result<Self, Self::Error> {
		match winit {
			VirtualKeyCode::Key1 => Ok(Key::Key1),
			VirtualKeyCode::Key2 => Ok(Key::Key2),
			VirtualKeyCode::Key3 => Ok(Key::Key3),
			VirtualKeyCode::Key4 => Ok(Key::Key4),
			VirtualKeyCode::Key5 => Ok(Key::Key5),
			VirtualKeyCode::Key6 => Ok(Key::Key6),
			VirtualKeyCode::Key7 => Ok(Key::Key7),
			VirtualKeyCode::Key8 => Ok(Key::Key8),
			VirtualKeyCode::Key9 => Ok(Key::Key9),
			VirtualKeyCode::Key0 => Ok(Key::Key0),
			VirtualKeyCode::A => Ok(Key::A),
			VirtualKeyCode::B => Ok(Key::B),
			VirtualKeyCode::C => Ok(Key::C),
			VirtualKeyCode::D => Ok(Key::D),
			VirtualKeyCode::E => Ok(Key::E),
			VirtualKeyCode::F => Ok(Key::F),
			VirtualKeyCode::G => Ok(Key::G),
			VirtualKeyCode::H => Ok(Key::H),
			VirtualKeyCode::I => Ok(Key::I),
			VirtualKeyCode::J => Ok(Key::J),
			VirtualKeyCode::K => Ok(Key::K),
			VirtualKeyCode::L => Ok(Key::L),
			VirtualKeyCode::M => Ok(Key::M),
			VirtualKeyCode::N => Ok(Key::N),
			VirtualKeyCode::O => Ok(Key::O),
			VirtualKeyCode::P => Ok(Key::P),
			VirtualKeyCode::Q => Ok(Key::Q),
			VirtualKeyCode::R => Ok(Key::R),
			VirtualKeyCode::S => Ok(Key::S),
			VirtualKeyCode::T => Ok(Key::T),
			VirtualKeyCode::U => Ok(Key::U),
			VirtualKeyCode::V => Ok(Key::V),
			VirtualKeyCode::W => Ok(Key::W),
			VirtualKeyCode::X => Ok(Key::X),
			VirtualKeyCode::Y => Ok(Key::Y),
			VirtualKeyCode::Z => Ok(Key::Z),
			VirtualKeyCode::Escape => Ok(Key::Escape),
			VirtualKeyCode::F1 => Ok(Key::F1),
			VirtualKeyCode::F2 => Ok(Key::F2),
			VirtualKeyCode::F3 => Ok(Key::F3),
			VirtualKeyCode::F4 => Ok(Key::F4),
			VirtualKeyCode::F5 => Ok(Key::F5),
			VirtualKeyCode::F6 => Ok(Key::F6),
			VirtualKeyCode::F7 => Ok(Key::F7),
			VirtualKeyCode::F8 => Ok(Key::F8),
			VirtualKeyCode::F9 => Ok(Key::F9),
			VirtualKeyCode::F10 => Ok(Key::F10),
			VirtualKeyCode::F11 => Ok(Key::F11),
			VirtualKeyCode::F12 => Ok(Key::F12),
			VirtualKeyCode::F13 => Ok(Key::F13),
			VirtualKeyCode::F14 => Ok(Key::F14),
			VirtualKeyCode::F15 => Ok(Key::F15),
			VirtualKeyCode::F16 => Ok(Key::F16),
			VirtualKeyCode::F17 => Ok(Key::F17),
			VirtualKeyCode::F18 => Ok(Key::F18),
			VirtualKeyCode::F19 => Ok(Key::F19),
			VirtualKeyCode::F20 => Ok(Key::F20),
			VirtualKeyCode::F21 => Ok(Key::F21),
			VirtualKeyCode::F22 => Ok(Key::F22),
			VirtualKeyCode::F23 => Ok(Key::F23),
			VirtualKeyCode::F24 => Ok(Key::F24),
			VirtualKeyCode::Snapshot => Ok(Key::Snapshot),
			VirtualKeyCode::Scroll => Ok(Key::ScrollLock),
			VirtualKeyCode::Pause => Ok(Key::Pause),
			VirtualKeyCode::Insert => Ok(Key::Insert),
			VirtualKeyCode::Home => Ok(Key::Home),
			VirtualKeyCode::Delete => Ok(Key::Delete),
			VirtualKeyCode::End => Ok(Key::End),
			VirtualKeyCode::PageDown => Ok(Key::PageDown),
			VirtualKeyCode::PageUp => Ok(Key::PageUp),
			VirtualKeyCode::Left => Ok(Key::Left),
			VirtualKeyCode::Up => Ok(Key::Up),
			VirtualKeyCode::Right => Ok(Key::Right),
			VirtualKeyCode::Down => Ok(Key::Down),
			VirtualKeyCode::Back => Ok(Key::Back),
			VirtualKeyCode::Return => Ok(Key::Return),
			VirtualKeyCode::Space => Ok(Key::Space),
			VirtualKeyCode::Compose => Err(()),
			VirtualKeyCode::Caret => Err(()),
			VirtualKeyCode::Numlock => Ok(Key::Numlock),
			VirtualKeyCode::Numpad0 => Ok(Key::Numpad0),
			VirtualKeyCode::Numpad1 => Ok(Key::Numpad1),
			VirtualKeyCode::Numpad2 => Ok(Key::Numpad2),
			VirtualKeyCode::Numpad3 => Ok(Key::Numpad3),
			VirtualKeyCode::Numpad4 => Ok(Key::Numpad4),
			VirtualKeyCode::Numpad5 => Ok(Key::Numpad5),
			VirtualKeyCode::Numpad6 => Ok(Key::Numpad6),
			VirtualKeyCode::Numpad7 => Ok(Key::Numpad7),
			VirtualKeyCode::Numpad8 => Ok(Key::Numpad8),
			VirtualKeyCode::Numpad9 => Ok(Key::Numpad9),
			VirtualKeyCode::NumpadAdd => Ok(Key::NumpadPlus),
			VirtualKeyCode::NumpadDivide => Ok(Key::NumpadSlash),
			VirtualKeyCode::NumpadDecimal => Err(()),
			VirtualKeyCode::NumpadComma => Err(()),
			VirtualKeyCode::NumpadEnter => Ok(Key::NumpadEnter),
			VirtualKeyCode::NumpadEquals => Err(()),
			VirtualKeyCode::NumpadMultiply => Ok(Key::NumpadAsterisk),
			VirtualKeyCode::NumpadSubtract => Ok(Key::NumpadMinus),
			VirtualKeyCode::AbntC1 => Err(()),
			VirtualKeyCode::AbntC2 => Err(()),
			VirtualKeyCode::Apostrophe => Ok(Key::Apostrophe),
			VirtualKeyCode::Apps => Err(()),
			VirtualKeyCode::Asterisk => Err(()),
			VirtualKeyCode::At => Err(()),
			VirtualKeyCode::Ax => Err(()),
			VirtualKeyCode::Backslash => Ok(Key::Backslash),
			VirtualKeyCode::Calculator => Err(()),
			VirtualKeyCode::Capital => Ok(Key::CapitalLock),
			VirtualKeyCode::Colon => Err(()),
			VirtualKeyCode::Comma => Ok(Key::Comma),
			VirtualKeyCode::Convert => Err(()),
			VirtualKeyCode::Equals => Ok(Key::Equals),
			VirtualKeyCode::Grave => Ok(Key::Grave),
			VirtualKeyCode::Kana => Err(()),
			VirtualKeyCode::Kanji => Err(()),
			VirtualKeyCode::LAlt => Ok(Key::LAlt),
			VirtualKeyCode::LBracket => Ok(Key::LBracket),
			VirtualKeyCode::LControl => Ok(Key::LControl),
			VirtualKeyCode::LShift => Ok(Key::LShift),
			VirtualKeyCode::LWin => Ok(Key::LWin),
			VirtualKeyCode::Mail => Err(()),
			VirtualKeyCode::MediaSelect => Err(()),
			VirtualKeyCode::MediaStop => Err(()),
			VirtualKeyCode::Minus => Ok(Key::Minus),
			VirtualKeyCode::Mute => Err(()),
			VirtualKeyCode::MyComputer => Err(()),
			VirtualKeyCode::NavigateForward => Err(()),
			VirtualKeyCode::NavigateBackward => Err(()),
			VirtualKeyCode::NextTrack => Err(()),
			VirtualKeyCode::NoConvert => Err(()),
			VirtualKeyCode::OEM102 => Err(()),
			VirtualKeyCode::Period => Ok(Key::Period),
			VirtualKeyCode::PlayPause => Err(()),
			VirtualKeyCode::Plus => Err(()),
			VirtualKeyCode::Power => Err(()),
			VirtualKeyCode::PrevTrack => Err(()),
			VirtualKeyCode::RAlt => Ok(Key::RAlt),
			VirtualKeyCode::RBracket => Ok(Key::RBracket),
			VirtualKeyCode::RControl => Ok(Key::RControl),
			VirtualKeyCode::RShift => Ok(Key::RShift),
			VirtualKeyCode::RWin => Ok(Key::RWin),
			VirtualKeyCode::Semicolon => Ok(Key::Semicolon),
			VirtualKeyCode::Slash => Ok(Key::Slash),
			VirtualKeyCode::Sleep => Err(()),
			VirtualKeyCode::Stop => Err(()),
			VirtualKeyCode::Sysrq => Err(()),
			VirtualKeyCode::Tab => Ok(Key::Tab),
			VirtualKeyCode::Underline => Err(()),
			VirtualKeyCode::Unlabeled => Err(()),
			VirtualKeyCode::VolumeDown => Err(()),
			VirtualKeyCode::VolumeUp => Err(()),
			VirtualKeyCode::Wake => Err(()),
			VirtualKeyCode::WebBack => Err(()),
			VirtualKeyCode::WebFavorites => Err(()),
			VirtualKeyCode::WebForward => Err(()),
			VirtualKeyCode::WebHome => Err(()),
			VirtualKeyCode::WebRefresh => Err(()),
			VirtualKeyCode::WebSearch => Err(()),
			VirtualKeyCode::WebStop => Err(()),
			VirtualKeyCode::Yen => Err(()),
			VirtualKeyCode::Copy => Err(()),
			VirtualKeyCode::Paste => Err(()),
			VirtualKeyCode::Cut => Err(()),
		}
	}
}
