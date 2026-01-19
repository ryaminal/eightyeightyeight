# Track Specification: Enhance Wizard Device Discovery

## 1. Overview
This track upgrades the `eightyeightyeight init` wizard to strictly use GStreamer APIs for hardware discovery. Instead of blindly listing `/dev/video*`, it will use `gst::DeviceMonitor` to find video sources and query their supported capabilities (resolution, framerate, formats).

## 2. Goals
- **Better UX:** Display human-readable device names (e.g., "Logitech Webcam C920") alongside device paths.
- **Accuracy:** Only list devices that GStreamer can actually use.
- **Capabilities Discovery:** Eliminate guessing for resolution and framerate by presenting a selectable list of supported modes.

## 3. Key Features
- **Device Monitor:** Use `gst::DeviceMonitor` to filter for `Video/Source`.
- **Caps Parsing:** Extract struct properties (width, height, framerate) from the device's `caps`.
- **Selection UI:**
    1. Select Device (by name).
    2. Select Mode (Resolution/FPS combination).
- **Fallback:** Retain manual entry option for headless or weird setups.

## 4. Technical Approach
- **Initialize GStreamer:** Ensure `gst::init()` is called before the wizard runs.
- **Device Probe:** Start a `DeviceMonitor`, filter for "Video/Source", and collect `Device` objects.
- **Caps Enumeration:** For the selected device, get its `caps`. Iterate over the structures to find unique combinations of `width`, `height`, and `framerate`.
- **Inquire Select:** Use `Select` prompts for both device and mode selection.

## 5. Success Criteria
- Wizard lists actual device names.
- Wizard lists valid resolutions/framerates for the selected device.
- Generated `config.toml` contains values guaranteed to be supported by the hardware.
