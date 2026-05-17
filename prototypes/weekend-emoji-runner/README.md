# Weekend Emoji Runner

Three.js single-file MVP for an emoji side-view runner.

## How to Run

```sh
cd prototypes/weekend-emoji-runner
python3 -m http.server 5177
```

Open `http://127.0.0.1:5177/`.

Single HTML, no build. Three.js loads through an importmap CDN.

## Controls

| Key | Action |
|---|---|
| A / D | Move left / right |
| W / S | Move up / down |
| R | Restart after win or loss |

## Status

MVP complete. See `MILESTONE.md` for the finish line and `CONCEPT.md`
for implementation scope and verification criteria.

## Scope Boundary

The MVP uses side-view traversal, block platforms, coin-like pickups, and
a goal flag. It does not use Mario names, sprites, sounds, character
likenesses, enemy likenesses, or level layouts.
