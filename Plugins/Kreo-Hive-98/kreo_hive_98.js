/* =========================================================
   Kreo Hive 98 - Custom Standalone Fix (v1.3)
   Fixed official developer array length mismatch & packet wrapper
   ========================================================= */

export function Name() { return "Kreo Hive 98 Custom"; }
export function VendorId() { return 0x320F; }
export function ProductId() { return 0x5055; }
export function Publisher() { return "Gourav Kumar"; }
export function Size() { return [18, 6]; }
export function DeviceType() { return "keyboard"; }

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

var vLeds = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
    19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37,
    41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58,
    59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74,
    75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91,
    92, 93, 94, 95, 96, 97, 99, 100, 101, 102, 103, 104
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
    // Corrected to use only the validated EVision mode switch sequence
    device.write([0x04, 0x8c, 0x00, 0x0b, 0x30, 0x50, 0x01], 64);
    device.pause(10);
}

export function Render() {
    sendColors();
}

export function Shutdown() {
    sendColors("#000000");
}

function sendColors(overrideColor) {
    const RGBData = new Array(105 * 3).fill(0);

    // Limit loop boundary strictly to the smallest array to avoid index mismatch crashes
    const maxItems = Math.min(vLeds.length, vLedPositions.length);

    for (let iIdx = 0; iIdx < maxItems; iIdx++) {
        const position = vLedPositions[iIdx];
        if (!position) { continue; } // Ultimate safety fallback

        const iPxX = position[0];
        const iPxY = position[1];
        
        let color;
        if (overrideColor) {
            color = [0, 0, 0];
        } else {
            const fetchedColor = device.color(iPxX, iPxY);
            color = fetchedColor ? fetchedColor : [0, 0, 0];
        }

        const targetAddress = vLeds[iIdx] * 3;
        RGBData[targetAddress]     = color[0];
        RGBData[targetAddress + 1] = color[1];
        RGBData[targetAddress + 2] = color[2];
    }

    writeRGBPackage(RGBData);
}

function writeRGBPackage(RGBData) {
    const bytesToSend = 56;
    const TotalPackets = Math.ceil(RGBData.length / bytesToSend);

    for (let index = 0; index < TotalPackets; index++) {
        const data = RGBData.splice(0, bytesToSend);
        
        while(data.length < bytesToSend) { data.push(0x00); }

        const packetSum = data.reduce((sum, num) => sum + num, 0);
        let calculatedValue = packetSum + (index * bytesToSend) + 74;
        
        if (index >= 5) {
            calculatedValue = packetSum + ((index - 5) * bytesToSend) + 99;
        }

        const high = (calculatedValue >>> 8) & 0xFF;
        const low = calculatedValue & 0xFF;
        const bytesSentIndex = index * bytesToSend;
        const bsHigh = (bytesSentIndex >>> 8) & 0xFF;
        const bsLow = bytesSentIndex & 0xFF;

        const header = [0x04, low, high, 0x12, bytesToSend, bsLow, bsHigh, 0x00];
        const packet = header.concat(data);

        device.write(packet, 64);
        device.pause(1);
    }
}