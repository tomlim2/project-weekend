---
title: "Ornamental Fish Seeds v4 — Hallucination Aquarium (US Release)"
tags:
  - studio-weekend
  - spec
  - seeds
  - english
date: 2026-04-18
source: claude
status: draft
---

# Ornamental Fish Seeds v4 — Hallucination Aquarium (US Release)

Pivot from AI/corporate-meme theme to ornamental fish NFT collection satire.
Audience: US aquarium hobbyists + NFT-culture-aware casuals.
All arrays are implementation-ready. Implementer lifts directly into JS.
Two comedy axes: (B) NFT card UI format parody + (C) breeder-variant-name absurdity.

---

## §1 — Seed Arrays

---

### SPECIES (60 entries)

```js
const SPECIES = [
  // Freshwater — livebearers & bettas
  "Guppy",
  "Endler's Livebearer",
  "Molly",
  "Platy",
  "Swordtail",
  "Betta",

  // Freshwater — cichlids
  "Discus",
  "Angelfish",
  "Oscar",
  "Flowerhorn",
  "African Cichlid",
  "Jack Dempsey Cichlid",
  "Peacock Cichlid",
  "Parrotfish",
  "Convict Cichlid",

  // Freshwater — goldfish variants
  "Oranda Goldfish",
  "Ranchu Goldfish",
  "Ryukin Goldfish",
  "Telescope Goldfish",
  "Bubble Eye Goldfish",
  "Pearlscale Goldfish",

  // Freshwater — tetras & schoolers
  "Cardinal Tetra",
  "Neon Tetra",
  "Rummy-nose Tetra",
  "Black Skirt Tetra",

  // Freshwater — catfish & bottom dwellers
  "Pleco",
  "Bristlenose Pleco",
  "Corydoras",
  "Otocinclus",
  "Kuhli Loach",

  // Freshwater — gouramis & labyrinth
  "Pearl Gourami",
  "Dwarf Gourami",
  "Honey Gourami",

  // Freshwater — exotic & show
  "Arowana",
  "Koi",
  "Axolotl",
  "Garra Rufa",
  "Rope Fish",
  "Elephant Nose Fish",
  "African Butterfly Fish",
  "Freshwater Stingray",

  // Freshwater — shrimp & inverts
  "Crystal Red Shrimp",
  "Cherry Shrimp",
  "Amano Shrimp",
  "Mystery Snail",
  "Nerite Snail",
  "Rabbit Snail",

  // Saltwater — reef community
  "Clownfish",
  "Mandarin Dragonet",
  "Seahorse",
  "Firefish Goby",
  "Watchman Goby",
  "Clown Goby",
  "Flasher Wrasse",
  "Six-line Wrasse",
  "Royal Gramma",

  // Saltwater — tangs & display
  "Blue Hippo Tang",
  "Yellow Tang",
  "Flame Angelfish",
  "Copperband Butterflyfish"
];
```

---

### VARIANT_TRAITS (80 entries)

Stackable breeder adjectives. Pick 1–4 per specimen; never repeat the same trait.

```js
const VARIANT_TRAITS = [
  // Color patterns
  "Albino",
  "Marble",
  "Galaxy",
  "Piebald",
  "Koi-pattern",
  "Mosaic",
  "Panda",
  "Snakeskin",
  "Cobra",
  "Tuxedo",
  "Neon",
  "Sunset",
  "Solar Flare",
  "Black-metallic",
  "Gold-dust",
  "Platinum",
  "Champagne",
  "Chocolate",
  "Blue Steel",
  "Copper",
  "Tri-color",
  "Bi-color",
  "Snow",
  "Obsidian",
  "Ghost",
  "Phantom",
  "Hellboy",
  "Candy",
  "Multicolor",
  "Yellow-base",

  // Body / scale texture
  "Dragon-scale",
  "Metallic",
  "Satin",
  "Dumbo",
  "Fancy",
  "Jumbo",
  "Micro",
  "Balloon",
  "Short-body",
  "Crowntail",
  "Halfmoon",
  "Double-tail",
  "Plakat",
  "Spadetail",
  "Combtail",
  "Veiltail",

  // Fin type / length
  "Longfin",
  "Shortfin",
  "Super Delta",
  "Over-halfmoon",
  "Rosetail",
  "Feathertail",
  "Butterfly",
  "Flagtail",
  "Pintail",
  "Fullmoon",

  // Special designations
  "Full-red",
  "Moscow",
  "Yellow Tail",
  "Lazuli",
  "Emerald",
  "Emperor",
  "Royal",
  "Show-grade",
  "F1 Import",
  "Wild-type",
  "Back-bred",
  "Line-bred",
  "Foundation",
  "OHM",
  "HMPK",
  "Giant"
];
```

---

### LINEAGES (25 entries)

Breeder-line suffix phrases. ~15% suffix chance per specimen.

```js
const LINEAGES = [
  "Thai Line 2019",
  "Japanese Nisai Spawn",
  "Amazon Wild-Caught Strain",
  "Singapore 2021 F3",
  "German Blue Line",
  "Florida Certified Stock",
  "Back-Bred Mutation 2023",
  "Indonesia Origin 2018",
  "Taiwan Top-Grade F2",
  "Malaysian Import Line",
  "Czech Breeding Project 2020",
  "Ukrainian Albino Strain",
  "California Domestic F4",
  "Hong Kong Show Line 2022",
  "Brazilian Wild-Type 2017",
  "Sri Lanka Blue Line",
  "Thai Red-Base F5",
  "Ohio Domestic Strain",
  "Korean Platinum Project",
  "Dutch Longfin Line 2021",
  "IBC Qualified Stock 2023",
  "Aquarama Singapore Origin",
  "F1 Domestic Cross 2024",
  "Original Importer Stock",
  "Collector Estate Line"
];
```

---

### HABITATS (50 entries)

Actual aquarium setups and natural habitats.

```js
const HABITATS = [
  // Planted / aquascape setups
  "a densely planted iwagumi layout",
  "a Dutch-style planted tank with alternating stem rows",
  "a Walstad dirted tank with no CO2 injection",
  "a high-tech planted nano with pressurized CO2",
  "a low-tech 29-gallon with floaters and dim lighting",
  "a jungle-style unscaped tank the owner calls 'natural'",

  // Biotope setups
  "a blackwater biotope with tannins at 8ppm",
  "an Amazonian biotope with fine sand and driftwood",
  "the Lake Tanganyika rock zone recreated in a 75-gallon",
  "a West African stream biotope with current pump",
  "a Lake Malawi cichlid tank with crushed coral substrate",
  "a Southeast Asian softwater setup",

  // Natural habitats
  "a Rio Negro tributary, water nearly black",
  "the upper Xingu drainage in Brazil",
  "the flooded forest margins of the Amazon basin",
  "a Mekong lowland rice paddy drainage",
  "a Japanese koi pond in November",
  "a shallow tidal flat in the Philippine Sea",
  "a coral rubble zone at 15-foot depth",
  "the zooxanthellae column of a Pacific reef face",

  // Breeder / industry environments
  "a Thai breeder's outdoor concrete vat",
  "a Singapore fish farm raceway tank",
  "the overflow trough of a wholesaler's holding tank",
  "a Jakarta facility grow-out barrel, 2022 spawn",
  "a Florida tropical fish farm pond",
  "a plastic tub in a spare bedroom in Van Nuys",
  "a basement fishroom with 28 running tanks",
  "a garage fishroom with a 300-gallon sump system",
  "the culling tank at a show-breeders facility",

  // Retail / display environments
  "a 240-gallon community display at a boutique fish store",
  "the cooler near the filter output in a neglected pet store",
  "a Petco cup on the shelf next to the conditioners",
  "a LFS display tank with too many species and one pleco",
  "the quarantine system of a reputable online vendor",

  // Reef / saltwater
  "a 180-gallon SPS-dominant reef with calcium reactor",
  "a mixed reef running Zeovit methodology",
  "a FOWLR with a 30-year-old emperor angelfish",
  "a frag tank at a coral vendor's facility",
  "a nano reef in a 10-gallon Fluval spec",
  "the sump of a 200-gallon reef system",

  // Quirky / deadpan
  "a 5-gallon Spec V in a pediatric dentist's waiting room",
  "an office desk cube measuring 2.5 gallons",
  "a hotel lobby feature wall aquarium, poorly maintained",
  "a 10-gallon in a college dorm, pH never tested",
  "the impulse-buy tank of someone who watched one YouTube video",
  "a 40-gallon breeder the owner insists is temporary"
];
```

---

### FOODS (40 entries)

Real fish foods and hobbyist-accurate specifics.

```js
const FOODS = [
  // Frozen / live
  "frozen bloodworms",
  "live brine shrimp nauplii",
  "frozen mysis shrimp",
  "frozen daphnia",
  "live blackworms",
  "mosquito larvae",
  "live tubifex worms",
  "white worms from a culture the owner has maintained for two years",
  "grindal worms",
  "live daphnia",

  // Dry / pellet
  "Hikari sinking pellets",
  "New Life Spectrum micro pellets",
  "Repashy Spawn & Grow gel food",
  "Repashy Super Green gel food",
  "spirulina wafers",
  "Omega One betta pellets",
  "Bug Bites tropical formula",
  "Xtreme Krill Flakes",
  "Vibra Bites by Hikari",
  "Ken's premium flake",

  // Specialty / controversial
  "beefheart paste (controversial)",
  "banana peppers, blended (disputed method)",
  "egg yolk paste, used once, regretted",
  "home-cultured green water",
  "biofilm scraped from aged driftwood",
  "blanched zucchini rounds",
  "blanched spinach, clipped to the glass",
  "algae wafers they ignore if bloodworms are available",
  "nori sheet (saltwater use)",
  "golden pearls WDFD 5-50 micron for fry",

  // Context-flavored
  "whatever sinks from the surface skimmer",
  "the flake the owner has used since 1997",
  "whatever the corydoras doesn't finish",
  "the automated feeder, set on 'overload' by accident",
  "three bloodworms per day (per the breeder's strict note)",
  "nothing, reportedly, and yet it thrives",
  "the algae bloom the owner has been failing to control",
  "a brand discontinued in 2018, sourced from eBay",
  "the food its tankmate refused",
  "the worms its tank neighbor ate first"
];
```

---

### BEHAVIORS (50 entries)

Real fish behaviors and absurd aquarium-forum observations.
Third-person present tense.

```js
const BEHAVIORS = [
  // Classic betta / labyrinth behavior
  "Glass-surfs only when the ceiling fan rotates.",
  "Builds a bubble nest that nobody asked for.",
  "Flares fins at its own reflection for 14 minutes, then loses interest.",
  "Responds to its owner's face but not the owner's hand.",
  "Spits food out three times, then eats it on the fourth drop.",

  // Territorial behavior
  "Guards a single rock against a species that is not present.",
  "Charges the feeding ring but will not enter it.",
  "Claims the left corner of the tank and enforces that claim.",
  "Rearranges the gravel substrate every third night.",
  "Attempts to eat the thermometer probe.",

  // Schooling / social dynamics
  "Schools tightly during LED blue hour, disperses otherwise.",
  "Refuses to school until the tank population reaches exactly 9.",
  "Leads the school toward the filter intake and then stops.",
  "Achieves shoal coherence only during feeding, then fragments.",
  "Observes schoolmates from a fixed position 2 inches above gravel.",

  // Feeding quirks
  "Refuses food on Sundays.",
  "Accepts food only from the owner's left hand.",
  "Eats exclusively at depth, ignoring surface feeders.",
  "Waits for all other fish to feed before approaching.",
  "Fasts voluntarily for four days post-water-change.",
  "Accepts frozen bloodworms; rejects live bloodworms on principle.",

  // Health & response behaviors
  "Shows no measurable stress indicators despite the ammonia reading.",
  "Velvet-clears without treatment in a planted tank. Forum posts remain disputed.",
  "Developed a stress stripe the day the heater brand was changed.",
  "Recovers from fin rot faster than the treatment timeline suggests.",
  "Eats normally throughout a 0.5ppm ammonia reading. No signs.",

  // Display & personality
  "Paces the front glass exactly at 7:30 AM.",
  "Sleeps wedged vertically between the heater and the back glass.",
  "Investigates every new object placed in the tank within 90 seconds.",
  "Recognizes the approach of the food container from 4 feet away.",
  "Follows the magnetic algae scraper the length of the tank.",
  "Has not been seen resting in two years of ownership.",

  // Pleco / catfish behaviors
  "Remains stationary for 19 hours, then relocates 3 inches.",
  "Rasps the same driftwood log every night at midnight.",
  "Hides inside the ceramic cave and is assumed dead every three weeks.",
  "Emerges from the cave exactly once and will not repeat it.",

  // Shrimp / invert behaviors
  "Molts during a 15% water change. Timing unverified.",
  "Grazes on biofilm in a circuit pattern that suggests a preference.",
  "Carries eggs for 28 days; releases them into the filter intake.",
  "Grooms the corydoras. The corydoras accepts this.",

  // Saltwater behaviors
  "Selects the most expensive coral head as its anemone substitute.",
  "Rearranges a single piece of frag rubble compulsively.",
  "Attends the glass every feeding but turns down nori.",
  "Ignores all tankmates except the goby, whom it monitors closely.",

  // Absurdist observations
  "Achieves a calming effect on the tank that no parameter explains.",
  "Has outlived three tanks, two heaters, and one marriage.",
  "Is referenced by username in at least one archived forum thread.",
  "Exhibits a behavioral quirk that contradicts four Wikipedia sentences."
];
```

---

### TRAIT_LORE (50 entries)

One-line NFT-card-flavored specimen trivia. Card-game-lore feel.

```js
const TRAIT_LORE = [
  "One of 247 documented specimens with this color expression.",
  "Pattern stabilized in 2019 by a Bangkok breeder.",
  "Sold at auction for an undisclosed sum.",
  "Featured in the 2023 Aquama Tokyo Expo catalog.",
  "Traced to a single founder pair, names withheld.",
  "Disqualified from IBC show judging for ambiguous category classification.",
  "Holds the 2022 longest-fin regional title.",
  "Lineage appears in the GFSA registry under a different collector name.",
  "Fin expression considered unstable; mutation confirmed in F2.",
  "Photographed in the 2021 TFH print edition.",
  "Acquired in a lot of 12; this specimen was the only survivor.",
  "Breeder retired after this spawn. Line considered closed.",
  "Color band does not breed true. Prized for this.",
  "Documented in two academic papers under a provisional species name.",
  "Exceeded expected show dimensions by 18%.",
  "Spawn of the last known pair from the original import.",
  "Exhibited at Aquarama Singapore 2022 under a different working name.",
  "Traded in a three-way deal involving a Florida exporter.",
  "Registry entry lists country of origin as 'unverified.'",
  "Classified as an accidental hybrid by one judge; disputed by three others.",
  "The fin mutation recurred independently in a Czech line two years later.",
  "Held the auction record for its class for 14 months.",
  "Genetic sample retained by the importer. Results pending.",
  "Not listed in the current CITES appendix. This may change.",
  "The pigmentation regressed in F3; F1 and F2 are considered definitive.",
  "Owner sold the line to a Singapore facility in 2022.",
  "Three show-quality siblings are unaccounted for.",
  "A photo of this specimen circulates on Reddit without attribution.",
  "Long-fin expression appeared without intentional selective pressure.",
  "The exporting farm closed one year after this specimen shipped.",
  "Judged 'best in class' at a regional show that has since disbanded.",
  "Import papers describe a different species. Mislabeling suspected.",
  "The color base is consistent with a mutation first documented in 1997.",
  "Two specimens from this batch are in university collection.",
  "The breeder's notes reference a 'fourth generation refinement.'",
  "Arrived at quarantine with no documentation. Papers reconstructed.",
  "Fin shape matches a 2018 Japanese line but origin is unverified.",
  "An off-show specimen; the show fish are in private collections.",
  "Considered the last verified F2 from the original import batch.",
  "Listed in a collector's estate inventory, 2021 auction.",
  "Produced from a pairing the breeder considered a long shot.",
  "The first specimen from this variant to ship commercially.",
  "Referenced in the hobbyist newsletter AQUA Notes, March 2020.",
  "Owner has declined to sell three times. Offered twice by intermediaries.",
  "Fin length documented at 2.3x body length. Registry note flags anomaly.",
  "Shows signs of a recessive trait not expressed in either parent.",
  "Photographed for a magazine spread; image was cut from final print.",
  "Purchased from a wholesaler's clearance rack for $4.",
  "The scale count is inconsistent with the recorded species. Filed.",
  "Recovered from a tank that had been unattended for six weeks."
];
```

---

### WEAKNESSES (40 entries)

Environmental sensitivities in real aquarist language.

```js
const WEAKNESSES = [
  "temperature swings beyond 2°F per hour",
  "dissolved copper above 0.1ppm",
  "LED spectrums below 6500K",
  "ammonia spikes after a missed water change",
  "pH below 6.5 in a non-blackwater setup",
  "pH above 7.8",
  "KH below 3 in soft water setups",
  "TDS above 250ppm",
  "any automated feeder — ever",
  "the sound of a heater clicking on",
  "chloramine in municipal tap water",
  "live plants it mistakes for competitors",
  "salt creep entering the sump from an evaporation event",
  "hard water above GH 15",
  "any medication containing malachite green",
  "metronidazole at above-label doses",
  "water changes larger than 30% without temperature matching",
  "the magnet scraper coming within 3 inches of its territory",
  "gravel vacuuming on a Wednesday",
  "any new fish added without a 4-week quarantine",
  "the owner's reflection at night",
  "the airline tubing vibrating against the glass",
  "dissolved oxygen below 7ppm",
  "an airstone placed near its resting position",
  "other males of the same species",
  "its own fry",
  "the filter output aimed directly at its preferred corner",
  "nitrate above 20ppm sustained over three weeks",
  "flake food after conditioning on live or frozen",
  "bright light before a 30-minute acclimation period",
  "acoustic vibrations from subwoofers near the tank stand",
  "spray bottles, aerosols, or air fresheners near the tank",
  "phosphate above 0.25ppm",
  "silicate in the water column above 1ppm",
  "tank mates with aggressive fin-nipping records",
  "fluctuating salinity in a reef setting",
  "any dewormer added without reef-safe verification",
  "the QT tank being smaller than the display tank",
  "being moved during an active bubble nest build",
  "the owner's decision to 'just add one more fish'"
];
```

---

### CATCH_ORIGINS (40 entries)

Specimen origin in specimen-registry format. Dateable.

```js
const CATCH_ORIGINS = [
  "Wild-caught, Rio Negro tributary, 2018.",
  "Captive-bred, Jakarta facility, 2022 spawn.",
  "Confiscated import, Miami CBP, 2021.",
  "Donated by a returning collector, 2020.",
  "Private breeder estate sale, 2019, undisclosed location.",
  "Captive-bred, Singapore exporter, F3 domestic line, 2023.",
  "Wild-caught, Mekong basin, Thailand, 2017.",
  "Captive-bred, Florida farm, commercial grade, 2022.",
  "Unknown origin; paperwork lists 'tropical Asia, unspecified.'",
  "Acquired at auction, Denver Koi Fest 2021.",
  "Wild-caught, Xingu River system, Brazil, 2016. Permit verified.",
  "Captive-bred, Czech Republic, IBC-registered line, 2020.",
  "Captive-bred, Los Angeles basement fishroom, 2023.",
  "Wild-caught, Kalimantan, Indonesia. CITES II. 2019.",
  "Captive-bred, Taiwan, show-grade pool, F5, 2022.",
  "Rescue specimen, LFS liquidation, 2021. Origin undocumented.",
  "Wild-caught, Lake Tanganyika, Tanzania, 2018.",
  "Imported via intermediary, Malaysia, 2020. Re-export flagged.",
  "Captive-bred, Ohio hobbyist, line in maintenance since 2014.",
  "Wild-caught, Philippines reef collection, CITES-exempt, 2019.",
  "Captive-bred, Munich hobbyist collective, 2021 season.",
  "Lot purchase, Bangkok wholesaler, 2020. Species listed generically.",
  "Wild-caught, Amazon mainstem, 2017. Two in the original shipment.",
  "Captive-bred, New Jersey commercial importer, F2, 2023.",
  "Rescued from display tank decommission, aquarium hotel, 2022.",
  "Captive-bred, Ukrainian show line, exported prior to 2022.",
  "Wild-caught, Rio Tapajós, 2015. One of four in the import.",
  "Captive-bred, private Minnesota breeder, 2020. Line retired.",
  "Acquired through hobbyist exchange, no documentation retained.",
  "Wild-caught, Okinawan coastal reef, 2019. Import licensed.",
  "Captive-bred, Vancouver Island facility, 2021 spawn.",
  "Confiscated ornamental import, LAX port, 2018. Rehomed.",
  "Wild-caught, Sri Lankan river system, 2016.",
  "Captive-bred, Dutch breeding cooperative, 2022.",
  "Pair split, collector estate, California, 2020.",
  "Wild-caught, Venezuelan llanos drainage, 2018.",
  "Captive-bred, Osaka hobbyist, Nisai class 2022.",
  "Acquired via online auction; stated origin Japan, unverified.",
  "Captive-bred, Houston fishroom, 2021. Breeder moved, line dispersed.",
  "Wild-caught, Peruvian Amazon, 2019. CITES paperwork attached."
];
```

---

### CLASSIFICATIONS (50 entries)

Pseudo-taxonomic category labels for the NFT card, aquarist-flavored.

```js
const CLASSIFICATIONS = [
  "Freshwater Nano Specimen",
  "Show-Grade Ornamental",
  "Exhibition-Tier Breed",
  "Foundation Stock",
  "Retired Tournament Line",
  "Heirloom Variant",
  "Freshwater Centerpiece Specimen",
  "Saltwater Display-Quality Specimen",
  "Aquascape Foreground Specimen",
  "Aquascape Midground Specimen",
  "Biotope-Authentic Specimen",
  "Rare Import Specimen",
  "Restricted Import Specimen",
  "Closed Lineage Specimen",
  "Open-Registry Specimen",
  "IBC-Eligible Specimen",
  "GFSA-Registered Line",
  "KHA Certified Koi Grade",
  "Nano Reef Candidate",
  "Softwater Specialist",
  "Hardwater Adapted Line",
  "Blackwater Biotope Specimen",
  "Brackish-Tolerant Variant",
  "Captive-Bred Conservation Stock",
  "Commercial Grade, Unexpected Quality",
  "Rescue-and-Rehabilitated Specimen",
  "Experimental Mutation, Unstabilized",
  "Established Mutation, Confirmed F3+",
  "Wild-Type Preservationist Stock",
  "First Domestic Generation (F1)",
  "Advanced Domestic Line (F4+)",
  "Hybrid Origin, Disputed Classification",
  "Single Spawn Recovery",
  "Show Retiree, Breeding Program Entry",
  "Retired Show Champion",
  "Pre-export Holding Specimen",
  "Auction Lot Survivor",
  "Estate Collection Transfer",
  "Long-Term Captive Specimen (5+ Years)",
  "Original Import Stock, Unmodified",
  "Selective Pressure Line, Generation 3",
  "Double Recessive Expression",
  "Single Dominant Mutation",
  "Back-Cross Stabilization Project",
  "Type Specimen, Pending Registration",
  "Flagship Specimen, Discontinued Facility",
  "Hobbyist-Grade Show Attempt",
  "Certified Aquama Expo Participant",
  "IBC Disqualified — Category Ambiguous",
  "Unregistered — Acquisition Pending"
];
```

---

### TANK_CONDITIONS (30 entries)

Actual water parameter readouts with slight deadpan absurdity.

```js
const TANK_CONDITIONS = [
  "pH 7.2, slowly declining",
  "pH 6.8, stable for now",
  "pH 8.2, buffered by coral substrate",
  "GH 8, KH 4",
  "GH 3, KH 1 — soft and acidic as intended",
  "GH 15, KH 10 — harder than optimal",
  "temperature: 78°F, stable",
  "temperature: 74°F — the heater disagreed",
  "temperature: 82°F — the heater lost",
  "TDS: 180ppm",
  "TDS: 340ppm — last water change overdue",
  "TDS: 80ppm — RO/DI, remineralized",
  "ammonia: 0ppm (as expected)",
  "ammonia: 0.25ppm (being addressed)",
  "ammonia: undetectable (the cycle is complete)",
  "nitrite: 0ppm, nitrate: 10ppm, phosphate: 0.05ppm",
  "nitrate: 40ppm — 'not ideal but the fish look fine'",
  "salinity: 1.025 SG, stable",
  "salinity: 1.026 SG, post top-off correction",
  "calcium: 420ppm, alkalinity: 8.5 dKH",
  "O2: 7.8ppm — well-aerated",
  "dissolved copper: undetectable — reef-safe confirmed",
  "last water change: 4 days overdue",
  "last water change: 3 weeks overdue — owner 'monitors the parameters'",
  "last water change: completed today, 20%",
  "lights on 8hr photoperiod, blue channel 2hr fade",
  "lights on 12hr — the timer broke in January",
  "CO2: 25–30ppm — pressurized, drop checker green",
  "no CO2 — heavily planted and getting away with it",
  "filter: canister, last cleaned August. Smelled fine."
];
```

---

## §2 — Display Name Construction

Each fish's display name is a stacked breeder-style designation.

**Target format examples:**
- `Platinum Galaxy Halfmoon Longfin Betta, Thai Line 2019`
- `Marble Snakeskin Koi-pattern Guppy`
- `Albino Dragon-scale Oscar`
- `Solar Flare Veiltail Betta, Singapore 2021 F3`
- `Ghost Mosaic Crystal Red Shrimp`

### Stacking Probabilities

| Trait count | Probability | Description |
|-------------|-------------|-------------|
| 1 trait | 60% | Most specimens |
| 2 traits | 30% | Uncommon |
| 3 traits | 8% | Rare |
| 4 traits | 2% | Ultra-rare / UNCLASSIFIABLE-eligible |

- 15% chance of lineage suffix `, {LINEAGE}`.
- Never stack the same trait twice in a single name.
- Traits appear in order: color/pattern first, then body type, then fin type.

### `generateDisplayName()` Pseudocode

```js
function generateDisplayName(species, variantTraits, lineages) {
  // 1. Pick species (always)
  const speciesName = pick(species);

  // 2. Determine trait count
  const r = Math.random();
  let traitCount;
  if      (r < 0.60) traitCount = 1;
  else if (r < 0.90) traitCount = 2;
  else if (r < 0.98) traitCount = 3;
  else               traitCount = 4;

  // 3. Pick unique traits
  const shuffled = [...variantTraits].sort(() => Math.random() - 0.5);
  const chosen = shuffled.slice(0, traitCount);

  // 4. Maybe add lineage
  const lineageSuffix = Math.random() < 0.15
    ? `, ${pick(lineages)}`
    : '';

  // 5. Assemble
  return `${chosen.join(' ')} ${speciesName}${lineageSuffix}`;
}
```

### 5 Example Outputs

1. `Platinum Galaxy Halfmoon Betta, Thai Line 2019`
2. `Marble Snakeskin Guppy`
3. `Albino Dragon-scale Oscar`
4. `Panda Corydoras`
5. `Solar Flare Veiltail Mosaic Betta, Singapore 2021 F3`

---

## §3 — Scientific Name Generator (Real-Fish Flavor)

### `GENUS_BY_SPECIES` — Map (60 entries)

```js
const GENUS_BY_SPECIES = {
  "Guppy":                  "Poecilia",
  "Endler's Livebearer":    "Poecilia",
  "Molly":                  "Poecilia",
  "Platy":                  "Xiphophorus",
  "Swordtail":              "Xiphophorus",
  "Betta":                  "Betta",
  "Discus":                 "Symphysodon",
  "Angelfish":              "Pterophyllum",
  "Oscar":                  "Astronotus",
  "Flowerhorn":             "Hybrid",
  "African Cichlid":        "Metriaclima",
  "Jack Dempsey Cichlid":   "Rocio",
  "Peacock Cichlid":        "Aulonocara",
  "Parrotfish":             "Hybrid",
  "Convict Cichlid":        "Amatitlania",
  "Oranda Goldfish":        "Carassius",
  "Ranchu Goldfish":        "Carassius",
  "Ryukin Goldfish":        "Carassius",
  "Telescope Goldfish":     "Carassius",
  "Bubble Eye Goldfish":    "Carassius",
  "Pearlscale Goldfish":    "Carassius",
  "Cardinal Tetra":         "Paracheirodon",
  "Neon Tetra":             "Paracheirodon",
  "Rummy-nose Tetra":       "Hemigrammus",
  "Black Skirt Tetra":      "Gymnocorymbus",
  "Pleco":                  "Hypostomus",
  "Bristlenose Pleco":      "Ancistrus",
  "Corydoras":              "Corydoras",
  "Otocinclus":             "Otocinclus",
  "Kuhli Loach":            "Pangio",
  "Pearl Gourami":          "Trichopodus",
  "Dwarf Gourami":          "Trichogaster",
  "Honey Gourami":          "Trichogaster",
  "Arowana":                "Osteoglossum",
  "Koi":                    "Cyprinus",
  "Axolotl":                "Ambystoma",
  "Garra Rufa":             "Garra",
  "Rope Fish":              "Erpetoichthys",
  "Elephant Nose Fish":     "Gnathonemus",
  "African Butterfly Fish": "Pantodon",
  "Freshwater Stingray":    "Potamotrygon",
  "Crystal Red Shrimp":     "Caridina",
  "Cherry Shrimp":          "Neocaridina",
  "Amano Shrimp":           "Caridina",
  "Mystery Snail":          "Pomacea",
  "Nerite Snail":           "Vittina",
  "Rabbit Snail":           "Tylomelania",
  "Clownfish":              "Amphiprion",
  "Mandarin Dragonet":      "Synchiropus",
  "Seahorse":               "Hippocampus",
  "Firefish Goby":          "Nemateleotris",
  "Watchman Goby":          "Cryptocentrus",
  "Clown Goby":             "Gobiodon",
  "Flasher Wrasse":         "Paracheilinus",
  "Six-line Wrasse":        "Pseudocheilinus",
  "Royal Gramma":           "Gramma",
  "Blue Hippo Tang":        "Paracanthurus",
  "Yellow Tang":            "Zebrasoma",
  "Flame Angelfish":        "Centropyge",
  "Copperband Butterflyfish": "Chelmon"
};
```

### `VARIANT_EPITHETS` — Map (40 mappings)

```js
const VARIANT_EPITHETS = {
  "Albino":        "albinica",
  "Marble":        "marmoratus",
  "Galaxy":        "galaxiensis",
  "Piebald":       "piebaldicus",
  "Koi-pattern":   "koideus",
  "Mosaic":        "mosaicus",
  "Panda":         "pandanus",
  "Snakeskin":     "serpentinus",
  "Cobra":         "cobriensis",
  "Tuxedo":        "tuxedensis",
  "Neon":          "neonicus",
  "Sunset":        "crepuscularis",
  "Solar Flare":   "solarflaris",
  "Black-metallic":"atro-metallicus",
  "Gold-dust":     "auripulveris",
  "Platinum":      "platineus",
  "Champagne":     "bullanticus",
  "Chocolate":     "chocolatus",
  "Blue Steel":    "caerulaceus",
  "Copper":        "cupreus",
  "Ghost":         "phantomicus",
  "Phantom":       "spectralis",
  "Halfmoon":      "lunaris",
  "Double-tail":   "bifurcatus",
  "Longfin":       "longipinnis",
  "Veiltail":      "veliferus",
  "Crowntail":     "coronalis",
  "Dragon-scale":  "draconisquamatus",
  "Dumbo":         "auricularis",
  "Balloon":       "vesicularis",
  "Emperor":       "imperatus",
  "Giant":         "giganticus",
  "Rosetail":      "roseapinnatus",
  "Show-grade":    "exhibitionis",
  "OHM":           "ohmicus",
  "Metallic":      "metallicus",
  "Wild-type":     "sylvestris",
  "Foundation":    "fundatoris",
  "Back-bred":     "retrogeneticus",
  "Tri-color":     "tricoloris"
};
```

### Binomial Format

`{Genus} {epithet}` — two-word only.

**Fallback rule:** If the primary variant trait has no entry in `VARIANT_EPITHETS`, latinize phonetically:
- Ends in consonant → add `-icus`
- Ends in vowel → add `-ensis`
- Multi-word traits → use first word only

### `generateScientificName(species, primaryVariant)` Pseudocode

```js
function generateScientificName(species, primaryVariant) {
  const genus = GENUS_BY_SPECIES[species] ?? latinizeGenus(species);
  let epithet = VARIANT_EPITHETS[primaryVariant];

  if (!epithet) {
    // Phonetic fallback
    const base = primaryVariant.split(' ')[0].toLowerCase();
    epithet = base.match(/[aeiou]$/) ? base + 'ensis' : base + 'icus';
  }

  return `${genus} ${epithet}`;
}
```

### 5 Sample Outputs

| Species | Primary Variant | Scientific Name |
|---------|-----------------|-----------------|
| Betta | Halfmoon | *Betta lunaris* |
| Guppy | Galaxy | *Poecilia galaxiensis* |
| Discus | Albino | *Symphysodon albinica* |
| Crystal Red Shrimp | Mosaic | *Caridina mosaicus* |
| Arowana | Platinum | *Osteoglossum platineus* |

---

## §4 — NFT Card UI Spec

Every spawned fish displays this metadata block.

### Rarity Tiers

| Tier | Label | Mint Size | Rarity % | Spawn Weight |
|------|-------|-----------|----------|--------------|
| CLASS I | Common Tank Stock | / 10000 | 43% | 43 |
| CLASS II | Show-Quality Specimen | / 3000 | 28% | 28 |
| CLASS III | Regional Champion Line | / 500 | 18% | 18 |
| CLASS IV | Heritage Breeding Line | / 100 | 8% | 8 |
| UNCLASSIFIABLE | Unregistered Specimen | / 12 or 1 of 1 | 3% | 3 |

For flavor: perturb displayed rarity ±0.1% per specimen (`displayed = tierRarity + (Math.random() * 0.2 - 0.1)`).

UNCLASSIFIABLE specimens: 50% chance of `/ 1 of 1`, 50% chance of `/ 12`.

### Edition Number

Tracked client-side using a `Map<string, number>` keyed by **fish signature**.

**Signature rule:**
```js
// signature = "{species}|{sortedTraits.join('|')}"
// e.g. "Betta|Galaxy|Halfmoon|Longfin"
function getFishSignature(species, traits) {
  return [species, ...[...traits].sort()].join('|');
}

// Edition counter
const editionCounts = new Map();
function getEditionNumber(signature) {
  const n = (editionCounts.get(signature) ?? 0) + 1;
  editionCounts.set(signature, n);
  return n;
}
```

Edition display: `Edition #${String(n).padStart(4, '0')} / ${mintSizeForTier}`.

### Hash ID

Deterministic from fish data. Format: `SPECIMEN-#A7F23-B9`.

```js
function generateHashId(species, traits, catchOrigin) {
  // Simple deterministic hash — implementer may swap for a proper hash fn
  const raw = [species, ...traits, catchOrigin].join('_');
  let h = 0;
  for (let i = 0; i < raw.length; i++) h = (h * 31 + raw.charCodeAt(i)) >>> 0;
  const hex = h.toString(16).toUpperCase().padStart(8, '0');
  return `SPECIMEN-#${hex.slice(0,5)}-${hex.slice(5,7)}`;
}
```

### Trait Chips

Rendered as inline chips in NFT-marketplace style.

```html
<!-- Trait chip row -->
<div class="trait-chips">
  <!-- Repeat per trait -->
  <span class="trait-chip">Galaxy</span>
  <span class="trait-chip">Halfmoon</span>
  <span class="trait-chip">Longfin</span>
</div>
```

CSS suggestion:
```css
.trait-chip {
  display: inline-block;
  padding: 2px 10px;
  border-radius: 12px;
  background: #f0ece4;
  border: 1px solid #d0c8bc;
  font-size: 11px;
  font-weight: 600;
  color: #4a4038;
  letter-spacing: 0.02em;
}
```

### Full Card HTML Structure

```html
<div class="fish-card" data-tier="CLASS_III">
  <!-- Header row -->
  <div class="card-header">
    <span class="card-edition">Edition #0147 / 500</span>
    <span class="card-hash">SPECIMEN-#A7F23-B9</span>
  </div>

  <!-- Fish name + scientific name -->
  <div class="card-name">Platinum Galaxy Halfmoon Longfin Betta, Thai Line 2019</div>
  <div class="card-sci"><em>Betta lunaris galaxiensis</em></div>

  <!-- Rarity badge -->
  <div class="card-rarity-row">
    <span class="rarity-badge">CLASS III — Regional Champion Line</span>
    <span class="rarity-pct">Rarity: 18.0%</span>
  </div>

  <!-- Trait chips -->
  <div class="trait-chips">
    <span class="trait-chip">Platinum</span>
    <span class="trait-chip">Galaxy</span>
    <span class="trait-chip">Halfmoon</span>
    <span class="trait-chip">Longfin</span>
  </div>

  <!-- Description (tone-generated) -->
  <div class="card-desc">…</div>

  <!-- Provenance -->
  <div class="card-provenance">
    <span class="provenance-label">Origin:</span>
    Captive-bred, Singapore exporter, F3 domestic line, 2023.
    <span class="provenance-arrow">→</span>
    Current display tank.
  </div>
</div>
```

### JS Assembly Notes

```js
function buildCardData(species, traits, tier, catchOrigin) {
  const signature  = getFishSignature(species, traits);
  const editionNum = getEditionNumber(signature);
  const mintSize   = MINT_SIZE[tier];        // map tier → number
  const hashId     = generateHashId(species, traits, catchOrigin);
  const rarityBase = RARITY_BASE[tier];      // e.g. 18
  const rarityDisp = (rarityBase + (Math.random() * 0.2 - 0.1)).toFixed(1);
  return { signature, editionNum, mintSize, hashId, rarityDisp };
}
```

---

## §5 — Tones

Two tones, replacing v3's four.

---

### Tone 1: Breeder Spec Sheet

**Voice:** Dry, technical, hobby-forum. Reads like a livestock listing or a studbook entry. No hype. Just data.

**Template:**

```
{displayName} ({scientificName})
Tier: {classLabel} | Edition {editionStr} | Rarity: {rarityPct}%

Observed in {habitat}.
Diet: {food}.

Behavioral note: {behavior}

Collector lore: {traitLore}
Known sensitivity: {weakness}

Origin: {catchOrigin}
Tank conditions at acquisition: {tankConditions}
```

**Sample 1 (CLASS III):**

> Platinum Galaxy Halfmoon Longfin Betta, Thai Line 2019 (*Betta lunaris galaxiensis*)
> Tier: Regional Champion Line | Edition #0147 / 500 | Rarity: 18.1%
>
> Observed in a densely planted iwagumi layout.
> Diet: live brine shrimp nauplii.
>
> Behavioral note: Builds a bubble nest that nobody asked for.
>
> Collector lore: Pattern stabilized in 2019 by a Bangkok breeder.
> Known sensitivity: temperature swings beyond 2°F per hour.
>
> Origin: Captive-bred, Singapore exporter, F3 domestic line, 2023.
> Tank conditions at acquisition: pH 7.2, slowly declining.

**Sample 2 (CLASS I):**

> Marble Snakeskin Guppy (*Poecilia marmoratus*)
> Tier: Common Tank Stock | Edition #0031 / 10000 | Rarity: 42.9%
>
> Observed in a plastic tub in a spare bedroom in Van Nuys.
> Diet: Hikari sinking pellets.
>
> Behavioral note: Schools tightly during LED blue hour, disperses otherwise.
>
> Collector lore: Purchased from a wholesaler's clearance rack for $4.
> Known sensitivity: chloramine in municipal tap water.
>
> Origin: Captive-bred, Florida farm, commercial grade, 2022.
> Tank conditions at acquisition: TDS: 180ppm.

---

### Tone 2: Auction Listing

**Voice:** Slightly hyped, like an eBay or OpenSea listing. "Rare opportunity," "impeccable lineage," "verified provenance." Not breathless — calibrated enthusiasm.

**Template:**

```
🔖 {displayName}
{scientificName} · {classLabel} · Edition {editionStr}

Rarity: {rarityPct}% — a genuinely uncommon encounter.

This specimen was collected from {habitat} and presents in exceptional condition. Primary diet: {food}. Behavioral profile: {behavior}

Trait highlights: {traitChips (joined with " · ")}

Lore: {traitLore}
Provenance: {catchOrigin}

Known sensitivity: {weakness}

Starting conditions logged: {tankConditions}

Serious inquiries welcome. This listing reflects a verified specimen.
```

**Sample 1 (CLASS III):**

> **Platinum Galaxy Halfmoon Longfin Betta, Thai Line 2019**
> *Betta lunaris galaxiensis* · Regional Champion Line · Edition #0147 / 500
>
> Rarity: 18.1% — a genuinely uncommon encounter.
>
> This specimen was collected from a densely planted iwagumi layout and presents in exceptional condition. Primary diet: live brine shrimp nauplii. Behavioral profile: Builds a bubble nest that nobody asked for.
>
> Trait highlights: Platinum · Galaxy · Halfmoon · Longfin
>
> Lore: Pattern stabilized in 2019 by a Bangkok breeder.
> Provenance: Captive-bred, Singapore exporter, F3 domestic line, 2023.
>
> Known sensitivity: temperature swings beyond 2°F per hour.
>
> Starting conditions logged: pH 7.2, slowly declining.
>
> Serious inquiries welcome. This listing reflects a verified specimen.

**Sample 2 (UNCLASSIFIABLE):**

> **Ghost Mosaic Dragon-scale Freshwater Stingray, Collector Estate Line**
> *Potamotrygon phantomicus* · ★ Unregistered Specimen ★ · Edition #0001 / 1 of 1
>
> Rarity: 3.0% — one of a kind.
>
> This specimen was collected from an unknown biotope and presents in exceptional condition. Primary diet: live blackworms. Behavioral profile: Has outlived three tanks, two heaters, and one marriage.
>
> Trait highlights: Ghost · Mosaic · Dragon-scale
>
> Lore: Acquisition refused by two auction houses. Classification pending with CITES.
> Provenance: Acquired through hobbyist exchange, no documentation retained.
>
> Known sensitivity: any medication containing malachite green.
>
> Starting conditions logged: TDS: 340ppm — last water change overdue.
>
> Serious inquiries welcome. Verified as unique in the current registry.

---

### Tone Selection Weights

| Tier | Breeder Spec Sheet | Auction Listing |
|------|--------------------|-----------------|
| CLASS I | 70% | 30% |
| CLASS II | 70% | 30% |
| CLASS III | 50% | 50% |
| CLASS IV | 50% | 50% |
| UNCLASSIFIABLE | 30% | 70% |

```js
function pickTone(tier) {
  const auctionChance = {
    CLASS_I: 0.30, CLASS_II: 0.30,
    CLASS_III: 0.50, CLASS_IV: 0.50,
    UNCLASSIFIABLE: 0.70
  };
  return Math.random() < (auctionChance[tier] ?? 0.40)
    ? 'AUCTION'
    : 'BREEDER_SPEC';
}
```

---

## §6 — UI Copy Map

Replacement strings for every user-facing element.

| Element | v3 English | v4 English |
|---------|-----------|-----------|
| Page `<title>` | Hallucination Aquarium — Unclassified Specimen Observer v3 | Hallucination Aquarium — Ornamental Specimen Catalog |
| H1 | 🐟 Hallucination Aquarium | 🐟 Hallucination Aquarium |
| Hint text | Press Release to observe an unclassified specimen. Tank resets on refresh. | Press Release Specimen to admit an ornamental specimen to the collection. Catalog resets on refresh. |
| Aquarium empty hint | An unclassified specimen will surface shortly… | A curated specimen will drift in shortly… |
| Release button | 🔭 Release | 🔭 Release Specimen |
| Pinned card close | ✕ | ✕ |
| Dex panel H2 | 📋 Observation Log | 📋 Specimen Catalog |
| Dex section: habitats | Habitat Log — N / 50 sites | Habitat Registry — N / 50 sites logged |
| Dex section: foods | Prey Sighted — N types | Diet on Record — N types documented |
| Dex section: trivia | Field Notes Collected — N / 50 entries | Lore Entries Cataloged — N / 50 |
| Dex list header | Specimen Log | Catalog Entries |
| Tier overlay subtitle | This may not be a fish. | This specimen has not been classified by any recognized registry. |
| Dev tools × 10 release | × 10 Release | × 10 Admit |
| Dev tools reset | 🚿 Reset Tank | 🚿 Drain Catalog |

### Milestone Toasts (12)

All in breeder/curator voice. Replace v3's toasts wholesale.

```js
const MILESTONE_TOASTS = [
  // Collection milestones
  { at: 1,   text: "First specimen admitted. Catalog is open." },
  { at: 5,   text: "5 specimens on record. The tank is showing personality." },
  { at: 10,  text: "10 cataloged. A modest but earnest collection." },
  { at: 20,  text: "20 specimens. You are operating a fishroom, not a hobby." },
  { at: 30,  text: "30 on record. At this volume, a sump is recommended." },
  { at: 50,  text: "50 cataloged. The registry is beginning to take shape." },
  // Rarity milestones
  { tier: "CLASS_III", first: true, text: "First Regional Champion Line specimen. Worth noting." },
  { tier: "CLASS_IV",  first: true, text: "First Heritage Breeding Line. The lineage has been verified." },
  { tier: "UNCLASSIFIABLE", first: true, text: "★ Unregistered Specimen admitted. Two auction houses have declined to classify this. ★" },
  // Behavioral / catalog milestones
  { habitats: 10,  text: "10 habitats on record. The biotope range is broadening." },
  { habitats: 25,  text: "25 habitat sites logged. Cross-reference with your water report." },
  { lore: 25,      text: "25 lore entries cataloged. The registry is getting interesting." }
];
```

### Hallucination Overlay (UNCLASSIFIABLE tier — 5 entries)

Fish-themed cryptid flavor. Two lines each.

```js
const HALLUCINATION_SUBTITLES = [
  "This entity exists outside known fish taxonomy.\nAcquisition is logged; classification is not.",
  "Suspected hybrid: parentage unverifiable.\nThe IBC has been notified. No response expected.",
  "Unclassifiable — acquisition refused by two auction houses.\nOne judge described it as 'plausible but inadmissible.'",
  "Registry cross-reference returns no matches.\nThis specimen predates the current classification system.",
  "Pattern does not breed true and has never repeated.\nConsidered by some to be a one-generation phenomenon."
];
```

---

## §7 — Tier Labels

**Format: `CLASS I / II / III / IV / ★ UNCLASSIFIABLE ★`**

| Tier | Short Label | Descriptor Phrase |
|------|-------------|-------------------|
| CLASS I | Common Tank Stock | Commonly available. Breeds readily. Found in most LFS. |
| CLASS II | Show-Quality Specimen | Meets minimum show criteria. Above retail grade. |
| CLASS III | Regional Champion Line | Documented show record or recognized breeder line. |
| CLASS IV | Heritage Breeding Line | Closed lineage, traceable provenance. Rarely offered. |
| ★ UNCLASSIFIABLE ★ | Unregistered Specimen | Outside current registry definitions. Classification pending. |

```js
const TIER_LABELS = {
  CLASS_I:         { short: "Common Tank Stock",       phrase: "Commonly available. Breeds readily. Found in most LFS." },
  CLASS_II:        { short: "Show-Quality Specimen",   phrase: "Meets minimum show criteria. Above retail grade." },
  CLASS_III:       { short: "Regional Champion Line",  phrase: "Documented show record or recognized breeder line." },
  CLASS_IV:        { short: "Heritage Breeding Line",  phrase: "Closed lineage, traceable provenance. Rarely offered." },
  UNCLASSIFIABLE:  { short: "Unregistered Specimen",   phrase: "Outside current registry definitions. Classification pending." }
};
```

---

## §8 — Core-Loop Sanity Check Question

**The one question to resolve before implementation begins:**

> **Does the NFT card appear as a permanent panel on the card, or as an expandable layer the player taps/clicks into?**

This is the single layout decision with the most downstream consequences. If the card face always shows edition number, rarity, hash ID, and trait chips, you need those elements to be compact enough to fit alongside the fish name and description without feeling like a data dump — which will require either tighter visual hierarchy or truncation logic. If the NFT metadata lives behind a toggle ("View Certificate" or "Show Provenance"), the main card stays readable at a glance and the collector layer rewards curious users without cluttering the ambient experience. The breeder-spec tone reads well as a primary face; the auction listing tone reads better when revealed. Consider whether the two tones should drive two different layout modes, or whether one consistent layout handles both.

---

## Summary

**Category counts:**
- SPECIES: 60 | VARIANT_TRAITS: 80 | LINEAGES: 25 | HABITATS: 50 | FOODS: 40
- BEHAVIORS: 50 | TRAIT_LORE: 50 | WEAKNESSES: 40 | CATCH_ORIGINS: 40
- CLASSIFICATIONS: 50 | TANK_CONDITIONS: 30

**Total seeds: 515 entries across 11 arrays.**

**3 strongest collector-hook seeds:**
1. TRAIT_LORE: `"Recovered from a tank that had been unattended for six weeks."` — earns a laugh from anyone in the hobby and lands a note of pathos.
2. BEHAVIORS: `"Has outlived three tanks, two heaters, and one marriage."` — immediately quotable; the line is doing dual work as character lore and light tragedy.
3. CATCH_ORIGINS: `"Confiscated import, Miami CBP, 2021."` — deadpan and specific; feels like it belongs on an actual specimen card at a natural history museum.

**1 flagged item:**
The `GENUS_BY_SPECIES` entries for `"Flowerhorn"` and `"Parrotfish"` are mapped to `"Hybrid"` — which is accurate (both are man-made hybrids with no valid genus) but will look odd in the scientific name output. Recommended: map them to plausible-sounding placeholder genera (`"Cichlasomoides"` for Flowerhorn, `"Hoplarchus"` for the blood parrot) and add a card footnote `*hybrid origin — genus designation informal*`. Keeps the format consistent without lying.

**SYMPTOMS vs. WEAKNESSES:**
Drop SYMPTOMS entirely — it reads as medical/AI and has no natural home in aquarium language. Keep WEAKNESSES (repurposed from v3): environmental sensitivities are exactly how hobbyists talk, the array produced strong entries, and it pairs naturally with the card's "Known sensitivity:" field. The replacement is clean.
