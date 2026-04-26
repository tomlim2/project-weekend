---
title: "Movement Types v6 — Hallucination Aquarium"
tags:
  - studio-weekend
  - spec
  - movement
  - animation
date: 2026-04-18
source: claude
status: draft
---

# Movement Types v6 — Hallucination Aquarium

Species-accurate behavior profiles for all 60 SPECIES entries. Replaces the single shared wander algorithm with a dispatch table keyed by `movementType`.

---

## §1 — Movement Type Taxonomy

Seven distinct behavior classes. All speed values are relative to the current v5 swimmer baseline (1.0 = `baseSpeed` range 0.7–1.4 px/frame at 60 fps).

---

### `swimmer`

**Description:** Mid-water free swimmer. Uses the existing wander + circular boundary reflection algorithm unchanged.

| Parameter | Value |
|-----------|-------|
| Speed range | 0.7–1.4× (baseline 1.0) |
| Bounding area | Full bowl — circle radius `swimR` (bowl radius − 20 px) |
| Turn behavior | Smooth wander via `smoothNoise`; heading delta ±0.06 rad/frame |
| Sprite transform | `scaleX(-1)` flip on direction change |

**In the bowl:** Fish crosses the full water column continuously, occasionally bouncing off the curved glass and reversing. The most energetic-looking type.

---

### `bottomWalker`

**Description:** Stays on the gravel floor and scuttles horizontally. Represents crabs and hermit crabs.

| Parameter | Value |
|-----------|-------|
| Speed range | 0.3–0.6× (slow deliberate scuttle) |
| Bounding area | Bottom 15% of bowl height (`y ≥ bowlH × 0.85` in screen coords; `y ≥ R × 0.70` in bowl-center coords) |
| Turn behavior | Discrete direction reversal when hitting left/right bowl edge intersection at that Y band; no arc wandering |
| Sprite transform | Flip on direction change; no vertical rotation |

**In the bowl:** Tiny specimen creeping left and right along the gravel strip, bouncing off the bowl wall at floor level. Never leaves the bottom band.

---

### `bottomDweller`

**Description:** Rests near the substrate, hovers and makes occasional short upward hops, returns to bottom. Represents plecos, corydoras, otocinclus, gobies, loaches, shrimp.

| Parameter | Value |
|-----------|-------|
| Speed range | 0.2–0.5× horizontal; hop burst 0.8–1.2× for ~0.4 s |
| Bounding area | Bottom 25% of bowl (`y ≥ R × 0.50` in bowl-center coords); hop apex may reach bottom 35% briefly |
| Turn behavior | Slow horizontal wander (heading delta ±0.03 rad/frame); random hop trigger ~every 3–7 s (add −0.8 to `vy` for 0.4 s, then gravity-like return) |
| Sprite transform | Flip on direction change |

**In the bowl:** Specimen hugs the gravel, occasionally darts a short distance upward and settles back down. Noticeably calmer than a swimmer.

---

### `wallClinger`

**Description:** Adheres to the curved bowl glass, slides along the circular boundary. Represents snails, some plecos (assigned here for visual contrast).

| Parameter | Value |
|-----------|-------|
| Speed range | 0.1–0.25× (very slow) |
| Bounding area | Circle boundary at radius `bowlR − 10` px; angle advances at ~0.002–0.004 rad/frame |
| Turn behavior | Occasional direction reversal (random every 5–15 s); otherwise constant angular drift |
| Sprite transform | `rotate(θ + 90°)` so sprite faces tangentially along the glass; no horizontal flip needed (use rotation only) |

**In the bowl:** Specimen appears stuck to the inside of the glass, slowly orbiting the bowl. Distinctive silhouette that stands apart from free swimmers.

---

### `verticalDrifter`

**Description:** Anchors at a Y band and bobs vertically with gentle horizontal drift. Represents seahorses.

| Parameter | Value |
|-----------|-------|
| Speed range | Horizontal 0.1–0.3×; vertical bob ±20–40 px amplitude, period 3–5 s |
| Bounding area | Middle vertical band (Y within ±40% of bowl center); horizontal drift stays within ±60% of bowl radius |
| Turn behavior | Sin-based vertical oscillation (`y += amplitude × sin(t / period × 2π)`); slow horizontal wander (heading delta ±0.01 rad/frame) |
| Sprite transform | No flip — seahorses are upright. Apply slight `rotate(±5°)` sway matching vertical velocity direction |

**In the bowl:** Seahorse bobs gently up and down in place, drifting lazily sideways. Unmistakably different from every other type.

---

### `schooler`

**Description:** Mid-water swimmer with weak cohesion toward nearest same-species fish. Represents tetras, rasboras, and other obligate schoolers.

| Parameter | Value |
|-----------|-------|
| Speed range | 0.6–1.2× (slightly calmer than solo swimmers) |
| Bounding area | Full bowl — same as `swimmer` |
| Turn behavior | Base wander identical to `swimmer`; each frame add cohesion force toward centroid of nearest same-species fish within 60 px (weight 0.05), separation away from any fish within 18 px (weight 0.08), alignment toward average heading of neighbors within 60 px (weight 0.10) |
| Sprite transform | `scaleX(-1)` flip on direction change |

**Schooling parameters:**
- Cohesion weight: 0.05
- Separation radius: 18 px
- Alignment weight: 0.10
- Neighbor detection radius: 60 px (same species only)

**In the bowl:** Solo schoolers wander normally; once 2+ same-species fish are present they drift together, briefly align, then fragment and regroup. Creates organic flocking micro-patterns.

---

### `stationary`

**Description:** Does not move. Fixed at spawn position. Gentle sway animation only. Represents corals and anemones.

| Parameter | Value |
|-----------|-------|
| Speed range | 0.0 (no translation) |
| Bounding area | Fixed at spawn point; spawn restricted to middle 60% of bowl |
| Turn behavior | No directional movement. Per-frame sway: `displayX = spawnX + 3 × sin(simTime / 1800 + swayOffset)` |
| Sprite transform | No flip. Sway via CSS `translateX` or inline offset only |

**In the bowl:** Coral or anemone sits anchored mid-bowl, swaying softly in an imaginary current. Acts as visual anchor point around which swimmers school and orbit.

---

## §2 — Species-to-Type Mapping

Drop-in JS object. Every species from the v4 SPECIES array is included.

```js
const SPECIES_MOVEMENT = {
  // Freshwater — livebearers & bettas
  "Guppy":                    "swimmer",
  "Endler's Livebearer":      "swimmer",
  "Molly":                    "swimmer",
  "Platy":                    "swimmer",
  "Swordtail":                "swimmer",
  "Betta":                    "swimmer",

  // Freshwater — cichlids
  "Discus":                   "swimmer",
  "Angelfish":                "swimmer",
  "Oscar":                    "swimmer",
  "Flowerhorn":               "swimmer",
  "African Cichlid":          "swimmer",
  "Jack Dempsey Cichlid":     "swimmer",
  "Peacock Cichlid":          "swimmer",
  "Parrotfish":               "swimmer",
  "Convict Cichlid":          "swimmer",

  // Freshwater — goldfish variants
  // Fancy goldfish are slow mid-water swimmers; treated as swimmer for full-bowl presence
  "Oranda Goldfish":          "swimmer",
  "Ranchu Goldfish":          "swimmer",
  "Ryukin Goldfish":          "swimmer",
  "Telescope Goldfish":       "swimmer",
  "Bubble Eye Goldfish":      "swimmer",     // judgment call: could be verticalDrifter due to poor vision/buoyancy; swimmer chosen for bowl readability
  "Pearlscale Goldfish":      "swimmer",

  // Freshwater — tetras & schoolers
  "Cardinal Tetra":           "schooler",
  "Neon Tetra":               "schooler",
  "Rummy-nose Tetra":         "schooler",
  "Black Skirt Tetra":        "schooler",

  // Freshwater — catfish & bottom dwellers
  // Pleco: wallClinger for visual contrast (plecos rasp glass constantly in real life)
  "Pleco":                    "wallClinger",  // judgment call: could be bottomDweller; wallClinger is more visually distinct and accurate to glass-rasping behavior
  "Bristlenose Pleco":        "wallClinger",  // same rationale as Pleco
  "Corydoras":                "bottomDweller",
  "Otocinclus":               "wallClinger",  // judgment call: otocinclus are almost exclusively glass/plant grazers; wallClinger chosen over bottomDweller
  "Kuhli Loach":              "bottomDweller",

  // Freshwater — gouramis & labyrinth
  "Pearl Gourami":            "swimmer",
  "Dwarf Gourami":            "swimmer",
  "Honey Gourami":            "swimmer",

  // Freshwater — exotic & show
  "Arowana":                  "swimmer",
  "Koi":                      "swimmer",
  "Axolotl":                  "bottomDweller",  // axolotls are benthic; walk on substrate and rest on bottom
  "Garra Rufa":               "bottomDweller",  // doctor fish; stays near substrate, grazes bottom
  "Rope Fish":                "bottomDweller",  // Erpetoichthys; serpentine bottom-hugger, rarely mid-water
  "Elephant Nose Fish":       "swimmer",         // mid-water active swimmer with electric organ
  "African Butterfly Fish":   "swimmer",         // surface/mid-water; treats full bowl as territory
  "Freshwater Stingray":      "bottomDweller",   // benthic obligate; lies flat on substrate

  // Freshwater — shrimp & inverts
  "Crystal Red Shrimp":       "bottomDweller",
  "Cherry Shrimp":            "bottomDweller",
  "Amano Shrimp":             "bottomDweller",
  "Mystery Snail":            "wallClinger",
  "Nerite Snail":             "wallClinger",
  "Rabbit Snail":             "wallClinger",     // judgment call: rabbit snails also crawl substrate, but wallClinger is more visually legible in a round bowl

  // Saltwater — reef community
  "Clownfish":                "swimmer",
  "Mandarin Dragonet":        "bottomDweller",
  "Seahorse":                 "verticalDrifter",
  "Firefish Goby":            "bottomDweller",   // hovers just above substrate, darts into burrow; bottomDweller with occasional hover fits
  "Watchman Goby":            "bottomDweller",
  "Clown Goby":               "bottomDweller",   // perches on coral branches at low height; bottomDweller
  "Flasher Wrasse":           "swimmer",
  "Six-line Wrasse":          "swimmer",
  "Royal Gramma":             "swimmer",

  // Saltwater — tangs & display
  "Blue Hippo Tang":          "swimmer",
  "Yellow Tang":              "swimmer",
  "Flame Angelfish":          "swimmer",
  "Copperband Butterflyfish": "swimmer"
};
```

---

## §3 — Spawn Position Rules per Type

| Type | Spawn Position | Initial Velocity |
|------|----------------|------------------|
| `swimmer` | Random point on outer circle edge (radius `bowlR − 22`) | Directed inward toward center with ±30° angular offset; speed 0.7–1.4× |
| `bottomWalker` | Random X within horizontal chord of bottom 15% band; Y at top of that band | Horizontal only: `vx = ±0.3–0.6`, `vy = 0` |
| `bottomDweller` | Random position within bottom 25% of bowl (check point is inside circle) | Near-zero: `vx` ±0.1–0.3, `vy` 0; no inward burst |
| `wallClinger` | Random angle on circle boundary at radius `bowlR − 10` | Tangential: `vx = −sin(θ) × 0.1`, `vy = cos(θ) × 0.1`; direction (CW/CCW) chosen randomly |
| `verticalDrifter` | Random X within ±40% bowl radius; Y within ±20% of bowl center (middle band) | Near-zero: `vx` ±0.05–0.15, `vy` 0; bob starts on next frame |
| `schooler` | Random point on outer circle edge; if ≥1 same-species fish already on screen, bias spawn angle toward that cluster's centroid (±45°) | Same as `swimmer` |
| `stationary` | Random point within inner 60% of bowl radius (ensures not stuck to glass) | Zero — set `vx = 0`, `vy = 0` at creation |

**Utility for `bottomDweller`/`bottomWalker` spawn:** Given bowl center at (0,0) and radius R, the valid X range for a given Y (in bowl-center coords) is `[−√(R²−Y²), +√(R²−Y²)]`. Clamp to that chord before placing.

---

## §4 — Interaction Rules (Hallucination Recoil)

The hallucination event currently shakes all fish. Per-type reactions replace the uniform recoil:

| Type | Reaction |
|------|----------|
| `stationary` | Does NOT translate. Play a brief color/glow flash: CSS `filter: brightness(2) saturate(3)` pulsed over 0.6 s, then restore. Position unchanged. |
| `wallClinger` | Detaches from wall: set `movementType` to a temporary `'wandering'` sub-state, apply `vx/vy` burst (1.5× speed, random heading) for 2 s, then re-anchor to nearest wall angle and resume `wallClinger` logic. |
| `bottomWalker` | Small hop: add `vy = −1.5` for 0.3 s (capped at bottom 30% ceiling), then gravity-return; also multiply `vx` by 1.6 for 0.8 s. |
| `bottomDweller` | Same as `bottomWalker` — hop + brief speed boost on horizontal axis. |
| `verticalDrifter` | Retreats downward: add `vy = +2.0` for 0.5 s (downward in bowl-center Y coords), then return to normal bob. |
| `swimmer` | Existing v5 recoil behavior: random heading bias, speed 1.5× for `recoilUntil` duration. No change. |
| `schooler` | Same as `swimmer`. School cohesion forces are temporarily overridden during recoil — resume on next frame after `recoilUntil` expires. |

---

## §5 — Implementer Notes

Checklist of changes required in the v5 RAF loop:

- **New field on fish object.** Add `movementType: SPECIES_MOVEMENT[creature.species] ?? "swimmer"` at fish creation time (in the block around line 1514 of generator-v5.html).

- **Split `animateFish` update block into a dispatch table.** Replace the single per-fish update logic with:
  ```js
  const updateFns = {
    swimmer:        updateSwimmer,
    bottomWalker:   updateBottomWalker,
    bottomDweller:  updateBottomDweller,
    wallClinger:    updateWallClinger,
    verticalDrifter: updateVerticalDrifter,
    schooler:       updateSchooler,
    stationary:     updateStationary,
  };
  // Inside animateFish loop:
  (updateFns[fish.movementType] ?? updateFns.swimmer)(fish, dt, R, simTime);
  ```

- **Boundary math is per-type — not all types use full-circle reflection.**
  - `swimmer` and `schooler`: keep existing circular reflection (`dist > swimR` check).
  - `bottomWalker`: only check left/right X boundary at the Y band chord; no circular reflection needed.
  - `bottomDweller`: circular reflection still applies but clamp Y to bottom 25% after integration.
  - `wallClinger`: no reflection — fish tracks the circle boundary directly via angular position.
  - `verticalDrifter`: circular reflection for horizontal drift, but clamp Y to middle band.
  - `stationary`: no boundary math — skip position integration entirely.

- **`stationary` type needs no per-frame position update.** Only compute the sway offset: `displayX = fish.spawnX + 3 * Math.sin(simTime / 1800 + fish.swayOffset)`. Store `spawnX`, `spawnY`, and `swayOffset` (random `0–2π`) at fish creation.

- **`wallClinger` tracks angle, not x/y velocity.** Store `fish.wallAngle` (radians) and `fish.wallDir` (+1 or −1). Each frame: `fish.wallAngle += fish.wallDir * 0.003`; derive `fish.x = cos(wallAngle) * (R − 10)`, `fish.y = sin(wallAngle) * (R − 10)`. Apply tangential rotation to sprite.

- **`schooler` requires a per-frame same-species neighbor scan.** Before updating any schooler, build a lookup of positions keyed by species. Keep the neighbor radius at 60 px to avoid O(n²) cost at scale; the bowl holds max 20 fish so a full pairwise scan is acceptable.

- **Hallucination recoil dispatch.** The current `fish.recoilUntil` check should branch by `movementType` for stationary (glow only), wallClinger (detach logic), and verticalDrifter (downward retreat) before falling through to the default heading-bias path.

---

## Summary

**Count per type across all 60 species:**

| Type | Count | Species |
|------|-------|---------|
| `swimmer` | 35 | All cichlids, goldfish, bettas, livebearers, gouramis, large exotics, saltwater display fish |
| `bottomDweller` | 12 | Corydoras, Kuhli Loach, Axolotl, Garra Rufa, Rope Fish, Freshwater Stingray, all 3 shrimp, Mandarin Dragonet, Firefish Goby, Watchman Goby, Clown Goby |
| `wallClinger` | 6 | Pleco, Bristlenose Pleco, Otocinclus, Mystery Snail, Nerite Snail, Rabbit Snail |
| `schooler` | 4 | Cardinal Tetra, Neon Tetra, Rummy-nose Tetra, Black Skirt Tetra |
| `stationary` | 0 | No corals/anemones in the v4 SPECIES array |
| `verticalDrifter` | 1 | Seahorse |
| `bottomWalker` | 0 | No crabs or hermit crabs in the v4 SPECIES array |

**Judgment calls made:** Pleco and Bristlenose Pleco assigned `wallClinger` rather than `bottomDweller` — real plecos rasp glass surfaces and this choice creates stronger visual contrast. Otocinclus follows the same logic. Bubble Eye Goldfish kept as `swimmer` despite buoyancy issues (bottomDweller would be accurate but confusing given its goldfish appearance). Rabbit Snail assigned `wallClinger` over bottomDweller for bowl legibility. No species in v4 map to `stationary` or `bottomWalker`; those types are ready for future species additions (corals, crabs).
