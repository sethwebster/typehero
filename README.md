# TypeHero

Terminal typing trainer focused on **accuracy first**, with data-driven practice targeting your specific problem areas.

## Philosophy

- **Accuracy > Speed**: Raw WPM is meaningless without accuracy
- **No escape hatches**: Can't skip hard exercises or hide mistakes
- **Data-driven**: Every keystroke tracked, analyzed, and used to generate targeted practice
- **Immediate feedback**: Real-time visual indicators for every character
- **Adjusted WPM**: Primary metric = raw WPM × accuracy²

## Features

- **Real-time feedback**: See mistakes instantly as you type
- **Bigram tracking**: Identifies your slowest character pairs
- **Error analysis**: Tracks most error-prone keys
- **Targeted practice**: Auto-generates exercises from your problem areas
- **Persistent stats**: All sessions saved to `~/.typehero_stats.json`
- **Multiple modes**:
  - Random words (common English)
  - Code patterns (programming constructs)
  - Targeted drills (your problem bigrams)
  - Quick practice (20 words)

## Installation

```bash
cargo install --path .
```

Or run directly:
```bash
cargo run --release
```

## Usage

Launch the app and select a mode:

1. **Guided Practice** - Constraint-based finger placement training
   - **Teaches through constraints, not detection**: Only keys from active fingers are accepted
   - Progressive finger unlocking: Index → Middle → Ring → Pinky → Thumbs
   - ASCII keyboard map shows active fingers in color (inactive keys greyed out)
   - Illegal keys rejected (cursor doesn't advance) - shapes motor memory through negative capability
   - **3-2-1 countdown** before each attempt (no Enter key friction)
   - Tracks illegal key violations + accuracy per attempt
   - Mastery = 3 consecutive attempts at 95%+ accuracy with 0 illegal keys
   - Auto-advances when lesson mastered
   - Navigate: Ctrl-N (next lesson), Ctrl-P (previous lesson)
2. **Random Words** - Practice common English words
3. **Code Patterns** - Programming-specific patterns
4. **Targeted Practice** - Auto-generated from your slowest bigrams
5. **Quick Drill** - Short 20-word session
6. **View Statistics** - See lifetime stats and problem areas
7. **Quit**

### During Practice

- Type exactly what you see
- **No backspace** - mistakes stay visible
- ESC to quit (you'll get a reminder to push through if quitting early)
- Session ends when you complete the text

### Metrics

- **Raw WPM**: (characters / 5) / minutes
- **Accuracy**: (correct_chars / total_chars) × 100
- **Adjusted WPM**: Raw WPM × (accuracy/100)²

Example: 60 WPM at 90% accuracy = 60 × 0.81 = 48.6 adjusted WPM

## Rules

1. **No backspace** - Forces correct typing first time
2. **95% accuracy target** - Anything less = needs improvement
3. **Stats never reset** - Own your progress

## Tips for Improvement

1. **Slow down** - Speed comes from accuracy, not rushing
2. **Focus on problem bigrams** - Use targeted practice mode
3. **Watch the current character** - It's highlighted in blue
4. **Don't look at your hands** - Force muscle memory
5. **Consistent practice** - 15 minutes daily > 2 hours weekly

## Stats Location

All statistics are saved to `~/.typehero_stats.json`

Format:
- Session summaries (timestamp, WPM, accuracy)
- Total keystrokes and errors
- Per-key error rates
- Bigram timing data

## Architecture

See `AGENTS.md` for detailed architecture and design principles.
