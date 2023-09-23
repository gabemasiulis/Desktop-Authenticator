# Desktop Authenticator Device
### Purpose
The objective of this project is to teach myself rust lang, while also making something useful for my own desk/development workspace.
I'm continually annoyed by having to open my cellphone and pull up an authenticator app to get a Time-based One Time Password (TOTP) for a login service.

I'd like to build an ESP32-based device consisting of an e-ink display, a battery, and a rotary encoder. The idea is that this device can rotate through different services and display the current TOTP for a particular service.

Skateboard/MVP of this may just have TOTP secrets defined at compile time. Long term goal would be to authenticate with a web api to ensure the device is still active, and to fetch hmac secrets. Either way, I think web connectivity is a must, at the very least to sync with an NTP server to provide accurate time.

The device will need to be able to store wifi ssid and password info, the web server/api should be able to recognize/allow devices by MAC or other identifying information. A simple input for wifi SSID and Password should be all that is necessary. A single rotary encoder (for example KY-040) with push button switch should be enough input to handle this.