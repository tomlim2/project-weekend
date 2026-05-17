---
status: complete
type: milestone
---

# Milestone: Weekend Emoji Runner MVP

## Goal

Build a Three.js mini level that opens from `index.html` without a build
step. The player completes the level in 30-60 seconds.

## IP Boundary

| Allowed | Blocked |
|---|---|
| Side-view movement | Mario name in game UI |
| Block platforms | Mario character likeness |
| Coin-like pickups | Mario enemy likeness |
| Goal flag | Mario sound, music, sprites, level layouts |

## Player Promise

Move an emoji character through a short side-view level, collect coins,
avoid hazards, and touch the flag.

## Definition of Done

| ID | Requirement |
|---|---|
| DOD-01 | `index.html` opens in a modern browser without a build step. |
| DOD-02 | Player is visible as an emoji sprite. |
| DOD-03 | `A/D` moves horizontally. |
| DOD-04 | `W/S` moves vertically. |
| DOD-05 | Movement is direct 2D plane movement; MVP does not use gravity or jump arcs. |
| DOD-06 | Camera follows the player through a level spanning 3-4 screen widths. |
| DOD-07 | Level contains ground/platform blocks, 8 coins, 3 hazards, and 1 goal flag. |
| DOD-08 | At least 5 coins sit on the natural start-to-flag route. |
| DOD-09 | Coin pickups update the HUD within the same frame. |
| DOD-10 | Hazard contact causes a loss state. |
| DOD-11 | Goal contact causes a win state. |
| DOD-12 | `R` restarts after win or loss. |
| DOD-13 | README explains run command, controls, and status. |
| DOD-14 | `CONCEPT.md` records scope, stages, acceptance, and exclusions. |

## Non-Goals

- Exact Mario clone, Mario assets, copyrighted character likeness, or
  trademarked names.
- Full platformer physics, jump arcs, stomp combat, enemies with AI,
  multiple levels, mobile controls, audio, save data, or level editor.

## Build Plan

| Stage | Scope | Acceptance | Status |
|---|---|---|---|
| A | Scene, camera, renderer, emoji player, keyboard input | Player moves with `A/D/W/S` | complete |
| B | Handmade level blocks and camera follow | Player crosses 3-4 screen widths from start to goal area | complete |
| C | Coins, hazards, goal, HUD, restart loop | Full win/loss loop playable | complete |
| Polish | Visual readability, emoji texture quality, viewport fit | No text overlap, blank canvas, or cropped player at 375x667 and 1280x720 | complete |

## Verification

```sh
cd prototypes/weekend-emoji-runner
python3 -m http.server 5177
```

Open `http://127.0.0.1:5177/`.

| Check | Pass condition |
|---|---|
| Console | No runtime errors during load and manual play. |
| Movement | `A/D/W/S` each move the player in the documented direction. |
| Coin | Collect 1 coin and see HUD count increment. |
| Loss | Touch 1 hazard and see loss overlay. |
| Restart | Press `R` and see position, timer, coins, and overlay reset. |
| Win | Reach flag and see win overlay. |
