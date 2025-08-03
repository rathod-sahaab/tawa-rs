## Components
### Kernel
Kernel is the core component of this application/firmware, where most of the business logic resides. It can be separated into various layers but for project of this size is overkill. Responsibilities of kernel include:
1. Get triggered by user inputs and events.
2. Solicit inputs from devices, in a device agnostic format.
3. Prepare and schedule output data for devices, in a device agnostic format.

#### Prepare output

### Platforms
Embedded world is pretty diverse in terms of availability of micro-controllers, thus for portability of firmware platforms act just as an interface for the kernel.

Following platforms are supported as of now:
[x] STM32
[ ] Raspberry Pi
[ ] ESP32

### Devices
Similar to Platforms, devices are very diverse, even more so to be accurate.

- Displays
    - [x] SSD1306
- Relay
    - [x] Solid State Relay
- Temperature sensor
    - [x] Thermistor
- Input
    - [x] 3 buttons