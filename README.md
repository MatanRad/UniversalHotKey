# Universal Hot Key
Universal Hot Key is a simple  desktop automation tool designed to bring the core functionality of AutoHotKey to other platforms such as Linux and MacOS.

**Please Note:** This project is early in development and as such, it only implements a subset of the functionality of AHK and its scripting language.

## What's Currently in UHK?
Currently UHK implements a very simple "scripting language" which allows the user to define hotkeys & functions that can either be called manually or executed when the given hotkey is pressed.

The scripting language currently only allows for simulating key-presses and string typing (the `Send`, `SendRaw` commands as they're in the AHK language). But later on the language will support standard expressions and more desktop automation features.

Later this project will also have automatic installation scripts.

### Command Set
The following is a list of commands the UHK language currently supports:
* `Log` - Prints the given string to `stdout` 
* `Call` - Calls the requested function
* `Return` - Stops execution of the current function. This needs to be placed at the end of every hotkey/function.
* `Send` - Types a hotkey/keypress
	* Example: `Send +s` - Sends `Shift + S`.
	* Example: `Send {ENTER}{TAB}` - Sends Enter and then Tab.
* `SendRaw` - Types the given string without parsing its content (except for needed shift presses).
	* Example: `SendRaw +s` - Types the literal characters `plus` and `s`
	* Example: `SendRaw {ENTER}` - Types `{ENTER}` literally.
* `Sleep` - Sleeps for the given number of milliseconds.

## Usage
UHK finds its scripts via the configuration file at `/etc/uhk.json`.
The structure of the configuration file can be found at the [Installation](#Installation) section.

### Scripting
UHK's language is designed to be very similar to AHK's language.
It's best to learn by example so below is a script that does 2 things:
* Defines a `rick_roll` function and a hotkey `CTRL + WINKEY + R` that calls it
	* The function opens a terminal (assuming system `WINKEY + T` shortcut) and types a command that opens YouTube on the "Never Gonna Give You Up" music video.
* Defines a shortcut `CTRL + SHIFT + M` that types out an email.

```UHK
rick_roll { 
    Log "Commencing the Rick Roll!"
    Send #t 
    Sleep 1500
    SendRaw open "https://www.youtube.com/watch?v=dQw4w9WgXcQ"
    Send {ENTER}

    return
}

^#r::
    Call rick_roll
	return

^+m::
    Send example@email.com
	return
```

## Installation
**Note:** Currently UHK does not have a quick-installation script so it needs to be "installed manually". An installation script will later be added.

UHK is a single executable that needs to be run in the background of your PC and it can be done in any way you want.

The recommended way is to make a script that runs it and then use `systemctl` (or `launchctl` on MacOS) in order to keep UHK running on startup.

### Linux
On Linux, UHK gets its inputs directly from the keyboard device via the `/etc/input` filesystem. 

By default, it searches for one itself but we can also supply it with a custom path by overriding the `UHK_DEVICE` environment variable.

Some keyboard (Razer Blackwidow V2 for example) map 2 devices but only one of them actually sends the keyboard inputs.

Below is an example of a wrapper script that runs UHK with the correct parameters:
```bash
#!/bin/bash

# This script is ran from systemctl, meaning it doesn't have its own X11 Display, so we tell him what is the desktop display & authority.
export DISPLAY=:1 
export XAUTHORITY=/run/user/1000/gdm/Xauthority
export UHK_DEVICE=/dev/input/by-id/usb-Razer_Razer_BlackWidow_Chroma_V2-if01-event-kbd

# This is the actual UHK executable.
/usr/bin/universal-hot-key
```

### MacOS
Install using the same method as for Linux (i.e. wrapper script + `launchctl`).

The MacOS version of UHK doesn't use X11 nor does it read inputs from `/dev`. It uses Apple's Core Graphics framework to receive and send keypresses.
UHK needs accessibility permissions in order to read keyboard input and simulate keyboard presses.
On the first execution, your Mac will notify you of this and will instruct you to either permit it or disallow it. At that point you can allow it via the system preferences.