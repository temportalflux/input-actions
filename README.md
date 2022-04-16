# input-actions

WARNING: This crate is still being proven out and is very unstable, and will be until v1.0.0. You cannot rely on its API to be stable between versions and it should not be used for production code. Documentation is subject to being very out of date at any given time until first major release.

input-actions is a Rust crate heavily inspired by Unity library [Rewired](https://assetstore.unity.com/packages/tools/utilities/rewired-21676) ([Website](https://guavaman.com/projects/rewired/)).

This crate utilizes "gilrs" (which uses "rusty-xinput") to handle gamepad input,
and both of these crates expose potentially spammy levels of logging when devices are connected.
It is recommended you ignore or limit the logging levels of "gilrs" and "rusty_xinput" log targets/modules.
This is being tracked by https://gitlab.com/gilrs-project/gilrs/-/issues/105.

## Setup
input-actions uses a "set it and forget it" approach to system management.
As long as the system stays active for the lifecycle of the application,
and its [`update`](System::update) is called at regular intervals,
the rest is pretty hands-off.

```rust
let mut input_sys = System::new();
input_sys
	// There is only 1 player/user to send inputs for.
	.add_users(1)
	// These action names are complete up to you.
	// It is recommended that you store the strings as static properties
	// so they can be referenced throughout the consuming crate.
	.add_action("button1", Action::new(Kind::Button))
	.add_action("button2", Action::new(Kind::Button))
	.add_action("axis1", Action::new(Kind::Axis))
	.add_action("axis2", Action::new(Kind::Axis))
	// This specifies that there is 1 layout (the default layout, which is equivalent to `None`).
	.add_layout(LayoutId::default())
	// This adds bindings for each action for a given layout.
	// The group of bindings per layout is called an "action set".
	.add_action_set(
		// This specifies the name of the set. `ActionSetId::default()` is equivalent to `None`.
		ActionSetId::default(),
		ActionSet::default().with(
			// This action set contains 1 layout, the default layout added to the system above.
			LayoutId::default(),
			ActionMap::default()
				.bind(
					"button1",
					vec![
						binding::Source::Keyboard(source::Key::Return).bound(),
						binding::Source::Keyboard(source::Key::NumpadEnter).bound(),
						binding::Source::Gamepad(GamepadKind::DualAxisGamepad,
							binding::Source::Button(source::Button::VirtualConfirm
						)).bound(),
					],
				)
				.bind(
					"button2",
					vec![
						binding::Source::Keyboard(source::Key::Escape).bound(),
						binding::Source::Gamepad(GamepadKind::DualAxisGamepad,
							binding::Source::Button(source::Button::VirtualDeny)
						).bound(),
					],
				)
				.bind(
					"axis1",
					vec![
						binding::Source::Keyboard(source::Key::W).with_modifier(1.0),
						binding::Source::Keyboard(source::Key::S).with_modifier(-1.0),
						binding::Source::Gamepad(GamepadKind::DualAxisGamepad,
							binding::Source::Axis(source::Axis::LThumbstickX)
						).bound(),
					],
				)
				.bind(
					"axis2",
					vec![
						binding::Source::Keyboard(source::Key::A).with_modifier(-1.0),
						binding::Source::Keyboard(source::Key::D).with_modifier(1.0),
						binding::Source::Gamepad(GamepadKind::DualAxisGamepad,
							binding::Source::Axis(source::Axis::LThumbstickY)
						).bound(),
					],
				)
		),
	)
	// In order to use action set bindings, the user needs the action set enabled.
	// This call says "all users should have this action set enabled",
	// though it is equivalent to `mark_action_set_enabled` since there is only 1 user in this example.
	.enable_action_set_for_all(ActionSetId::default());
```

From there it is up to you to determine when and how to send the system updates
so it knows what actions are in what state.

You should call [`System::update`](System::update) during your update loop
(which will update the state of all actions for all users). This is primarily for
gamepad input polling and updating actions that need simulation.

The input-actions system also needs to know about mouse and keyboard input,
which can be supplied by any external source. If you are using [winit](https://crates.io/crates/winit),
you can enable the `winit` feature,
`input-actions = { version = "...", features = ["winit"] }`
and use the code below to sent window-based gameplay events:
```rust
event_loop.run(move |event, _, _| {
	if let Ok(input_event) = input::winit::parse_winit_event(&event) {
		input_sys.send_event(input_event);
	}
}
```

The input-actions system also supports logging via the `log` feature:
`input-actions = { version = "...", features = ["log"] }`

