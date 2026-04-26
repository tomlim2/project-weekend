---
title: Algorithm Design v8 — Schooling Boids + Depth System
tags:
  - studio-weekend
  - design
  - algorithm
date: 2026-04-19
source: claude
status: design
---

# Algorithm Design v8 — Hallucination Aquarium

**Scope:** replaces schooling math in v7, adds a 6-level depth system, wires both into existing recoil/hover/tooltip paths.

---

## §1 — Schooling (real Boids, tightened)

Applies to `movementType === "schooler"` only. `swimmer` keeps its v7 wander untouched.

### Radii and weights (fixed values)

| Symbol   | Value   | Meaning                                     |
| -------- | ------- | ------------------------------------------- |
| `S_sep`  | 22 px   | separation radius (same or cross species)   |
| `S_align`| 55 px   | alignment radius (same species only)        |
| `S_coh`  | 75 px   | cohesion radius (same species only)         |
| `W_sep`  | 0.22    | separation weight                           |
| `W_align`| 0.10    | alignment weight                            |
| `W_coh`  | 0.020   | base cohesion weight (was 0.05)             |
| `V_max`  | 1.4 × baseSpeed | velocity clamp after force accumulation |

Tradeoff rationale: cohesion looks tight at 0.05 in v7 because it was combined with *zero effective alignment pull* — fish drifted past each other. With alignment at 0.10 the group shares heading, so **lower** cohesion (0.020) is enough to hold shape without collapsing. Separation bumped from 18 → 22 px and weighted higher than v7 so the school has visible spacing. Too tight (W_coh > 0.05 with W_align ≥ 0.10) collapses to a dot; too loose (W_coh < 0.01) looks like independent wander.

### School tightness scaling

`W_coh_effective = clamp(W_coh * (1 + 0.12 * (N − 1)), 0.020, 0.050)` where N = same-species neighbor count in S_coh. 3 fish → 0.025; 6 fish → 0.032; 10 fish → 0.044; capped at 0.050.

### Leader/follower

**Skipped.** Boids alignment produces emergent leadership — front fish's heading propagates. Explicit leader adds state churn without visible improvement at n ≤ 6.

### Per-frame loop (O(n²), n ≤ 20)

```
for each schooler S:
  sepX = sepY = alignX = alignY = cohX = cohY = 0
  nSep = nAlign = nCoh = 0
  for each other fish O:
    dx = O.x − S.x; dy = O.y − S.y; d2 = dx*dx + dy*dy
    if d2 < S_sep² :                               // any species
      inv = 1 / (sqrt(d2) + 0.001)
      sepX −= dx * inv; sepY −= dy * inv; nSep++
    if O.species == S.species:
      if d2 < S_align²: alignX += O.vx; alignY += O.vy; nAlign++
      if d2 < S_coh²:   cohX   += O.x;  cohY   += O.y;  nCoh++
  if nAlign > 0: alignX /= nAlign; alignY /= nAlign
                 S.vx += (alignX − S.vx) * W_align
                 S.vy += (alignY − S.vy) * W_align
  if nCoh > 0:   cx = cohX/nCoh; cy = cohY/nCoh
                 W = W_coh_effective(nCoh)
                 S.vx += (cx − S.x) * W / 50
                 S.vy += (cy − S.y) * W / 50
  if nSep > 0:   S.vx += sepX * W_sep
                 S.vy += sepY * W_sep
  clamp |v| ≤ V_max
  integrate S.x, S.y
  reflectWalls(S)
```

### Force ordering and boundaries

Order: **alignment → cohesion → separation → wall reflect**. Separation runs last so it dominates close contact; wall reflect is terminal.

### Sample outcomes (720 × 380 tank)

| N same-species | Visual |
|----------------|--------|
| 3  | Loose triangle, ~60 px apart, drifts as a unit, scatters at walls and reforms |
| 6  | Classic tetra cloud ~100 px wide, dominant heading, briefly fragments at corners |
| 10 | Dense oval cloud; cohesion caps at 0.050; separation keeps ~25 px spacing |

---

## §2 — Depth system (0 frontmost → 5 deepest)

### Per-fish field and initial value

`fish.depth: integer`. Init via triangular distribution centered at 2.5:
`depth = round(2.5 + (rand() + rand() − 1) * 1.5)` then `clamp(0, 5)`.
Result: ~55% at 2–3, ~30% at 1 or 4, ~15% at 0 or 5.

### Visual rendering table

| depth | scale | opacity | blur    | blue tint | z-index |
|-------|-------|---------|---------|-----------|---------|
| 0     | 1.00  | 1.00    | 0 px    | 0%        | 105     |
| 1     | 0.91  | 0.95    | 0.3 px  | 7%        | 104     |
| 2     | 0.82  | 0.90    | 0.6 px  | 14%       | 103     |
| 3     | 0.73  | 0.85    | 0.9 px  | 21%       | 102     |
| 4     | 0.64  | 0.80    | 1.2 px  | 28%       | 101     |
| 5     | 0.55  | 0.75    | 1.5 px  | 35%       | 100     |

**Formulas:** `scale(d) = 1.0 − d * 0.09`, `opacity(d) = 1.0 − d * 0.05`, `blur(d) = d * 0.3 px`, `tint(d) = d * 7%`.

### Rendering approach (safe for emoji)

```
<div class="fish"         style="transform: translate(x,y); z-index: Z">
  <div class="fish-depth" style="transform: scale(S); opacity: O; filter: blur(Bpx)">
    <div class="fish-flip" style="transform: scaleX(±1)">
      <span class="fish-sprite">🐠</span>
    </div>
    <div class="fish-tint" style="background: rgba(80,140,220,T); mix-blend-mode: color"></div>
  </div>
</div>
```

- Transforms split across layers so depth-scale and sprite-flip never fight.
- Composition order (outer → inner): `translate(position) → scale(depth) → scaleX(flip) → emoji`.
- Tint via `mix-blend-mode: color` overlay. Fallback: `@supports not (mix-blend-mode: color)` drops the overlay and applies `filter: brightness(0.85) + hue-rotate(d * 8deg)` to depth wrapper.
- Blur on `.fish-depth` wrapper, never on emoji span (Safari drops glyphs otherwise).

### Parallax

**Include.** `parallax(d) = 1.0 − d * 0.06`. Depth 0 = 1.0 → depth 5 = 0.70. Apply after boid forces and recoil, before integration.

---

## §3 — Depth transition rule

### Turn-event detection

Per fish: `committedHeading` (init = spawn heading), `headingFrame` (frame counter).

Each frame:
- `dθ = angleDiff(fish.heading, fish.committedHeading)` (wrapped to [-π, π])
- If `|dθ| > π/3` **and** `simFrame − fish.headingFrame > 30`:
  - Fire turn event
  - `fish.committedHeading = fish.heading`
  - `fish.headingFrame = simFrame`

`Θ = π/3` (60°), `T = 30 frames` (~0.5 s at 60 fps). Lazy wander doesn't trip it; wall bounces and school re-headings do.

### On turn event

Roll `rand() < P_shift = 0.35`. If true, pick `Δ ∈ {−1, +1}` uniformly; `fish.depth = clamp(fish.depth + Δ, 0, 5)`.

### Transition animation

CSS on `.fish-depth`: `transition: transform 600ms ease-out, opacity 600ms ease-out, filter 600ms ease-out`. No per-frame JS lerp.

### Per-movement-type overrides

| Type             | Rule |
|------------------|------|
| `stationary`     | Locked at spawn. No listener. |
| `wallClinger`    | Locked at spawn. |
| `bottomWalker`   | Clamped [2, 4] at spawn. `P_shift = 0.15`. |
| `bottomDweller`  | Clamped [2, 4] at spawn. `P_shift = 0.15`. |
| `verticalDrifter`| Turn event also fires on `sign(vy)` flip. `P_shift = 0.35`. |
| `swimmer` / `schooler` | Default Θ, T, P_shift. |

---

## §4 — Integration with v7

- **Hallucination recoil:** on recoil start, roll `rand() < 0.40`; if true, shift ±1 depth. Gated by `fish.recoilDepthRolled` bool, cleared when `recoilUntil` elapses.
- **Sprite flip:** set on `.fish-flip` only (`scaleX(±1)`). `.fish-depth` owns `scale(d)`. Independent layers.
- **Hover / pin:** on pin, store `fish.storedDepth = fish.depth`, set `fish.depth = 0` (CSS tween animates). Suppress turn-triggered shifts while pinned. On unpin, restore.
- **NFT card tooltip:** add `Observed depth: Layer ${fish.depth + 1} / 6` line (1-indexed).

---

## §5 — Tuning knobs

```
S_sep = 22, S_align = 55, S_coh = 75
W_sep = 0.22, W_align = 0.10, W_coh = 0.020
V_max = 1.4 * baseSpeed
DEPTH_SCALE_MIN = 0.55, DEPTH_BLUR_MAX = 1.5, DEPTH_TINT_MAX = 0.35
P_SHIFT = 0.35, P_SHIFT_BOTTOM = 0.15, P_SHIFT_RECOIL = 0.40
TURN_THETA = π/3, TURN_FRAMES = 30
DEPTH_TRANSITION_MS = 600
SCHOOL_COH_CAP = 0.050
SCHOOL_N_SOFT_CAP = 8
```

---

## §6 — Implementer checklist

1. **Fish object additions:** `depth`, `storedDepth`, `committedHeading = heading`, `headingFrame = 0`, `recoilDepthRolled = false`.
2. **DOM wrapper rewrite:** 4-layer structure (`.fish` → `.fish-depth` → `.fish-flip` → `.fish-sprite`). Move flip from outer to `.fish-flip`.
3. **`applyDepth(fish)` helper:** writes transform/opacity/blur/z-index/tint based on `fish.depth`. Called on spawn and every depth change.
4. **Turn-event controller:** compute `dθ`, check thresholds, fire depth roll. Skip for `stationary`/`wallClinger`; y-sign variant for `verticalDrifter`.
5. **Rewrite `updateSchooler`** per §1 pseudocode.
6. **Parallax multiply** after forces, before integration.
7. **Recoil hook:** 40% depth shift gated by `recoilDepthRolled`.
8. **Pin hook:** swap depth with storedDepth, call `applyDepth`, suppress turn shifts.
9. **Tooltip:** add depth line to NFT card.
10. **CSS:** `.fish-depth { transition: transform 600ms ease-out, opacity 600ms ease-out, filter 600ms ease-out; will-change: transform, filter; }` + `.fish-tint` + fallback.

### New fish fields

```
depth:             int 0..5
storedDepth:       int 0..5
committedHeading:  float rad
headingFrame:      int
recoilDepthRolled: bool
```

---

## §7 — Risks

- **Emoji + `filter: blur()`** slow in Safari. Mitigation: blur on wrapper only. n=20 × mean depth 2.5 is fine.
- **Collapse to dot** — cap at `W_coh 0.050` + `S_sep 22` keeps spacing. If still tight, lower cap to 0.040.
- **Mid-flight depth jump** — 600ms CSS tween prevents teleport feel. Never mutate depth faster than turn-event rate.
- **Z-index thrash** — write only on depth *change*, not per frame.
- **Rectangular tank** — all boundary language uses v7's rectangular `reflectWalls`.

---

## Summary (3 most impactful changes)

1. **Boids rewrite** for schoolers: `S_sep 22 / S_align 55 / S_coh 75` with weights `0.22 / 0.10 / 0.020` — tetras actually school.
2. **6-level discrete depth** with split-layer DOM giving non-destructive scale, blur, opacity, blue tint, z-index per fish.
3. **Turn-triggered depth shifts** (Θ=60°, P_shift=0.35, 600ms tween) plus recoil and pin hooks — depth tied causally to behavior, not random flicker.
