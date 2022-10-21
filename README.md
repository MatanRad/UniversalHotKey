# Universal Hot Key
Universal Hot Key is a simple  desktop automation tool designed to bring the core functionality of AutoHotKey to other platforms such as Linux and MacOS.

**Please Note:** This project is early in development and as such, it only implements a subset of the functionality of AHK and its scripting language.

## What's Currently in UHK?
Currently UHK implements a very simple "scripting language" which allows the user to define hotkeys & functions that can either be called manually or executed when the given hotkey is pressed.

The scripting language currently only allows for simulating key-presses and string typing (the `Send`, `SendRaw` commands as they're in the AHK language). But later on the language will support standard expressions and more desktop automation features.

### Command Set
The following is a list of commands the UHK language currently supports:
* `Log` - Prints the given string to `stdout` 
* `Call` - Calls the requested function
* `Return` - Stops execution of the current function
* `Send` - Types a hotkey/keypress
	* Example: `Send +s` - Sends `Shift + S`.
	* Example: `Send {ENTER}{TAB}` - Sends Enter and then Tab.
* `SendRaw` - Types the given string without parsing its content (except for needed shift presses).
	* Example: `SendRaw +s` - Types the literal characters `plus` and `s`
	* Example: `SendRaw {ENTER}` - Types `{ENTER}` literally.
* `Sleep` - Sleeps for the given number of milliseconds.

