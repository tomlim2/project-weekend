---
status: complete
type: spec
milestone: MILESTONE.md
---

# Weekend Emoji Runner

## Pitch

Tiny 2.5D side-view platform toy. An emoji hero crosses one handmade
level, collects coins, dodges hazards, and reaches a flag.

## Milestone

`MILESTONE.md` owns the MVP finish line. This file owns the implementation
spec.

## IP Boundary

| Allowed | Blocked |
|---|---|
| Side-view level traversal | Mario name in game UI |
| Block platforms | Mario character likeness |
| Coin-like pickups | Mario enemy likeness |
| Goal flag | Mario sound, music, sprites, level layouts |

## Player Fantasy

짧은 장난감 레벨을 30~60초 안에 훑는다. 정교한 플랫포머보다
"대충 만든 이모지 캐릭터가 블록 위를 지나가며 코인을 먹고 깃발을
찍는다"는 즉시성에 집중한다.

## Core Loop

이동 → 코인 수집 → 위험물 회피 → 깃발 도착 → 결과 오버레이 →
`R`로 재시작.

## MVP Scope

- Three.js single HTML, no build.
- Side-view 2.5D camera with smooth horizontal follow.
- Emoji character sprite generated from canvas texture.
- `A/D` horizontal movement.
- `W/S` vertical movement.
- Direct 2D movement; no gravity, jump arc, or platform collision solver.
- One handmade level with:
  - ground/block platforms
  - 8 coins
  - 3 hazards
  - 1 goal flag
- Level spans 3-4 screen widths.
- Completion time target: 30-60 seconds on first successful run.
- HUD: coins, elapsed time, control hint.
- Win state on goal contact.
- Loss state on hazard contact.
- Restart with `R`.

## Acceptance Criteria

| ID | Criteria |
|---|---|
| AC-01 | `http://127.0.0.1:5177/` shows a nonblank Three.js scene. |
| AC-02 | Player emoji is visible at the start of the level. |
| AC-03 | `A` moves left and `D` moves right. |
| AC-04 | `W` moves up and `S` moves down. |
| AC-05 | Camera follows the player horizontally without losing the player or goal. |
| AC-06 | Touching a coin hides it and increments the HUD within the same frame. |
| AC-07 | Touching a hazard shows a loss overlay. |
| AC-08 | Touching the flag shows a win overlay. |
| AC-09 | `R` resets position, timer, coins, and overlay state. |
| AC-10 | Browser console has no runtime errors during a manual play pass. |
| AC-11 | 375x667 and 1280x720 viewports show no HUD overlap, blank canvas, or cropped player. |

## Stretch

- Simple jump/gravity mode as an alternate control preset.
- Coin sound, hazard sound, win jingle via Web Audio.
- Second level loaded from a compact JSON array.
- Mobile touch overlay.

## Out of Scope

- Exact Mario clone, Mario art, Mario name usage in the game UI, physics
  engine, stomp combat, power-ups, scrolling tile editor, asset loading,
  mobile controls, multiple levels, music, save data, localStorage.

## Tech Stack

- Three.js `0.169.0` via CDN importmap.
- Single `index.html`.
- Canvas-generated emoji textures.
- CSS HUD / overlay.
- Manual AABB/radius collision checks.

## Run Target

```sh
cd prototypes/weekend-emoji-runner
python3 -m http.server 5177
```

Open `http://127.0.0.1:5177/`.

## Build Stages

| Stage | Goal | Verification | Status |
|---|---|---|---|
| A | Scene, camera, renderer, emoji player, keyboard input | Player moves with `A/D/W/S` | complete |
| B | Handmade level blocks and camera follow | Player can cross 3-4 screen widths from start to goal area | complete |
| C | Coins, hazards, goal, HUD, restart loop | Full win/loss loop playable | complete |
| Polish | Viewport fit, readable colors, no blank canvas | 375x667 and 1280x720 browser pass | complete |

## Architecture

| Area | Role |
|---|---|
| Scene setup | Renderer, camera, lights, background |
| Entity factories | Blocks, emoji sprites, goal flag |
| Level data | Static arrays for blocks, coins, hazards |
| Input | `KeyboardEvent.code` using `KeyA`, `KeyD`, `KeyW`, `KeyS`, `KeyR` |
| Game state | `playing`, `won`, `lost` plus position, timer, coin count |
| Update loop | Movement, collision, camera follow, HUD refresh |

## Level Contract

| Entity | Count | Placement rule |
|---|---:|---|
| Player start | 1 | Left edge, visible on first frame |
| Goal flag | 1 | Right edge, reachable by direct movement |
| Coins | 8 | 5 on natural route, 3 optional off-route |
| Hazards | 3 | Avoidable with `A/D/W/S`; no unavoidable contact |
| Platforms | 10-14 | At least 3 vertical layers, no dead-end trap |

## Decisions

- **Movement:** direct 2D plane movement. This matches the user request for
  `A/D` and `W/S` rather than jump-only controls.
- **Visuals:** emoji sprites drawn into canvas textures. Fast to author,
  intentionally rough, and easy to replace later.
- **Collision:** small distance/AABB checks against collectible, hazard,
  and goal trigger boxes. No physics dependency for MVP.
- **Asset policy:** no external bitmap assets. Emoji/canvas/geometry only.
