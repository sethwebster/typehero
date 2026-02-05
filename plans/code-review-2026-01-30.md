# Code Review: uninstall-openclaw.sh

**File**: `/Users/sethwebster/Development/typehero/uninstall-openclaw.sh`
**Date**: 2026-01-30
**Reviewer**: Claude Opus 4.5

---

## Executive Summary

The script has solid fundamentals (`set -euo pipefail`, dry-run support, confirmation prompts) but contains **critical security vulnerabilities** in path handling and **dangerous shell config modifications** that could break user environments. The `rm -rf` operations lack sufficient safeguards against path injection and symlink attacks.

---

## Critical Issues

### 1. Path Injection in `safe_remove()` via Environment Variable

**Location**: Lines 196-219 (`find_git_checkouts`) and line 118 (`safe_remove`)

**Problem**: `OPENCLAW_GIT_DIR` is read from environment without any validation or canonicalization. An attacker (or misconfigured env) could set:
```bash
OPENCLAW_GIT_DIR="/etc"
OPENCLAW_GIT_DIR="/"
OPENCLAW_GIT_DIR="/home/user/../../etc"
```

The checks for `.git` directory and `package.json` provide some protection, but:
1. If someone creates a malicious `/evil/.git` and `/evil/package.json` with matching content
2. Path traversal (`../`) bypasses intended directory scope

**Impact**: Arbitrary directory deletion with user privileges

**Solution**:
```bash
validate_path() {
    local path="$1"
    local resolved

    # Reject empty paths
    [[ -z "$path" ]] && return 1

    # Reject paths outside HOME
    resolved="$(cd "$path" 2>/dev/null && pwd -P)" || return 1

    case "$resolved" in
        "$HOME"/*) return 0 ;;
        *)
            echo -e "${ERROR}Refusing to remove path outside HOME: $resolved${NC}" >&2
            return 1
            ;;
    esac
}
```

### 2. `rm -rf` on Glob Patterns Without Safeguards

**Location**: Lines 277-281

```bash
for pattern in ".openclaw-*" "openclaw"; do
    local target="${npm_root}/${pattern}"
    if [[ -e "$target" || -L "$target" ]]; then
        safe_remove "$target" "npm cache/package"
    fi
done
```

**Problem**: The `target` variable contains a literal glob pattern (`.openclaw-*`), which is then passed to `rm -rf`. The glob expansion happens in `rm`, not in the test. However, if `npm_root` contains spaces or special characters, this becomes unpredictable.

**Impact**: Potential unintended file deletion if npm_root is unusual

**Solution**: Enumerate files explicitly:
```bash
if [[ -n "$npm_root" && -d "$npm_root" ]]; then
    while IFS= read -r -d '' target; do
        safe_remove "$target" "npm cache/package"
    done < <(find "$npm_root" -maxdepth 1 \( -name ".openclaw-*" -o -name "openclaw" \) -print0 2>/dev/null)
fi
```

### 3. Symlink Following in `safe_remove()`

**Location**: Lines 102-124

**Problem**: `rm -rf` follows symlinks by default for the target itself. If `$path` is a symlink to another directory, `rm -rf` removes the link, not the target. However, the check `[[ ! -e "$path" && ! -L "$path" ]]` doesn't verify that symlinks point where expected.

A symlink at `~/.openclaw` pointing to `/` would not delete `/`, but a symlink at `~/openclaw` pointing to `~/important-project` would delete it if it happens to have a `.git` and matching `package.json`.

**Impact**: Data loss through symlink manipulation

**Solution**: Add symlink detection and resolution:
```bash
safe_remove() {
    local path="$1"
    local desc="$2"

    if [[ ! -e "$path" && ! -L "$path" ]]; then
        return 0
    fi

    # Warn on symlinks
    if [[ -L "$path" ]]; then
        local target
        target="$(readlink "$path")"
        echo -e "${WARN}!${NC} ${INFO}${path}${NC} is a symlink to ${INFO}${target}${NC}"
        if ! prompt_yn "Remove symlink only (not target)?" "n"; then
            return 0
        fi
        # Remove symlink only, not target
        if [[ "$DRY_RUN" != "1" ]]; then
            rm "$path"  # No -rf, just remove symlink
        fi
        return 0
    fi
    # ... rest of function
}
```

---

## Architecture Concerns

### 4. Shell Config Modification is Too Aggressive

**Location**: Lines 304-311

```bash
for rc in "$HOME/.bashrc" "$HOME/.zshrc" "$HOME/.bash_profile" "$HOME/.profile"; do
    if remove_line_from_file "$rc" ".npm-global" "npm global PATH entry"; then
    if remove_line_from_file "$rc" ".local/bin" "local bin PATH entry"; then
```

**Problem**: These patterns (`.npm-global`, `.local/bin`) are common PATH additions that many tools use, not just OpenClaw. Removing ALL lines containing these strings will break:
- nvm, fnm, volta (Node version managers)
- pipx, poetry (Python tools)
- cargo (Rust)
- Any user's custom PATH modifications

**Impact**: Breaks user's shell environment for unrelated tools

**Solution**: Use more specific patterns that the installer actually writes:
```bash
# Only remove lines with OpenClaw-specific comments
remove_line_from_file "$rc" "# OpenClaw" "OpenClaw PATH entry"
# Or use exact line matching if installer writes a known pattern:
remove_line_from_file "$rc" 'export PATH=.*\.npm-global.*# added by openclaw' "OpenClaw npm PATH"
```

### 5. Missing Cleanup: Launchd/Systemd Services

**Location**: `stop_daemon()` only calls `openclaw daemon stop`

**Problem**: If the installer registers a launchd plist (`~/Library/LaunchAgents/`) or systemd user unit (`~/.config/systemd/user/`), these would persist and attempt to start a now-deleted binary.

**Impact**: Zombie service entries, confusing error messages on login

**Solution**:
```bash
remove_service_files() {
    # macOS launchd
    local plist="$HOME/Library/LaunchAgents/com.openclaw.daemon.plist"
    if [[ -f "$plist" ]]; then
        launchctl unload "$plist" 2>/dev/null || true
        safe_remove "$plist" "launchd service definition"
    fi

    # Linux systemd
    local unit="$HOME/.config/systemd/user/openclaw.service"
    if [[ -f "$unit" ]]; then
        systemctl --user disable openclaw 2>/dev/null || true
        systemctl --user stop openclaw 2>/dev/null || true
        safe_remove "$unit" "systemd service unit"
        systemctl --user daemon-reload 2>/dev/null || true
    fi
}
```

### 6. Missing Cleanup: npm Link / Workspaces

**Problem**: If the installer used `npm link` from a git checkout (common for development installs), there's a symlink in npm's global `node_modules` pointing to the source. `npm uninstall -g openclaw` may not remove this if it was linked differently.

**Solution**: Check for and remove npm link:
```bash
uninstall_npm_global() {
    # Also handle npm link scenarios
    local npm_root
    npm_root="$(npm root -g 2>/dev/null || true)"
    if [[ -n "$npm_root" && -L "$npm_root/openclaw" ]]; then
        echo -e "${WARN}->>${NC} Found npm link"
        npm unlink -g openclaw 2>/dev/null || true
    fi
    # ... existing logic
}
```

---

## DRY Opportunities

### 7. Repeated Color/Status Pattern

**Location**: Throughout (lines 110-123, 139-154, 162-173, etc.)

**Problem**: Every function has the same pattern:
```bash
echo -e "${WARN}->>${NC} Found: ..."
if [[ "$DRY_RUN" == "1" ]]; then
    echo -e "${MUTED}   [dry-run] Would ...${NC}"
    return 0
fi
if some_action; then
    echo -e "${SUCCESS}tick${NC} Done"
else
    echo -e "${ERROR}x${NC} Failed"
fi
```

**Solution**: Extract to helper:
```bash
run_action() {
    local desc="$1"
    local dry_run_msg="$2"
    shift 2

    echo -e "${WARN}->>${NC} ${desc}"

    if [[ "$DRY_RUN" == "1" ]]; then
        echo -e "${MUTED}   [dry-run] ${dry_run_msg}${NC}"
        return 0
    fi

    if "$@"; then
        echo -e "${SUCCESS}tick${NC} Done"
    else
        echo -e "${ERROR}x${NC} Failed: $desc"
        return 1
    fi
}

# Usage:
run_action "Found npm global install" "Would uninstall" npm uninstall -g openclaw
```

---

## Maintenance Improvements

### 8. Silent Failure in Argument Parser

**Location**: Lines 64-75

```bash
*) shift ;;  # Silently ignores unknown args
```

**Problem**: Unknown arguments are silently ignored. User typos (`--dryrun` instead of `--dry-run`) proceed without warning.

**Solution**:
```bash
*)
    echo -e "${ERROR}Unknown option: $1${NC}" >&2
    print_usage >&2
    exit 1
    ;;
```

### 9. No Logging for Audit Trail

**Problem**: Uninstall actions have no persistent record. If something goes wrong, there's no way to determine what was deleted.

**Solution**:
```bash
LOG_FILE="${TMPDIR:-/tmp}/openclaw-uninstall-$(date +%Y%m%d-%H%M%S).log"

log() {
    echo "[$(date -Iseconds)] $*" >> "$LOG_FILE"
}

# In safe_remove:
log "REMOVE: $path ($desc)"

# At end:
echo -e "${MUTED}Log saved to: $LOG_FILE${NC}"
```

### 10. Bash-Specific Syntax Without Fallback

**Location**: Line 99

```bash
[[ "${answer,,}" == "y" || "${answer,,}" == "yes" ]]
```

**Problem**: `${var,,}` (lowercase) is Bash 4+ only. macOS ships with Bash 3.2. The shebang is `#!/bin/bash` which on macOS points to ancient Bash.

**Impact**: Script fails on stock macOS

**Solution**:
```bash
# POSIX-compatible lowercase
answer_lower="$(echo "$answer" | tr '[:upper:]' '[:lower:]')"
[[ "$answer_lower" == "y" || "$answer_lower" == "yes" ]]
```

Or use `#!/usr/bin/env bash` and document Bash 4+ requirement.

### 11. Missing Temporary File Cleanup on Interrupt

**Location**: Lines 147-154

```bash
local tmp="${file}.tmp.$$"
if grep -v "$pattern" "$file" > "$tmp" && mv "$tmp" "$file"; then
```

**Problem**: If script is interrupted (Ctrl+C) between creating temp file and moving it, orphan `.tmp.$$` files remain.

**Solution**:
```bash
# At top of script
TEMP_FILES=()
cleanup_temp() {
    for f in "${TEMP_FILES[@]}"; do
        rm -f "$f" 2>/dev/null || true
    done
}
trap cleanup_temp EXIT

# In remove_line_from_file:
local tmp="${file}.tmp.$$"
TEMP_FILES+=("$tmp")
```

---

## Nitpicks

### 12. Inconsistent Exit Codes

Functions return 1 on failure but `main` doesn't check these returns. With `set -e`, some failures will exit, others won't (those in pipelines or conditionals).

### 13. Color Variables Could Use tput

The hardcoded ANSI codes won't work in all terminals. `tput` is more portable:
```bash
if [[ -t 1 ]]; then
    BOLD="$(tput bold)"
    NC="$(tput sgr0)"
    # etc.
else
    BOLD="" NC="" # No colors in pipes
fi
```

### 14. Missing Version/Checksum

No way to verify which version of the uninstaller is running. Add:
```bash
UNINSTALLER_VERSION="1.0.0"
echo -e "${MUTED}Uninstaller version: ${UNINSTALLER_VERSION}${NC}"
```

---

## Strengths

1. **`set -euo pipefail`** at the top is the correct defensive posture
2. **Dry-run mode** is well-implemented and consistent
3. **Confirmation prompt** with TTY detection prevents accidental CI runs
4. **Graceful degradation** when commands don't exist (`|| true` patterns)
5. **Legacy config cleanup** shows attention to migration paths
6. **Keep-config option** provides escape hatch for users who want partial uninstall
7. **Clear user communication** about what system deps remain
8. **No sudo** - everything runs as user, limiting blast radius

---

## Summary of Required Changes

| Priority | Issue | Effort |
|----------|-------|--------|
| CRITICAL | Path injection via OPENCLAW_GIT_DIR | Medium |
| CRITICAL | Shell config patterns too broad | Low |
| HIGH | Missing launchd/systemd cleanup | Medium |
| HIGH | Bash 4+ syntax on macOS | Low |
| MEDIUM | Symlink handling | Medium |
| MEDIUM | Temp file cleanup on interrupt | Low |
| LOW | DRY refactor for action pattern | Medium |
| LOW | Argument parser validation | Low |

The script should not be deployed until the CRITICAL issues are resolved. The shell config modification issue in particular could break user environments in ways that are difficult to diagnose.
