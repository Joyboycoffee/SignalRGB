export function Name() { return "Kreo Hive 98 V2"; }
export function VendorId() { return 0x320F; }
export function ProductId() { return 0x5055; }
export function Publisher() { return "Kreo"; }
export function Size() { return [18, 6]; } 
export function DeviceType() { return "keyboard"; }
export function ImageUrl() { return "https://assets.signalrgb.com/devices/brands/kreo/keyboards/hive98.png"; }

export function Validate(endpoint) { 
    return endpoint.interface === 1 && endpoint.usage === 0x0092; 
}

var vLedNames = [
    "Esc", "F1", "F2", "F3", "F4", "F5", "F6", "F7", "F8", "F9", "F10", "F11", "F12", "Del", "Insert", "PgUp", "PgDn",
    "`", "1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "-_", "=+", "Backspace", "Num Lock", "Num /", "Num *", "Num -",
    "Tab", "Q", "W", "E", "R", "T", "Y", "U", "I", "O", "P", "[", "]", "\\", "Num 7", "Num 8", "Num 9", "Num +",
    "CapsLock", "A", "S", "D", "F", "G", "H", "J", "K", "L", ";", "'", "Enter", "Num 4", "Num 5", "Num 6",
    "Left Shift", "Z", "X", "C", "V", "B", "N", "M", ",", ".", "/", "Right Shift", "Up Arrow", "Num 1", "Num 2", "Num 3", "Num Enter",
    "Left Ctrl", "Left Win", "Left Alt", "Space", "Right Alt", "Fn", "Right Ctrl", "Left Arrow", "Down Arrow", "Right Arrow", "Num 0", "Num Del"
];

// Fully resolved V2 hardware matrix layout map
var vLeds = [
    0, 8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96, 104, 114, 122, 115,
    1, 9, 17, 25, 33, 41, 49, 57, 65, 73, 81, 89, 97, 105, 6, 14, 22, 30,
    2, 10, 18, 26, 34, 42, 50, 58, 66, 74, 82, 90, 98, 106, 38, 46, 54, 86,
    3, 11, 19, 27, 35, 43, 51, 59, 67, 75, 83, 91, 107, 62, 70, 78,
    4, 20, 28, 36, 44, 52, 60, 68, 76, 84, 92, 108, 116, 124, 94, 102, 126,
    5, 13, 21, 45, 77, 85, 101, 109, 117, 125, 110, 118
];

var vLedPositions = [
    [0, 0], [1, 0], [2, 0], [3, 0], [4, 0], [5, 0], [6, 0], [7, 0], [8, 0], [9, 0], [10, 0], [11, 0], [12, 0], [13, 0], [14, 0], [15, 0], [16, 0],
    [0, 1], [1, 1], [2, 1], [3, 1], [4, 1], [5, 1], [6, 1], [7, 1], [8, 1], [9, 1], [10, 1], [11, 1], [12, 1], [13, 1], [14, 1], [15, 1], [16, 1], [17, 1],
    [0, 2], [1, 2], [2, 2], [3, 2], [4, 2], [5, 2], [6, 2], [7, 2], [8, 2], [9, 2], [10, 2], [11, 2], [12, 2], [13, 2], [14, 2], [15, 2], [16, 2], [17, 2],
    [0, 3], [1, 3], [2, 3], [3, 3], [4, 3], [5, 3], [6, 3], [7, 3], [8, 3], [9, 3], [10, 3], [11, 3],          [13, 3], [14, 3], [15, 3], [16, 3],
    [0, 4],         [2, 4], [3, 4], [4, 4], [5, 4], [6, 4], [7, 4], [8, 4], [9, 4], [10, 4], [11, 4], [12, 4], [13, 4], [14, 4], [15, 4], [16, 4], [17, 4],
    [0, 5], [1, 5], [2, 5],                         [6, 5],                         [10, 5], [11, 5], [12, 5], [13, 5], [14, 5], [15, 5], [16, 5], [17, 5]
];

export function LedNames() { return vLedNames; }
export function LedPositions() { return vLedPositions; }

export function Initialize() {
    device.write([0x04, 0x8c, 0x00, 0x0b, 0x30, 0x50, 0x01], 64);
    device.pause(20);
}

export function Render() {
    sendColors();
}

export function Shutdown() {
    sendColors("#000000");
}

function sendColors(overrideColor) {
    const RGBData = new Array(127 * 3).fill(0);

    const loopCount = Math.min(vLeds.length, vLedPositions.length);

    for (let iIdx = 0; iIdx < loopCount; iIdx++) {
        const position = vLedPositions[iIdx];
        if (!position) { continue; }
        
        // Clamps layout lookups securely within the 18x6 canvas dimensions
        let iPxX = Math.min(Math.max(position[0], 0), 17);
        let iPxY = Math.min(Math.max(position[1], 0), 5);
        
        let color = overrideColor ? [0, 0, 0] : (device.color(iPxX, iPxY) || [0, 0, 0]);

        const targetAddress = vLeds[iIdx] * 3;
        if (targetAddress + 2 < RGBData.length) {
            RGBData[targetAddress]     = color[0];
            RGBData[targetAddress + 1] = color[1];
            RGBData[targetAddress + 2] = color[2];
        }
    }

    writeRGBPackage(RGBData);
}

function writeRGBPackage(RGBData) {
    let bytesSentIndex = 0;

    while (RGBData.length > 0) {
        const bytesToSend = Math.min(56, RGBData.length);
        const data = RGBData.splice(0, bytesToSend);

        const packetSum = data.reduce((sum, num) => sum + num, 0);
        
        const bsHigh = (bytesSentIndex >>> 8) & 0xFF;
        const bsLow = bytesSentIndex & 0xFF;

        // Unified EVision checksum formula: Command (0x12) + bytesToSend + bsLow + bsHigh + dataSum thank me later it took me a lot of time to figure this out 
        const calculatedValue = 18 + bytesToSend + bsLow + bsHigh + packetSum;

        const high = (calculatedValue >>> 8) & 0xFF;
        const low = calculatedValue & 0xFF;

        const header = [0x04, low, high, 0x12, bytesToSend, bsLow, bsHigh, 0x00];
        
        // Pad with zeros to fill the 64-byte USB HID report size (8 bytes header + 56 bytes data) this is important to ensure the device receives the full packet took me a whole night to figure this out 
        const padding = new Array(56 - bytesToSend).fill(0x00);
        const packet = header.concat(data).concat(padding);

        device.write(packet, 64);
        
        // Flat 4ms spacing between packets (minimum hardware threshold) with no trailing pause
        if (RGBData.length > 0) {
            device.pause(4);
        }

        bytesSentIndex += bytesToSend;
    }
}