// USB device descriptors taken from https://git.io/JI4nK
pub const PRODUCT_ID: u16 = 0x01ed;
pub const VENDOR_ID: u16 = 0x27b8;

// taken from blink1-tool: https://git.io/JeWXW (canon: https://git.io/JeWXl)
pub const HID_SET_REPORT: u8 = 0x09;
pub const HID_FEATURE: u16 = 0x03 << 0x08;

// Full command list can be found at github.com/todbot/blink1/blob/9bec7d35/hardware/firmware_mk2/main.c#L477-L490.
pub const FADE_COMMAND_ACTION: u8 = 0x63; // 'c'
pub const IMMEDIATE_COMMAND_ACTION: u8 = 0x6e; // 'n'
