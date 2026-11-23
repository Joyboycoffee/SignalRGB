# Kreo Hive 98 Keyboard Plugin for SignalRGB

Custom hardware driver providing per-key RGB lighting synchronization for the **Kreo Hive 98** mechanical keyboard.

**Author:** Gourav Kumar  
**GitHub:** [@Joyboycoffee](https://github.com/Joyboycoffee)

---

## Device Information

- **Device:** Kreo Hive 98 Mechanical Gaming Keyboard
- **Vendor ID (VID):** `0x320F`
- **Product ID (PID):** `0x5055`
- **Endpoint Interface:** `1`
- **Endpoint Usage:** `0x0092`
- **Canvas Matrix Size:** `18 x 6` (98 addressable LEDs)

---

## Files

| File | Version | Description |
| :--- | :--- | :--- |
| `kreo_hive_98_v2.js` | **V2 (Recommended)** | Complete hardware matrix layout mapping matching the 18x6 canvas with exact positions for Numpad, Navigation cluster, Arrow keys, and Function row. |
| `kreo_hive_98.js` | V1 (Legacy) | Initial standalone fix resolving official array length mismatches and packet wrapper errors. |

---

## Protocol & Reverse Engineering

The keyboard uses an EVision microcontroller with a dedicated checksum formula for packet transmission:
```javascript
// Command (0x12) + bytesToSend + bsLow + bsHigh + packetSum
const calculatedValue = 18 + bytesToSend + bsLow + bsHigh + packetSum;
const high = (calculatedValue >>> 8) & 0xFF;
const low = calculatedValue & 0xFF;
```

---

## Installation

1. Copy `kreo_hive_98_v2.js` to:
   ```
   %USERPROFILE%\Documents\WhirlwindFX\Plugins\Kreo\
   ```
2. Restart or reload plugins in **SignalRGB** (Settings &rarr; Plugins &rarr; Reload).
3. The keyboard will appear in the **Devices** tab ready to be positioned on your canvas.
