# Kreo Hive 98 SignalRGB Device Plugin

Custom hardware plugin enabling full per-key RGB synchronization for the **Kreo Hive 98** mechanical keyboard in [SignalRGB](https://signalrgb.com/).

**Author:** Gourav Kumar  
**GitHub:** [@Joyboycoffee](https://github.com/Joyboycoffee)

---

## Supported Hardware

- **Device:** Kreo Hive 98 Mechanical Gaming Keyboard (98-Key 96% Layout)
- **Vendor ID (VID):** `0x320F`
- **Product ID (PID):** `0x5055`
- **Endpoint Interface:** `1`
- **Endpoint Usage:** `0x0092`

---

## Versions Available

| File | Status | Description |
| :--- | :--- | :--- |
| `kreo_hive_98_v2.js` | **Recommended (V2)** | Fully resolved physical matrix map matching the 18x6 canvas layout with exact key alignments for the number pad, navigation keys, and arrow cluster. |
| `kreo_hive_98.js` | Legacy (V1.3) | Initial standalone fix resolving official array length mismatches and packet wrapper errors. |

---

## Technical Highlights & Reverse Engineering

### 1. EVision Protocol Checksum
The Kreo Hive 98 runs on an EVision microcontroller controller firmware that requires a specific header structure and checksum formula:
```javascript
// Command (0x12 = 18) + bytesToSend + bsLow + bsHigh + dataSum
const calculatedValue = 18 + bytesToSend + bsLow + bsHigh + packetSum;
const high = (calculatedValue >>> 8) & 0xFF;
const low = calculatedValue & 0xFF;
```

### 2. Physical Layout & Matrix Dimensions
- **Canvas Size:** `18 x 6` grid
- **Total Keys Mapped:** 98 individual addressable LEDs
- Maps standard alphanumeric rows, top function row, dedicated arrow cluster, and complete 10-key numeric keypad without missing indices or layout misalignments.

---

## Installation

1. Navigate to your SignalRGB user plugins folder:
   ```
   %USERPROFILE%\Documents\WhirlwindFX\Plugins\Kreo\
   ```
   *(Create the `Kreo` directory if it does not already exist).*
2. Copy `kreo_hive_98_v2.js` into that directory.
3. Restart **SignalRGB** or go to **Settings &rarr; Plugins &rarr; Reload Plugins**.
4. Open the **Devices** tab in SignalRGB: **Kreo Hive 98 V2** will appear automatically and can be positioned on your canvas layout!
