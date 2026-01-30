# TypeHero - Typing Trainer Architecture

## Core Principles

### Accuracy Over Speed
- WPM means nothing if accuracy is poor
- Never advance until accuracy threshold met (default 95%)
- Penalize mistakes more than reward speed
- Show accuracy-adjusted WPM (raw WPM * accuracy²)

### No Escape Hatches
- Can't skip difficult exercises
- Can't reset stats to hide poor performance
- Encourages pushing through discomfort (quit warning at <50% completion)
- Mistakes force additional practice of problem patterns

### Immediate Feedback
- Visual indicator on every keystroke (correct/incorrect)
- Real-time WPM and accuracy display
- Highlight current character to type
- Show mistake immediately, don't allow backspace to hide it

### Data-Driven Practice
- Track every keystroke with timestamp
- Identify slowest key pairs (bigrams)
- Identify most error-prone keys/patterns
- Generate targeted exercises from mistakes

### Constraint-Based Teaching (Guided Practice)
**The Philosophy**: Typing tutors know fingers the same way compilers know intent — by defining what is allowed, enforcing constraints, and shaping behavior over time.

**Key Insight**: A CLI tutor never observes your hands. Instead, it defines a finger-key contract and refuses progress when you violate it. Think compiler, not camera.

**The Finger-Key Mapping**:
```
Left Pinky:   ` q a z
Left Ring:    w s x
Left Middle:  e d c
Left Index:   r f v t g b
Right Index:  y h n u j m
Right Middle: i k ,
Right Ring:   o l .
Right Pinky:  p ; /
Thumbs:       space
```

**Enforcement, Not Detection**:
- Lessons define `allowed_keys: HashSet<char>` based on active fingers
- Input filter: `if !lesson.is_key_allowed(ch) { continue; }`
- Illegal keys don't advance cursor, increment `illegal_keys` counter
- No message saying "wrong finger" — only "that key isn't part of this exercise"
- The constraint shapes motor memory through **negative capability**

**Why This Works**:
1. **Cheating is unstable** - Requires conscious override, breaks rhythm, causes inconsistency
2. **Later phases expose it** - Speed + interleaving + rhythm constraints collapse cheating
3. **Motor learning hates instability** - Brain associates correct motion with success

**Progressive Unlocking**:
- Phase 1: Slow, isolated finger practice
- Phase 2: Remove backspace (from other modes)
- Phase 3: Rhythm constraints, speed requirements
- System where cheating becomes harder than correctness

**Making It Feel Intelligent** (Without Hardware):
- Track key-to-key transition timing
- Flag latency spikes (likely finger reach)
- Identify uneven inter-key timing patterns
- Never claim certainty, only nudge: "Right ring finger seems unstable"

**Why This Beats Camera Detection**:
- No privacy issues
- Works over SSH
- No setup friction
- Teaches habits, not compliance
- Scales universally
- Typing skill is about constraints, not surveillance

## Architecture

### Core Components

#### 1. Input Handler
- Raw terminal mode (no line buffering)
- Capture keystrokes with precise timing
- No backspace - mistakes stay visible
- Record every key with timestamp

#### 2. Exercise Engine
- Load text prompts (common words, code, prose)
- Generate targeted exercises from mistake patterns
- Calculate real-time metrics
- Enforce accuracy thresholds

#### 3. Statistics Tracker
- Per-session stats (WPM, accuracy, duration)
- Lifetime aggregate stats
- Per-key error rates
- Bigram speed map
- Persist to JSON file

#### 4. Display Manager
- TUI with clear visual feedback
- Current character highlighted
- Typed text with mistakes marked
- Live stats panel
- Progress bar

#### 5. Practice Modes
- **Guided Practice**: Constraint-based finger placement training
  - **Philosophy**: "Typing tutors don't detect fingers — they enforce finger ownership"
  - **Constraint model**: Defines allowed key set per lesson, rejects illegal keys
  - Input filtering: Only keys from active finger(s) accepted; illegal keys don't advance cursor
  - Progressive finger unlocking:
    - Phase 1: Index fingers (home row anchors)
    - Phase 2: Add middle fingers
    - Phase 3: Add ring fingers
    - Phase 4: Add pinkies
    - Phase 5: Add space bar (thumbs)
  - ASCII keyboard visualization: Active fingers in color, inactive greyed out
  - Text generation respects constraints: No spaces until thumbs unlocked
  - **Mastery requirements**: 3 consecutive attempts at 95%+ accuracy with 0 illegal keys
  - Auto-advances when lesson mastered
  - 3-2-1 countdown before attempts (no Enter key friction)
  - Tracks: time, accuracy, errors, illegal_keys per attempt
  - **The constraint IS the teacher** - shapes motor memory through negative capability
- **Random Words**: Common English words
- **Code**: Programming patterns (braces, operators, camelCase)
- **Targeted**: Generated from your worst bigrams/keys
- **Custom**: Paste your own text

### State Management

```
Session {
  text: String,
  typed: Vec<(char, timestamp, correct: bool)>,
  start_time: Instant,
  errors: HashMap<char, usize>,
  bigram_times: HashMap<(char, char), Duration>,
}

Stats {
  sessions: Vec<SessionSummary>,
  total_keys: usize,
  total_errors: usize,
  key_errors: HashMap<char, usize>,
  slowest_bigrams: Vec<((char, char), Duration)>,
}
```

### Dependencies

- `crossterm`: Terminal control, raw mode, events
- `serde` + `serde_json`: Stats persistence
- `rand`: Text generation
- `chrono`: Timestamps

### File Structure

```
src/
  main.rs           - CLI entry, mode selection
  engine.rs         - Core typing test loop
  guided.rs         - Guided practice with finger placement
  stats.rs          - Statistics tracking/persistence
  display.rs        - TUI rendering
  exercises.rs      - Text generation
  input.rs          - Keystroke capture (includes Ctrl-N/Ctrl-P)
```

### Metrics

#### Raw WPM
`(total_chars / 5) / (elapsed_minutes)`

#### Accuracy
`(correct_chars / total_chars) * 100`

#### Adjusted WPM (primary metric)
`raw_wpm * (accuracy / 100)²`

Example: 60 WPM at 90% accuracy = 60 * 0.81 = 48.6 adjusted WPM

### Rules

1. **No backspace during tests** - Forces you to type correctly first time
2. **Push through discomfort** - Quit warning if <50% complete (growth happens when you persist)
3. **Can't advance until 95% accuracy** - Prevents rushing
4. **Mistakes generate targeted drills** - Every error = 10 reps of that pattern
5. **Stats never reset** - Own your progress

### Anti-Patterns to Avoid

- ❌ Allowing backspace (hides real accuracy)
- ❌ Short tests (no statistical significance)
- ❌ Random text without mistake tracking
- ❌ Showing only WPM without accuracy
- ❌ Optional exercises (you'll skip hard ones)

### Success Metrics

- Accuracy > 95% consistently
- Adjusted WPM > 60
- No bigram > 200ms
- No single key > 10% error rate
