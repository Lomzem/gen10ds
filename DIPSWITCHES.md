# GEN10 DIP Switch Settings

The combination of switch settings determines the overall operation of the GEN10.

## Table 1. GEN10 DIP Switch Setting Descriptions

| SWITCH | FUNCTION | DIP Set LEFT (default) | DIP Set RIGHT |
|--------|----------|------------------------|---------------|
| 1 | Sync format for outputs 1 through 4 | Select SD sync (SD). | Select HD sync (HD). |
| 2 | Sync format for outputs 5 and 6 | Select SD sync (SD). | Select HD sync (HD). |
| 3 | SD output and AES signals | SD Output is Color Black (BLK) | SD Output is 75% Color Bars (BARS) |
| 3 | SD output and AES signals | AES-11 output is silent | AES-11 output is tone |
| 4 | Frame Rate Format | Select NTSC related frame rates (59.94)<br><br>NOTE: For true 60/24 frame rates see Table 2 below. | Select PAL related frame rates (50) |
| 5 | HD Line Rate Format | Select 1080 line formats (1080) | Select 720 line formats (720) |
| 6, 7, 8 | HD Formats (other) | These three DIP switches (S1, S2, S2) act together with switches S4 and S5 to select other, less common HD formats. Zero (0) is Left position, one (1) is Right position. See Table 2 below for setting information. | |

## DIP Switches 6, 7, 8 (Other HD Formats)

For the most popular HD formats switches 6, 7, and 8 should be set to the default position (all three switches set to LEFT or "000", as listed in the following table). Switches 6, 7, and 8 should only be changed to select other HD formats that are more rarely used. Table 2 shows how to select those formats using switches 4 through 8 in combination.

To use the table, first locate the video format you want in the columns on the right, and then set switches as shown in the corresponding left hand columns.

Both the switch numbers and labels are both shown in the table, with the labels in parentheses (for example, switch 4 is labeled "FORMAT" and switch 8 is labeled "S3"). The SW 8, SW7, and SW 6 switches form a binary value shown in the left hand columns., with the DIP switch direction indicated by:

- 0 = Switch LEFT
- 1 = Switch RIGHT

## Table 2. HD Formats Set with DIP Switches 4 through 8

| SW4 (FORMAT) | | | 59.94 | | 50 | |
|--------------|--------|--------|--------|--------|--------|--------|
| **SW5 (HD FMT)** | | | **1080** | **720** | **1080** | **720** |
| **SW8 (S3)** | **SW7 (S2)** | **SW6 (S1)** | | | | |
| 0 | 0 | 0 | 1080i59.94<br>1080psf29.97 | 720p59.94 | 1080i50 | 720p50 |
| 0 | 0 | 1 | 1080psf23.98 | 720p23.98 | 1080i50 | 720p25 |
| 0 | 1 | 0 | 1080p23.98 | 720p23.98 | 1080p25 | 720p25 |
| 0 | 1 | 1 | 1080p29.97 | 720p29.97 | 1080p25 | 720p25 |
| 1 | 0 | 0 | 1080i60<br>1080psf30 | 720p60 | 1080i50<br>1080psf25 | 720p50 |
| 1 | 0 | 1 | 1080psf24 | 720p24 | 1080i50 | 720p25 |
| 1 | 1 | 0 | 1080p24 | 720p24 | 1080p25 | 720p25 |
| 1 | 1 | 1 | 1080p30 | 720p30 | 1080p25 | 720p25 |

**NOTE:** For 1080psf29.97 use 1080i59.94, for 1080psf30 use 1080i60, and for 1080psf25 use 1080i50.
