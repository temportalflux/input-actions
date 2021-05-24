# ReBound
-----

ReBound is a Rust crate heavily inspired by Unity library [Rewired](https://assetstore.unity.com/packages/tools/utilities/rewired-21676) (https://guavaman.com/projects/rewired/).

## Terminology

### System

The ReBound System holds the configured users, actions, layouts, and various maps and categories. Consumers of ReBound will configure their "System" object to set up how actions are configured, and User objects will query the system to determine the state of a given Action.

### User

Users are to ReBound as Players are to Rewired. A user is a person using an input device (Controller). They can have multiple Controllers attached to them (where players can switch between one controller to another). Each user also has a layout that they are using and what Map Categories are enabled (both of (which are stored in the System).

### Controller

As in Rewired, a Controller represents a physical (gamepad/joystick/etc) or virtual (keyboard/mouse) device with buttons and/or axes. Controllers cannot be queried on their own, they must be associated with a User. Controllers are essentially the literal devices which input comes from.

### Action

As in Rewired, an Action is a application/consumer facing event which a User can trigger via a Controller. If a consumer of ReBound has access to a User and Action, they can cause gameplay events to happen when actions are triggered. A Controller map its buttons and/or axes to Actions via Controller Maps.

### Action Behavior

Action Behaviors are akin to Input Behaviors in Rewired. They modify the input from a Controller when determining the state of an Action. This modification could be things like the following (among others):
- Simulating a digital axis (for buttons which mapped to axis actions)
- Axis deadzones
- Double-Press button actions
- Controller type-specific sensitivity

### Layout

A layout is a setting that Users use in conjunction with Map Categories to determine which controller maps to use. Layouts can be used to define profiles such as "left-handed" and "southpaw" to change a User's Controller input experience. Each layout has a list of Map Categories & their Controller Maps which handle mapping a controller's buttons and/or axes to Actions.

### Map Category

A map category contains a set of Controller Maps to use when the category is enabled. A category can be enabled/disabled for a given user to turn on/off groups of actions. The example used in Rewired sets forth categories such as "Gameplay Shared", "Infantry", "Vehicle", "Airplane", and "Menu". Any of these categories can be toggled to allow or block input from the actions mapped in the category.

### Controller Map

As in Rewired, a Controller Map handles the connective tissue between Controllers and Actions. Each controller map can associate a button/axis on a Controller to an Action. This is how Users know what controller inputs drive what Actions. Controller Maps are stored in Map Categories so that consumers of ReBound can toggle on/off collections of controller maps all at once.
