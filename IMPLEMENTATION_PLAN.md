# GEN10 DIP Switch Simulator - Implementation Plan

## Project Overview

A terminal-based interactive simulator that visualizes an 8-switch DIP switch bank and displays the resulting GEN10 configuration in real-time using the `ratatui` crate.

---

## Visual Design

### Mock UI - Mixed HD/SD Example

```
┌─────────────────── GEN10 DIP Switch Simulator ───────────────────┐
│                                                                   │
│                          DIP SWITCHES                             │
│                                                                   │
│                              1                                    │
│      SD         [ ○─────────── ]           HD                     │
│                        OUT 1-4                                    │
│                                                                   │
│                              2                                    │
│      SD         [ ───────────● ]           HD                     │
│                        OUT 5-6                                    │
│                                                                   │
│                              3                                    │
│      BLK        [ ───────────● ]         BARS                     │
│                        SD OUT                                     │
│                                                                   │
│                              4                                    │
│    59.94        [ ○─────────── ]           50                     │
│                        FORMAT                                     │
│                                                                   │
│                              5                                    │
│     1080        [ ○─────────── ]          720                     │
│                        HD FMT                                     │
│                                                                   │
│                              6                                    │
│                 [ ○─────────── ]                                  │
│                        S1                                         │
│                                                                   │
│                              7                                    │
│                 [ ○─────────── ]                                  │
│                        S2                                         │
│                                                                   │
│                              8                                    │
│                 [ ○─────────── ]                                  │
│                        S3                                         │
│                                                                   │
├───────────────────── CURRENT CONFIGURATION ──────────────────────┤
│                                                                   │
│  Group 1 (Out 1-4):  HD - 1080i59.94 (1080psf29.97)              │
│  Group 2 (Out 5-6):  SD - 525i (NTSC)                            │
│  SD Output:          75% Color Bars                              │
│  Audio (AES-11):     Tone                                        │
│                                                                   │
│  Video Format Bitmask: 0b00000000  (0x00)                        │
│                          87654321                                 │
│                                                                   │
├──────────────────────── KEYBINDINGS ─────────────────────────────┤
│  [1-8] Toggle Switch  |  [R] Reset  |  [Q] Quit                 │
└───────────────────────────────────────────────────────────────────┘
```

### Switch Visual Style: Circle Toggle with Fill

**Switch States:**
```
LEFT (OFF):                    RIGHT (ON):
┌──────────────────┐          ┌──────────────────┐
│  ○               │          │               ● │
│  ─────────────   │          │  ─────────────  │
└──────────────────┘          └──────────────────┘
  Gray circle                   Green filled circle
  DarkGray track               DarkGray track
```

**Animation States (smooth slide):**
```
25%:                50%:                75%:
┌──────────────┐   ┌──────────────┐   ┌──────────────┐
│   ◐          │   │      ◐       │   │         ◐    │
└──────────────┘   └──────────────┘   └──────────────┘
```

---

## Color Scheme

### Switch Colors
- **LEFT (OFF):** Gray circle `Color::Rgb(128, 128, 128)` on DarkGray track
- **RIGHT (ON):** Green filled circle `Color::Green` on DarkGray track
- **Animating:** Gradient from Gray to Green as it slides

### Configuration Display Colors
- **Group 1/2 Headers:** `Color::Cyan`
- **HD Format:** `Color::Green`
- **SD Format:** `Color::Yellow`
- **Video Format Bitmask:** `Color::White` for binary, `Color::Cyan` for hex

### Label Colors
- Switch numbers: `Color::White`
- Function labels: `Color::Gray`
- Active labels (for RIGHT switches): `Color::Green`

---

## Architecture & Components

### File Structure

```
gen10ds/
├── Cargo.toml
├── DIPSWITCHES.md
├── IMPLEMENTATION_PLAN.md
├── .gitignore
└── src/
    ├── main.rs              # Terminal init, 60 FPS event loop, cleanup
    ├── app.rs               # App state, SwitchAnimation, event handling
    ├── dipswitch.rs         # DipSwitch model, bitmask, OutputConfig
    ├── format_calculator.rs # SD/HD format calculation with Table 2
    └── ui.rs                # Canvas-based UI rendering with colors
```

### Dependencies (Cargo.toml)

```toml
[package]
name = "gen10ds"
version = "0.1.0"
edition = "2024"

[dependencies]
ratatui = "0.29"
crossterm = "0.28"
```

---

## Data Models

### src/dipswitch.rs

```rust
pub struct DipSwitch {
    switches: [bool; 8], // false = LEFT, true = RIGHT
}

pub struct OutputConfig {
    pub group1_format: String,  // Independent format for outputs 1-4
    pub group2_format: String,  // Independent format for outputs 5-6
    pub sd_video: SdVideo,
    pub audio: AudioOutput,
}

pub enum SdVideo {
    ColorBlack,
    ColorBars75,
}

pub enum AudioOutput {
    Silent,
    Tone,
}
```

**Methods:**
- `DipSwitch::new()` - All switches default to LEFT (false)
- `DipSwitch::toggle(index: usize)` - Toggle switch 0-7 (maps to 1-8)
- `DipSwitch::reset()` - Reset all to default
- `DipSwitch::get(index: usize) -> bool` - Get switch state
- `DipSwitch::get_config() -> OutputConfig` - Calculate current configuration
- `DipSwitch::get_bitmask() -> u8` - Calculate 8-bit bitmask
- `DipSwitch::get_bitmask_binary() -> String` - Format as "0b00001101"
- `DipSwitch::get_bitmask_hex() -> String` - Format as "0x0D"

**Video Format Bitmask:**
- Only includes switches 4-8 (video format related switches)
- Masked with `0b1111_1000` to exclude sync and SD output switches (1-3)
- Bit 3 = Switch 4 (FORMAT)
- Bit 4 = Switch 5 (HD FMT)
- Bit 5 = Switch 6 (S1)
- Bit 6 = Switch 7 (S2)
- Bit 7 = Switch 8 (S3)
- Example: Switches 4=ON, 5=ON → `0b00011000` = 0x18

---

## Format Calculation Logic

### src/format_calculator.rs

```rust
pub fn calculate_group_format(
    sync_switch: bool,       // false = SD, true = HD
    sw4_format: bool,        // false = 525/NTSC, true = 625/PAL
    sw5_hd_fmt: bool,        // false = 1080, true = 720
    sw6_s1: bool,
    sw7_s2: bool,
    sw8_s3: bool,
) -> String
```

### Format Logic

**SD Mode (sync_switch = false):**
- Switch 4 LEFT: `"SD - 525i (NTSC)"`
- Switch 4 RIGHT: `"SD - 625i (PAL)"`

**HD Mode (sync_switch = true):**
- Calculate from Table 2 using switches 4, 5, 6, 7, 8
- Prefix result with `"HD - "`
- Examples:
  - `"HD - 1080i59.94 (1080psf29.97)"`
  - `"HD - 720p50"`
  - `"HD - 1080p24"`

### Table 2 Implementation

Binary value from switches 6, 7, 8 (S1, S2, S3):
```rust
let s1_s2_s3 = (sw6_s1 as u8) | ((sw7_s2 as u8) << 1) | ((sw8_s3 as u8) << 2);
```

**Complete Table 2 Lookup:**

| SW4 | SW5 | S3S2S1 | Format |
|-----|-----|--------|--------|
| 0 (59.94) | 0 (1080) | 000 | 1080i59.94 (1080psf29.97) |
| 0 (59.94) | 0 (1080) | 001 | 1080psf23.98 |
| 0 (59.94) | 0 (1080) | 010 | 1080p23.98 |
| 0 (59.94) | 0 (1080) | 011 | 1080p29.97 |
| 0 (59.94) | 0 (1080) | 100 | 1080i60 (1080psf30) |
| 0 (59.94) | 0 (1080) | 101 | 1080psf24 |
| 0 (59.94) | 0 (1080) | 110 | 1080p24 |
| 0 (59.94) | 0 (1080) | 111 | 1080p30 |
| 0 (59.94) | 1 (720) | 000 | 720p59.94 |
| 0 (59.94) | 1 (720) | 001 | 720p23.98 |
| 0 (59.94) | 1 (720) | 010 | 720p23.98 |
| 0 (59.94) | 1 (720) | 011 | 720p29.97 |
| 0 (59.94) | 1 (720) | 100 | 720p60 |
| 0 (59.94) | 1 (720) | 101 | 720p24 |
| 0 (59.94) | 1 (720) | 110 | 720p24 |
| 0 (59.94) | 1 (720) | 111 | 720p30 |
| 1 (50) | 0 (1080) | 000 | 1080i50 |
| 1 (50) | 0 (1080) | 001 | 1080i50 |
| 1 (50) | 0 (1080) | 010 | 1080p25 |
| 1 (50) | 0 (1080) | 011 | 1080p25 |
| 1 (50) | 0 (1080) | 100 | 1080i50 (1080psf25) |
| 1 (50) | 0 (1080) | 101 | 1080i50 |
| 1 (50) | 0 (1080) | 110 | 1080p25 |
| 1 (50) | 0 (1080) | 111 | 1080p25 |
| 1 (50) | 1 (720) | 000 | 720p50 |
| 1 (50) | 1 (720) | 001 | 720p25 |
| 1 (50) | 1 (720) | 010 | 720p25 |
| 1 (50) | 1 (720) | 011 | 720p25 |
| 1 (50) | 1 (720) | 100 | 720p50 |
| 1 (50) | 1 (720) | 101 | 720p25 |
| 1 (50) | 1 (720) | 110 | 720p25 |
| 1 (50) | 1 (720) | 111 | 720p25 |

---

## Critical Logic Rules

1. **SD Override:**
   - When Switch 1 is LEFT → Group 1 shows SD format (switches 5-8 ignored)
   - When Switch 2 is LEFT → Group 2 shows SD format (switches 5-8 ignored)
   - SD format only considers Switch 4 for NTSC vs PAL

2. **HD Calculation:**
   - Only when Switch 1 is RIGHT → Group 1 uses Table 2
   - Only when Switch 2 is RIGHT → Group 2 uses Table 2
   - Both groups use the same switches 4-8, so when both are in HD mode, they show identical formats

3. **Format Strings:**
   - SD: `"SD - 525i (NTSC)"` or `"SD - 625i (PAL)"`
   - HD: `"HD - [format]"` where format comes from Table 2

4. **Switch 3 Dual Control:**
   - LEFT: Color Black + Silent
   - RIGHT: 75% Color Bars + Tone

---

## Animation System

### src/app.rs

```rust
pub struct App {
    dipswitch: DipSwitch,
    animations: [SwitchAnimation; 8],
    should_quit: bool,
    last_frame_time: Instant,
}

pub struct SwitchAnimation {
    target_state: bool,
    current_position: f64,  // 0.0 = LEFT, 1.0 = RIGHT
    animating: bool,
}

impl SwitchAnimation {
    const ANIMATION_DURATION: f64 = 0.15; // 150ms for smooth feel
    const ANIMATION_SPEED: f64 = 1.0 / Self::ANIMATION_DURATION;
    
    fn new(initial_state: bool) -> Self {
        Self {
            target_state: initial_state,
            current_position: if initial_state { 1.0 } else { 0.0 },
            animating: false,
        }
    }
    
    fn start_animation(&mut self, new_state: bool) {
        self.target_state = new_state;
        self.animating = true;
    }
    
    fn update(&mut self, delta_time: f64) {
        if !self.animating {
            return;
        }
        
        let target = if self.target_state { 1.0 } else { 0.0 };
        let direction = (target - self.current_position).signum();
        
        self.current_position += direction * Self::ANIMATION_SPEED * delta_time;
        
        // Clamp and check if animation complete
        if direction > 0.0 && self.current_position >= target {
            self.current_position = target;
            self.animating = false;
        } else if direction < 0.0 && self.current_position <= target {
            self.current_position = target;
            self.animating = false;
        }
    }
}
```

---

## Canvas Rendering

### src/ui.rs

```rust
use ratatui::widgets::canvas::{Canvas, Circle, Line, Shape};
use ratatui::symbols::Marker;

fn render_switch_canvas(
    switch_num: u8,
    position: f64,  // 0.0 to 1.0
    state: bool,
) -> impl Widget {
    Canvas::default()
        .marker(Marker::Braille)  // High resolution for smooth circles
        .x_bounds([0.0, 100.0])
        .y_bounds([0.0, 50.0])
        .paint(move |ctx| {
            // Draw track (horizontal line)
            ctx.draw(&Line {
                x1: 10.0,
                y1: 25.0,
                x2: 90.0,
                y2: 25.0,
                color: Color::DarkGray,
            });
            
            // Calculate circle position based on animation
            let circle_x = 10.0 + (position * 80.0);
            
            // Determine fill amount and color based on position
            let fill_amount = position;  // 0.0 to 1.0
            let color = Color::Rgb(
                (0.0 + fill_amount * 0.0) as u8,      // Red: 0 -> 0
                (128.0 + fill_amount * 127.0) as u8,  // Green: 128 -> 255
                (128.0 - fill_amount * 128.0) as u8,  // Blue: 128 -> 0
            );
            // Result: Gray (128,128,128) -> Green (0,255,0)
            
            // Draw circle
            ctx.draw(&Circle {
                x: circle_x,
                y: 25.0,
                radius: 8.0,
                color,
            });
            
            // Draw filled portion of circle (simulate fill effect)
            if fill_amount > 0.01 {
                // Draw multiple smaller circles to create fill effect
                for r in (0..=(8.0 * fill_amount) as u32).step_by(2) {
                    ctx.draw(&Circle {
                        x: circle_x,
                        y: 25.0,
                        radius: r as f64,
                        color: Color::Green,
                    });
                }
            }
        })
}
```

### Switch Labels

From DIP Switch Quick Reference:

| Switch | Left Label | Center Label | Right Label |
|--------|------------|--------------|-------------|
| 1 | SD | OUT 1-4 | HD |
| 2 | SD | OUT 5-6 | HD |
| 3 | BLK | SD OUT | BARS |
| 4 | 59.94 | FORMAT | 50 |
| 5 | 1080 | HD FMT | 720 |
| 6 | (none) | S1 | (none) |
| 7 | (none) | S2 | (none) |
| 8 | (none) | S3 | (none) |

---

## Event Loop & Main Application

### src/main.rs

```rust
impl App {
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        let target_fps = 60;
        let frame_duration = Duration::from_millis(1000 / target_fps);
        let mut last_frame = Instant::now();
        
        while !self.should_quit {
            // Calculate delta time
            let now = Instant::now();
            let delta = now.duration_since(last_frame).as_secs_f64();
            last_frame = now;
            
            // Update animations
            self.update_animations(delta);
            
            // Render
            terminal.draw(|frame| self.draw(frame))?;
            
            // Handle input (with timeout to maintain framerate)
            let timeout = frame_duration.saturating_sub(last_frame.elapsed());
            if event::poll(timeout)? {
                self.handle_events()?;
            }
        }
        
        Ok(())
    }
    
    fn update_animations(&mut self, delta_time: f64) {
        for animation in &mut self.animations {
            animation.update(delta_time);
        }
    }
    
    fn handle_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('1'..='8') => {
                let idx = (key as u8 - b'1') as usize;
                let new_state = !self.dipswitch.get(idx);
                self.dipswitch.toggle(idx);
                self.animations[idx].start_animation(new_state);
            }
            KeyCode::Char('r') | KeyCode::Char('R') => {
                self.dipswitch.reset();
                for anim in &mut self.animations {
                    anim.start_animation(false);
                }
            }
            KeyCode::Char('q') | KeyCode::Char('Q') => {
                self.should_quit = true;
            }
            _ => {}
        }
    }
}
```

---

## Implementation Phases

### Phase 1: Core Data Model (src/dipswitch.rs)
- [ ] Implement `DipSwitch` struct with 8-bool array
- [ ] Add `toggle()`, `reset()`, `get()` methods
- [ ] Implement bitmask calculation methods
- [ ] Define `OutputConfig`, `SdVideo`, `AudioOutput` enums
- [ ] Add tests for DipSwitch logic

### Phase 2: Format Calculator (src/format_calculator.rs)
- [ ] Implement `calculate_group_format()` function
- [ ] Add SD format logic (525i NTSC / 625i PAL)
- [ ] Implement full Table 2 lookup (32 combinations)
- [ ] Add tests for all format combinations

### Phase 3: Animation System (src/app.rs)
- [ ] Implement `SwitchAnimation` struct
- [ ] Add animation state tracking (position, target)
- [ ] Implement smooth linear interpolation
- [ ] Add delta-time based updates

### Phase 4: Canvas Rendering (src/ui.rs)
- [ ] Create `render_switch_canvas()` function
- [ ] Implement circle drawing with Canvas widget
- [ ] Add track line rendering
- [ ] Implement color gradient during animation (gray to green)
- [ ] Add fill effect for circle

### Phase 5: UI Layout (src/ui.rs)
- [ ] Create main layout structure (switches, config, keybindings)
- [ ] Render 8 switches with labels
- [ ] Display configuration section with color coding
- [ ] Render bitmask display with alignment
- [ ] Add keybindings help bar

### Phase 6: Event Loop & Integration (src/main.rs, src/app.rs)
- [ ] Setup terminal with ratatui + crossterm
- [ ] Implement 60 FPS render loop with delta time
- [ ] Add key event handling (1-8, r, q)
- [ ] Connect animations to switch toggles
- [ ] Add proper terminal cleanup

### Phase 7: Polish & Testing
- [ ] Test all 32 HD format combinations
- [ ] Verify SD override behavior for both groups
- [ ] Test animation smoothness at different terminal sizes
- [ ] Verify bitmask calculation for all states
- [ ] Test reset functionality
- [ ] Add error handling
- [ ] Performance optimization if needed

---

## Testing Checklist

- [ ] Switch 1 LEFT forces Group 1 to SD (525i or 625i only)
- [ ] Switch 2 LEFT forces Group 2 to SD (525i or 625i only)
- [ ] Switch 4 correctly toggles between 525i (NTSC) and 625i (PAL) in SD mode
- [ ] Switch 1 RIGHT enables HD calculation for Group 1
- [ ] Switch 2 RIGHT enables HD calculation for Group 2
- [ ] When both in HD, both groups show same format
- [ ] All 32 Table 2 combinations work correctly
- [ ] Switch 3 controls both SD output and audio together
- [ ] Visual switch indicator shows smooth animation
- [ ] Color gradient animates from gray to green
- [ ] Keys 1-8 toggle correctly
- [ ] Reset (R) returns all switches to LEFT with animation
- [ ] Quit (Q) exits cleanly
- [ ] Bitmask calculation is correct for all states
- [ ] Bitmask alignment is correct (87654321 aligned with bits after 'b')

---

## Technical Notes

1. **Marker Choice:** Using `Marker::Braille` for switches gives highest resolution for smooth circles (2×4 pixels per character cell)

2. **Animation Timing:** 150ms animation duration provides responsive feel without being jarring

3. **Frame Rate:** 60 FPS ensures smooth animation and responsive input

4. **Color Interpolation:** RGB color transitions during animation for visual feedback

5. **Circle Fill Effect:** Multiple overlapping circles with decreasing radius to simulate solid fill

6. **Video Format Bitmask:** Only shows switches 4-8 (masked with `0b1111_1000`) as these are the switches that determine video format. Switches 1-3 control sync and SD output, not the video format itself.

7. **Edition 2024:** Cargo.toml specifies edition "2024" as per user preference

---

## Example Configuration States

### Default State (All Switches LEFT)
```
Video Format Bitmask: 0b00000000 (0x00)
Group 1 (Out 1-4):  SD - 525i (NTSC)
Group 2 (Out 5-6):  SD - 525i (NTSC)
SD Output:          Color Black
Audio (AES-11):     Silent
```

### Example: Mixed HD/SD with Color Bars
```
Switches: 1=RIGHT, 2=LEFT, 3=RIGHT, 4-8=LEFT
Video Format Bitmask: 0b00000000 (0x00)
Group 1 (Out 1-4):  HD - 1080i59.94 (1080psf29.97)
Group 2 (Out 5-6):  SD - 525i (NTSC)
SD Output:          75% Color Bars
Audio (AES-11):     Tone
```

### Example: Both Groups HD PAL 720p50
```
Switches: 1=RIGHT, 2=RIGHT, 3=LEFT, 4=RIGHT, 5=RIGHT, 6-8=LEFT
Video Format Bitmask: 0b00011000 (0x18)
Group 1 (Out 1-4):  HD - 720p50
Group 2 (Out 5-6):  HD - 720p50
SD Output:          Color Black
Audio (AES-11):     Silent
```

---

## References

- **DIPSWITCHES.md** - Complete DIP switch configuration reference from GEN10 hardware documentation
- **ratatui Canvas Widget** - https://ratatui.rs/examples/widgets/canvas/
- **ratatui Documentation** - https://docs.rs/ratatui/latest/ratatui/

---

## End of Implementation Plan
