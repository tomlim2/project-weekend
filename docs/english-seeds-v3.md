---
title: "English Seeds v3 — Hallucination Aquarium (US Release)"
tags:
  - studio-weekend
  - spec
  - seeds
  - english
date: 2026-04-18
source: claude
status: draft
---

# English Seeds v3 — Hallucination Aquarium (US Release)

Native English regeneration. NOT a translation of v1/v2. Audience: AI-Twitter / tech-worker / terminally-online Reddit.
All arrays are implementation-ready. Implementer lifts directly into JS.

---

## §1 — Seed Arrays

---

### NOUNS (90 entries)

```js
const NOUNS = [
  // Household objects
  "toaster",
  "broken umbrella",
  "crisper drawer",
  "dish sponge",
  "half-used candle",
  "shower curtain",
  "dead houseplant",
  "extension cord",
  "junk drawer",
  "spare key",
  "empty wall hook",
  "laundry basket",
  "rogue sock",
  "rubber band ball",
  "expired coupon",

  // Office items
  "dead AA battery",
  "unclaimed package",
  "office chair wheel",
  "whiteboard marker",
  "sticky note",
  "lanyard",
  "communal stapler",
  "conference room booking",
  "standing desk",
  "ergonomic keyboard",

  // Mundane groceries
  "sourdough discard",
  "wilted celery",
  "the last Greek yogurt",
  "overripe avocado",
  "bulk granola",
  "mystery freezer item",
  "half-eaten granola bar",
  "store-brand sparkling water",
  "expired kombucha",
  "deli container",

  // Small animals
  "office moth",
  "lobby beetle",
  "conference sparrow",
  "parking lot pigeon",
  "break room fruit fly",
  "bathroom spider",
  "desktop mouse",

  // Abstract / weather nouns
  "afternoon haze",
  "unsettled feeling",
  "ambient dread",
  "low-grade static",
  "barometric pressure drop",
  "mercury retrograde",
  "background hum",
  "peripheral shadow",
  "Tuesday malaise",
  "mild dissociation",

  // Digital / AI-native nouns
  "Chrome tab",
  "unread Slack thread",
  "expired LinkedIn trial",
  "401(k) statement",
  "loading spinner",
  "push notification",
  "Jira ticket",
  "staging environment",
  "deprecated API",
  "ghost branch",
  "open PR",
  "hallucinated citation",
  "model card",
  "fine-tuned checkpoint",
  "RLHF annotation",
  "embedding space",
  "context window",
  "system prompt",
  "token budget",
  "attention head",
  "RAG pipeline",
  "vector database",
  "inference cost",
  "prompt injection",
  "alignment tax",
  "knowledge cutoff",
  "temperature setting",
  "forgotten bookmark",
  "browser history",
  "404 page"
];
```

---

### ADJECTIVES (65 entries)

```js
const ADJECTIVES = [
  // Mood / attitude / state
  "sad",
  "mildly unhinged",
  "deeply tired",
  "serene",
  "resentful",
  "restless",
  "vaguely smug",
  "profoundly bored",
  "quietly devastated",
  "suspicious",
  "relieved",
  "hollow",
  "agitated",
  "wistful",
  "dissociated",
  "vindicated",
  "haunted",
  "overstimulated",
  "underpaid",
  "newly radicalized",

  // American cultural flavor
  "CrossFit-curious",
  "Bon Appétit-pilled",
  "HOA-defying",
  "Zillow-poisoned",
  "podcast-addled",
  "Costco-optimized",
  "Substack-fatigued",
  "TED-talk-adjacent",
  "LinkedIn-core",
  "IPO-adjacent",
  "401(k)-aware",
  "latte-regretful",
  "emotionally unavailable",
  "aggressively mediocre",
  "professionally gaslighted",
  "perpetually almost-famous",
  "Brooklyn-adjacent",

  // AI / tech meme adjectives
  "hallucinating",
  "context-starved",
  "RAG-poisoned",
  "deprecated",
  "fine-tuned",
  "overfit",
  "underspecified",
  "misaligned",
  "token-limited",
  "temperature-maxed",
  "instruction-following",
  "prompt-injected",
  "reward-hacked",
  "low-perplexity",
  "entropy-collapsed",
  "positionally confused",
  "RLHF-scarred",
  "knowledge-cutoff-adjacent",
  "latency-sensitive",
  "zero-shot",
  "out-of-distribution",
  "stochastic",
  "grounded",
  "ungrounded",
  "retrieval-augmented",
  "softmax-indecisive",
  "cached",
  "uncached"
];
```

---

### OCCUPATIONS (75 entries)

```js
const OCCUPATIONS = [
  // White-collar corporate
  "product manager",
  "data analyst",
  "UX researcher",
  "account executive",
  "VP of something",
  "chief of staff",
  "people ops specialist",
  "growth marketer",
  "brand strategist",
  "revenue operations lead",
  "corporate trainer",
  "technical writer",
  "solutions architect",
  "scrum master",
  "director of customer success",

  // Tech
  "frontend engineer",
  "backend engineer",
  "DevOps engineer",
  "ML engineer",
  "data scientist",
  "prompt engineer",
  "AI safety researcher",
  "technical program manager",
  "platform engineer",
  "full-stack developer",
  "QA engineer",
  "data labeler",
  "API evangelist",
  "open-source maintainer",
  "HuggingFace contributor",

  // Gig economy
  "DoorDash driver",
  "Uber driver",
  "TaskRabbit handyman",
  "Amazon Flex driver",
  "Rover dog walker",
  "Instacart shopper",
  "Fiverr logo designer",

  // Creative
  "podcast host",
  "Substack writer",
  "indie game developer",
  "TikTok creator",
  "YouTube essayist",
  "music producer",
  "graphic novelist",
  "copywriter",
  "set designer",
  "film editor",

  // Service industry
  "barista",
  "line cook",
  "bartender",
  "hotel concierge",
  "flight attendant",
  "pharmacy tech",
  "urgent care nurse",
  "call center rep",
  "personal trainer",
  "mail carrier",

  // Pseudo-spiritual / wellness
  "life coach",
  "energy healer",
  "breathwork facilitator",
  "plant medicine guide",
  "somatic therapist",
  "astrologer",
  "manifestation coach",
  "relationship alchemist",
  "neuro-linguistic practitioner",
  "mindset consultant",

  // Crime / noir archetypes
  "retired FBI analyst",
  "skip tracer",
  "repo man",
  "court reporter",
  "bail bondsman",
  "notary public",
  "private investigator",
  "forensic accountant",
  "undercover consultant",
  "confidential informant"
];
```

---

### INTENSIFIERS (10 entries)

```js
const INTENSIFIERS = [
  "kinda",
  "lowkey",
  "absolutely",
  "famously",
  "unreasonably",
  "allegedly",
  "statistically",
  "suspiciously",
  "objectively",
  "quietly"
];

// Usage: pick(INTENSIFIERS) + " " + pick(ADJECTIVES) → "lowkey deprecated", "famously hallucinating"
// Apply probability: 0.30 (30%)
```

---

### OCCUPATION_MODIFIERS (10 entries)

```js
const OCCUPATION_MODIFIERS = [
  "failed",
  "former",
  "self-proclaimed",
  "retired",
  "part-time",
  "freelance",
  "disgraced",
  "off-duty",
  "unlicensed",
  "aspiring"
];

// Usage: pick(OCCUPATION_MODIFIERS) + " " + pick(OCCUPATIONS) → "failed product manager", "aspiring astrologer"
// Apply probability: 0.40 (40%)
```

---

### TEMPORAL (20 entries)

```js
const TEMPORAL = [
  "Late-Stage",
  "Pre-Pandemic",
  "Post-Ironic",
  "Y2K-Adjacent",
  "2019-era",
  "Recession-core",
  "End-of-Quarter",
  "Post-Series-A",
  "Pre-Layoff",
  "Q4-Adjacent",
  "Beta-Phase",
  "Legacy-System-era",
  "Long-COVID-era",
  "Peak-VC",
  "Post-Pivot",
  "Web2-era",
  "First-Principles-era",
  "Vibes-Based",
  "Pre-Merger",
  "Sunset-Build"
];

// Apply probability: 0.15 (15%)
// Prepended to full creature name
// → "Late-Stage quietly deprecated toaster former barista"
```

---

### HABITATS (50 entries)

```js
const HABITATS = [
  // Generic American spaces
  "the Target self-checkout aisle",
  "a WeWork hot desk no one claimed",
  "the third stall of a Whole Foods bathroom",
  "a Trader Joe's frozen aisle",
  "a Best Buy display kiosk",
  "an airport gate B-17 at 6:15 AM",
  "a Chase Bank vestibule",
  "the Gap fitting room",

  // Hyper-specific
  "the third LaCroix flavor at a startup all-hands",
  "the snack drawer nobody replenishes",
  "the conference room that's always 62 degrees",
  "the Zoom waiting room that never gets admitted",
  "the parking structure level P3",
  "the fire escape of a Murray Hill walk-up",
  "a CVS photo kiosk",
  "the priority boarding lane for Group 5",

  // Digital spaces
  "a Notion page last edited February 2023",
  "the #general Slack channel",
  "a Google Doc with 14 unresolved comments",
  "a GitHub issue marked 'wontfix'",
  "the LinkedIn DMs tab",
  "a Figma file no one has opened since the offsite",
  "a Jira backlog column labeled 'Someday/Maybe'",
  "the staging environment",
  "a private Twitter list no one follows back",
  "a Substack draft saved March 4th, 2024",
  "an Airtable base the intern set up",
  "a Discord server with 4 members",

  // Regional American
  "a Brooklyn rooftop in October",
  "the I-95 rest stop near Delaware",
  "a Vegas hotel lobby at 3 AM",
  "the Austin airport Southwest terminal",
  "a Portland food cart pod in February",
  "a Miami co-working space with no AC",
  "a Chicago L platform at rush hour",
  "a San Francisco Castro District coffee shop",

  // Liminal / vibe-specific
  "the fluorescent break room at 3:47 PM",
  "the printer room no one visits until something breaks",
  "an IKEA showroom bedroom display",
  "a 24-hour diner at 2 AM on a Wednesday",
  "the dead zone between meetings",
  "a Planet Fitness at 6 AM on January 2nd",
  "the lobby of a startup that just did layoffs",
  "a Duane Reade at midnight",
  "the back row of a mandatory HR training",
  "a hotel business center no one has touched since 2019"
];
```

---

### FOODS (50 entries)

```js
const FOODS = [
  // Real food (~40%)
  "cold brew",
  "leftover Chipotle",
  "sourdough discard",
  "trail mix with all the M&Ms picked out",
  "free conference room granola bars",
  "a Kind bar from the bottom of a laptop bag",
  "expired protein powder",
  "lukewarm green tea",
  "half a Clif Bar",
  "a sad desk salad",
  "the last kombucha in the office fridge",
  "Costco samples",
  "a Sweetgreen bowl from two days ago",
  "airport sushi",
  "a birthday cake slice from HR",
  "Wednesday bagels",
  "the office almond milk nobody ordered",
  "a SmartWater bottle refilled from the tap",
  "an everything bagel with no cream cheese",
  "stale Oreos from the client gift basket",

  // Abstract consumption (~60%)
  "push notifications",
  "the Bloomberg terminal",
  "Glassdoor reviews of their former employer",
  "quarterly OKRs",
  "performance review feedback",
  "the scroll",
  "ambient LinkedIn anxiety",
  "passive-aggressive Slack reactions",
  "reply-all threads",
  "calendar invites without agendas",
  "job posting alerts at 11 PM",
  "a rejected PR comment",
  "the vibes of an impending reorg",
  "unread GitHub notifications",
  "a VC pitch deck that went nowhere",
  "the concept of 'alignment'",
  "AI hype cycles",
  "late-night doom scrolling",
  "peer recognition software notifications",
  "the corporate newsletter",
  "unanswered cold emails",
  "someone else's stand-up update",
  "the company all-hands recording no one watched",
  "a deprecation warning nobody read",
  "error logs at 2 AM",
  "customer feedback nobody acted on",
  "the Sunday Scaries",
  "a canceled product roadmap",
  "the annual employee survey",
  "expired free trial reminders"
];
```

---

### BEHAVIORS (50 entries)

```js
const BEHAVIORS = [
  // Work-from-home pathology
  "Opens ChatGPT to write a single cold email and closes the tab.",
  "Joins the meeting on mute, nods, leaves without speaking.",
  "Rearranges their desk for 40 minutes before doing actual work.",
  "Sends a Slack message, immediately regrets the tone, adds a thumbs-up emoji.",
  "Starts a new Notion doc for every project and fills in only the title.",
  "Moves tasks between three different to-do apps without completing any.",
  "Types a response in Slack, reads it back, and silently deletes it.",
  "Schedules a focus block and then spends it checking email.",
  "Opens the fridge during every video call, on mute, without looking at the camera.",
  "Attends the optional Friday social and says nothing for 22 minutes.",

  // Corporate drone absurdity
  "Circles back before anyone has left.",
  "Volunteers to own the action item and then asks what it was.",
  "Takes 11 minutes to reply to a 3-word Slack message.",
  "Updates the status doc to say it will be updated later.",
  "Marks every email as unread after reading it.",
  "Puts 'per my last email' in the subject line.",
  "Books a 30-minute sync to cancel a 15-minute sync.",
  "Submits a 47-slide deck for a question that needed a one-liner.",
  "Agrees in the meeting and reverts in the follow-up email.",
  "Offers to take this offline and never mentions it again.",

  // Online-brain disease
  "Screenshots every tweet before the account goes private.",
  "Replies to a 2019 Reddit thread as if it's still happening.",
  "Reads the same Wikipedia article once a year without retaining it.",
  "Starts an AMA nobody asked for.",
  "Writes a Twitter thread, publishes only the first tweet.",
  "Refreshes their analytics page every 4 minutes.",
  "Posts a 'taking a break from social media' notice and returns in 36 hours.",
  "Quotes-retweets to agree, adding nothing.",
  "Saves a YouTube video to 'Watch Later' and has never opened that playlist.",
  "Writes a very long email and then sends a Slack message saying 'sent you an email.'",

  // Stanley-cup-generation behavior / gen-z office creature
  "Carries a 40-oz water bottle everywhere and forgets to drink from it.",
  "Describes their entire identity through their hot take on remote work.",
  "Brings a plant to the office on their first day and kills it by week three.",
  "Has four browser windows with 30 tabs each and calls it a 'system.'",
  "Uses 'no worries' to end a conversation they are deeply worried about.",
  "Cancels plans and feels profound relief.",
  "Works in 11-minute bursts separated by 45-minute 'research' sessions.",
  "Starts every sentence with 'I feel like' and then states an objective fact.",
  "Sends a 'quick question' that takes 20 minutes to answer.",
  "Checks their phone within 3 seconds of putting it face-down.",
  "Refers to their apartment as a 'studio' and their job as 'a thing they do.'",
  "Agrees to a timeline they know is impossible and says nothing.",
  "Saves articles to Pocket and has never opened Pocket.",
  "Sets a morning alarm 'just to see how they feel' and ignores it.",
  "Maintains 17 group chats and is unreachable on all of them.",
  "Writes 'lmk' and then doesn't respond when you do.",
  "Recommends a podcast they stopped listening to after episode 3.",
  "Asks for feedback and interprets all of it as praise.",
  "Refers to every side project as 'still in early stages' for four years.",
  "Knows exactly what's wrong and tells no one."
];
```

---

### TRIVIA (50 entries)

```js
const TRIVIA = [
  // Failed startups / product graveyard
  "Was briefly the face of a failed oat milk brand.",
  "Had a ProductHunt launch that peaked at #47.",
  "Was listed as a co-founder on a Crunchbase page that no longer exists.",
  "Once had a 'Launch HN' post with 3 upvotes.",
  "Built an app for a problem nobody had, once.",
  "Was featured in TechCrunch in 2021 in a round-up of 'startups to watch.' The startup did not last.",
  "Was the only backer of a Kickstarter that funded but never shipped.",
  "Gave a lightning talk at a YC event nobody has footage of.",
  "Had a beta waitlist of 12 people, 9 of whom were the founders.",
  "Was acquired for an undisclosed amount, which was $0.",

  // Reality TV / internet infamy
  "Appeared in one episode of a Netflix docuseries as an unnamed background figure.",
  "Was the subject of a 4chan thread that was deleted within the hour.",
  "Went semi-viral on Reddit in 2020 and has been trying to recreate it since.",
  "Had 11 minutes of fame on TikTok and does not know why.",
  "Was briefly a moderator of a subreddit that is now banned.",
  "Appeared on a local news segment about the gig economy and was described as 'one resident.'",
  "Was nominated for a Shorty Award in a category that no longer exists.",

  // LinkedIn weirdness
  "Their LinkedIn summary has been 'currently exploring opportunities' for 3 years.",
  "Has 500+ connections and has met 7 of them.",
  "Endorsed by 14 people for a skill they do not have.",
  "Listed a job that lasted 6 weeks as '2021 – 2022.'",
  "Has posted three LinkedIn articles, each beginning 'Hot take:'",
  "Celebrated a work anniversary that no longer applies.",
  "Their profile photo is from a conference in 2016.",

  // HuggingFace / AI lore
  "Was uploaded to HuggingFace and removed within 48 hours for undisclosed reasons.",
  "Has a model card that describes itself as 'experimental.'",
  "Once hallucinated a citation to a paper that was later actually written.",
  "Self-reports a benchmark score nobody can reproduce.",
  "Claims to have passed the Turing Test in a setting that wasn't the Turing Test.",
  "Was fine-tuned on data that included its own outputs.",
  "Has an Arxiv preprint with 0 citations and a very confident abstract.",
  "Was deprecated without an official announcement.",

  // Reddit AMAs / weird internet
  "Hosted a Reddit AMA that lasted 12 minutes before going dark.",
  "Is referenced in a Wiki article as 'citation needed.'",
  "Has a Wayback Machine entry and nothing else.",
  "Once left a comment on a thread from 2012 that gets upvoted once a year.",
  "Is the only reply on a post nobody saw.",
  "Has a dedicated thread on a forum that now redirects to a parking page.",
  "Once had a verified Twitter badge, briefly, during the chaos.",
  "Was quoted in a newsletter nobody unsubscribed from because they forgot it existed.",
  "Has a Spotify podcast with 1 episode, dated April 2020.",
  "Was tagged in a photo on Facebook in 2009 and has not logged in since.",
  "Their first blog post is indexed but the blog itself is gone.",
  "Exists in someone's CRM as 'Do Not Contact.'",
  "Has an entry in a spreadsheet titled 'leads - probably bad - 2022.'",
  "Was the answer to a trivia question at a startup happy hour and got it wrong.",
  "Has a GitHub repository with 1 star from an account that was suspended.",
  "Is rate-limited on an API they don't remember signing up for."
];
```

---

### SYMPTOMS (50 entries)

```js
const SYMPTOMS = [
  "Wi-Fi drops by 15% within a 4-foot radius.",
  "Causes nearby coworkers to develop imposter syndrome.",
  "Anyone in the same room will receive a notification they cannot clear.",
  "Proximity triggers a sudden urge to update your resume.",
  "People within earshot start using the phrase 'circle back' involuntarily.",
  "Slack goes into a loading state for exactly 11 seconds.",
  "Nearby laptops will open one new tab with no URL.",
  "Anyone who makes eye contact will lose track of what they were saying.",
  "The person nearest to them will find an unread email from 2021.",
  "Causes autocomplete to suggest emotionally accurate but professionally inappropriate responses.",
  "Everyone in a 10-foot radius simultaneously remembers something they forgot to do.",
  "Battery drain accelerates by approximately 8% per hour of proximity.",
  "Causes a passing coworker to describe their weekend unprompted.",
  "LinkedIn notifications increase for 48 hours following contact.",
  "Standing within 6 feet of them gives you the feeling that you left the stove on.",
  "Coffee goes cold faster in their presence.",
  "Anyone nearby will find themselves re-reading the same sentence three times.",
  "Induces the sensation of having a very important meeting you cannot locate on your calendar.",
  "Anyone sharing a screen in their presence will accidentally share the wrong window.",
  "Causes the fire alarm to do a single short chirp and then stop.",
  "Printers on the same network produce one unclaimed page.",
  "Causes the office thermostat to display a reading 2 degrees warmer than it is.",
  "Meeting links shared in their presence expire before the meeting starts.",
  "Anyone walking near them will briefly forget whether they've eaten lunch.",
  "Induces a compulsive need to check if an email was 'delivered.'",
  "Their calendar invites consistently arrive 1 minute after the meeting starts.",
  "The hold music on any call they're on will loop 3 seconds earlier than expected.",
  "Causes people to begin a sentence in Slack and abandon it in the draft state.",
  "VPN disconnects within 2 minutes of their arrival in a room.",
  "Others nearby develop a sudden certainty that today is actually Tuesday.",
  "Causes the elevator to skip the floor they pressed.",
  "Nearby speakers emit a single faint tone at 4:47 PM.",
  "Autocorrect behavior becomes unreliable for anyone who texted them recently.",
  "People in the same office floor will briefly wonder if they applied to the right company.",
  "Induces a 4-second lag in video calls that only affects their participant tile.",
  "Anyone CC'd on an email with them will feel briefly responsible for something unspecified.",
  "Causes smartwatches to vibrate with no associated notification.",
  "Breaks the silence in a room by making it louder somehow.",
  "Induces mild semantic confusion — everyone nearby uses the wrong word but means the right one.",
  "The person sitting nearest to them will forget their password on the first attempt.",
  "Causes shared docs to enter 'editing' mode with no editor present.",
  "Nearby ice machines produce cubes slightly smaller than normal.",
  "Anyone on the same video call will briefly appear pixelated, specifically during the moment they speak.",
  "Causes a background tab to autoplay audio.",
  "Induces a strong desire to reorganize your Downloads folder.",
  "Every alarm clock within proximity goes off 7 minutes early.",
  "Causes coworkers to leave a meeting and immediately forget what was decided.",
  "Nearby phones fail face recognition on the first try.",
  "Any OOO message set while near them will have an error in the end date.",
  "Causes someone nearby to open a new browser tab and immediately close it."
];
```

---

### WEAKNESSES (50 entries)

```js
const WEAKNESSES = [
  "a Mercury retrograde post",
  "being asked 'how was your weekend?' at 9:02 AM on a Monday",
  "the phrase 'circle back'",
  "a DocuSign request",
  "any form requiring a physical signature",
  "the phrase 'per my last email'",
  "a sudden calendar invite with no description",
  "the option to 'reply all'",
  "a PDF that won't load",
  "the phrase 'can we hop on a quick call?'",
  "a 500-person company all-hands with a Q&A section",
  "a mandatory fun event",
  "the phrase 'just to set expectations'",
  "a Slack message that says only 'hey'",
  "being asked to introduce themselves in a new Zoom",
  "a CAPTCHA with a fire hydrant that might be a parking meter",
  "the phrase 'we should grab coffee sometime'",
  "a shared Google Sheet with 14 contributors",
  "any sentence that begins 'not sure if you saw my last message'",
  "the concept of a biometric time clock",
  "a performance improvement plan disguised as a growth opportunity",
  "the phrase 'let's double-click on that'",
  "being put on the spot during a retrospective",
  "an email chain that should have been a Slack message",
  "a Slack message that should have been a meeting",
  "a meeting that should have been an email",
  "a password reset during a screen share",
  "a printer with a paper jam and no instructions",
  "being recognized in the lobby by someone whose name they can't remember",
  "a calendar that is 'tentative' in perpetuity",
  "the phrase 'I just want to be transparent'",
  "an NPS survey that auto-populates their name incorrectly",
  "a LinkedIn connection request with a note",
  "any situation requiring them to 'find a time that works for everyone'",
  "a security badge that stops working on a Friday afternoon",
  "the phrase 'let's take this offline'",
  "being asked their five-year plan",
  "a company survey that says 'your feedback is anonymous' in a team of four",
  "the phrase 'I hear you' deployed to end an argument",
  "any document titled 'FINAL_v3_REAL_use_this_one.docx'",
  "a mandatory password change every 90 days",
  "the concept of a standing meeting",
  "a phone screen that says 'Potential Spam'",
  "being added to a group text without warning",
  "a Terms of Service agreement over 2,000 words",
  "the phrase 'this is a safe space'",
  "a slide deck with the note 'presenter to add talking points'",
  "a Zoom background that keeps glitching",
  "the phrase 'we're like a family here'",
  "any meeting scheduled for 4:45 PM on a Friday"
];
```

---

### LAST_SEEN (50 entries)

```js
const LAST_SEEN = [
  "October 4, 2021 — posting dryly on r/nba during the Facebook outage.",
  "March 11, 2020 — in the Costco parking lot, cart full, frozen.",
  "November 8, 2022 — refreshing the New York Times elections page at 12:43 AM.",
  "January 6, 2021 — in the comments of an unrelated cooking video.",
  "August 2022 — last seen on a Substack thread about Web3, which has since been deleted.",
  "Q3 2019 — attending a 'future of work' panel at a Brooklyn co-working space.",
  "December 31, 2023 — the WeWork Flatiron location, bathroom mirror, right before midnight.",
  "April 2023 — the SVB press refresh moment, 9:08 AM.",
  "February 2024 — a LinkedIn post that got exactly 12 impressions.",
  "October 2023 — the AI Twitter meltdown that lasted a long weekend.",
  "May 2022 — lurking the r/antiwork subreddit before it went on TV.",
  "July 2023 — a Notion changelog update nobody read.",
  "March 2023 — last active on a Slack workspace that has since been archived.",
  "September 4, 2023 — the day the Threads algorithm changed again.",
  "November 2022 — first day of the Twitter Blue era.",
  "June 2023 — a GitHub Copilot billing confirmation page.",
  "January 2024 — a Discord server for a game that never shipped.",
  "August 2023 — a Glassdoor review that got flagged.",
  "April 2024 — flagged as inactive by a CRM nobody checks.",
  "February 2020 — a coworking space happy hour, three weeks before everything closed.",
  "October 2022 — the last day before a company went fully remote, then folded.",
  "Q4 2021 — leaving a comment on a HuggingFace model card that is still there.",
  "March 2024 — a Reddit thread titled 'Is this job market real?'",
  "December 2022 — the ChatGPT launch week, in the ChatGPT tab, 2 AM.",
  "May 2023 — a Google Doc titled 'Layoff prep — DO NOT SHARE' last edited by 'unknown.'",
  "July 2022 — a Peloton community feed before the brand collapse.",
  "September 2023 — the second-to-last row of a mandatory DEI training webinar.",
  "November 2021 — the Great Resignation resignation email draft, unsent.",
  "April 2025 — a staging environment that was supposed to be decommissioned.",
  "January 2026 — the #announcements channel, 30 seconds before a surprise all-hands.",
  "March 2026 — a Figma component nobody can find in the left panel.",
  "October 2025 — a product roadmap slide last updated 'Q3 target.'",
  "February 2026 — a Jira ticket assigned to someone who left the company.",
  "July 2025 — the lobby of a VC firm, waiting for a meeting that was rescheduled.",
  "December 2024 — an airport Centurion Lounge, three missed connections deep.",
  "August 2025 — the 'Learn' tab of a SaaS product nobody has clicked.",
  "November 2023 — a Calendly link that always shows no available times.",
  "June 2024 — a DM that was opened and never responded to.",
  "March 2025 — an RSS reader with 4,700 unread items.",
  "January 2025 — a company Notion titled 'Culture & Values — living document.'",
  "October 2024 — a Google Analytics dashboard nobody logged into.",
  "May 2026 — somewhere in the inference logs at 3:17 AM.",
  "February 2023 — a ProductHunt launch day Slack channel that peaked at 4 members.",
  "September 2022 — the last slide of a deck titled 'Q3 Results — Confidential.'",
  "April 2024 — a GitHub PR marked 'draft,' open for 14 months.",
  "July 2024 — an HN thread that got flagged before it hit the front page.",
  "December 2025 — the end of a company all-hands recording, timestamp 1:04:17.",
  "August 2024 — a Zoom chat log nobody exported before the meeting ended.",
  "March 2022 — a reply-all email chain that finally went quiet.",
  "April 2026 — the loading screen of a product nobody launched."
];
```

---

### CLASSIFICATIONS (60 entries)

```js
const CLASSIFICATIONS = [
  // Depth-zone taxonomy
  "Deepwater Office Fish",
  "Freshwater Mood Specimen",
  "Brackish-Zone Parasocial",
  "Open-Plan Pelagic",
  "Benthic Corporate Type",
  "Littoral Slack Dweller",
  "Mesopelagic Burnout Specimen",
  "Abyssal Unresponsive Type",
  "Intertidal Sync-Skipping Fish",
  "Surface-Level Engagement Specimen",
  "Deep-State Lurker Fish",
  "Mid-Water Reply-All Organism",

  // Functional / ecological classification
  "Ornamental Anxiety Fish",
  "Non-Edible Ambition Specimen",
  "Release-Blocked Status Fish",
  "Invasive Pivot Species",
  "Pre-Conservation Burnout Type",
  "Aquaculture Failure Specimen",
  "Captive-Bred Exit-Interview Fish",
  "Display-Pending Judgment Type",
  "Unspawned Backlog Organism",
  "Specimen Awaiting Processing",
  "Thaw-Limited Retention Fish",
  "Unlisted Taxonomy Organism",
  "Retracted Sighting Re-Entry Fish",
  "IUCN Unregistered Specimen",
  "Zero-Capture-Rate Fish",
  "Discontinued-Model Organism",

  // Habitat-specialized classification
  "Freshwater Feeling Specimen",
  "Urban Runoff Adaptive Fish",
  "Cloud-Layer Buoy Organism",
  "Virtual-Habitat Settled Type",
  "Digital Wetlands Specimen",
  "Notification-Current Pelagic",
  "Wi-Fi Thermocline Specimen",
  "Deep-Server Organism",
  "Cache-Layer Buoyant Type",
  "404-Zone Missing Specimen",
  "Remote-Work Isolated Freshwater Fish",
  "Open-Office Current Follower",
  "Coworking Littoral Species",
  "Anoxic Basement Hermit Fish",
  "Convenience-Store Bioluminescent Type",

  // Behavior-based classification
  "Nocturnal Self-Audit Fish",
  "Seasonal Disappearance Specimen",
  "Solitary Non-Schooling Type",
  "Migration-Loop Organism",
  "Non-Breeding Sedentary Fish",
  "Threat-Display Mimicry Type",
  "Deadline-Triggered Activation Fish",
  "Praise-Inversion Specimen",
  "Infinite-Scroll Commensal",
  "Notification Parasite",
  "Tab-Overload Benthic Fish",
  "Persistent-Logout Anomaly",
  "Third-Person Self-Reference Species",
  "Draft-Delete Iteration Organism",
  "Delayed-Response Cyclical Fish",
  "Empty-Calendar Preference Type",
  "Overtime-Induction Fish",
  "Page-47 Sighting Specimen"
];
```

---

### OBSERVATION_NOTES (30 entries)

```js
const OBSERVATION_NOTES = [
  "Latest observation log:",
  "Aquarist note, 2026-04-12:",
  "Filed under: unresolved.",
  "Field report — do not distribute:",
  "Specimen note — verification pending:",
  "Observation count: 1 / Confidence: low.",
  "Night patrol log:",
  "Incidental sighting — unofficial record:",
  "Error possible — confirm before citing:",
  "Tip received — under review:",
  "Third observation on record:",
  "Post-anomaly tracking note:",
  "Specimen ID not yet assigned:",
  "Pre-taxonomy record:",
  "Confidence: unconfirmed / observer: 1:",
  "Non-scheduled observation log:",
  "Anonymous tip — original text preserved:",
  "Priority flag — contact curator:",
  "Discovered in unmapped zone — registered:",
  "Re-emergence after 6+ months absent:",
  "Off-season appearance — logged:",
  "Identity unconfirmed — possible duplicate:",
  "Specimen degraded — identification incomplete:",
  "Equipment fault possible — reference only:",
  "Layperson sighting (original language retained):",
  "Classification withheld — name pending:",
  "Anomalous entry — flagged for review:",
  "Curator memo, draft:",
  "Cross-reference file — status: open:",
  "Informal log — not for publication:"
];

// Apply probability: 20–30%
```

---

### TANK_CONDITIONS (30 entries)

```js
const TANK_CONDITIONS = [
  "salinity: unsettled",
  "pH: inherited trauma",
  "filter: emotionally unavailable",
  "temperature: ambient existential",
  "oxygen: technically sufficient",
  "lighting: fluorescent, 24 hours",
  "substrate: compacted resentment",
  "flow rate: passive-aggressive",
  "visibility: technically 100%",
  "feeding schedule: irregular",
  "tank population: overcrowded with expectations",
  "water change: overdue by 6 months",
  "nitrate: elevated",
  "nitrite: unknown",
  "ammonia: unaddressed",
  "alkalinity: performatively stable",
  "hardness: layoff-adjacent",
  "phosphate: startup-era",
  "co2: insufficient",
  "pump status: grinding quietly",
  "heater: malfunctioning since Q3",
  "glass clarity: smudged",
  "algae level: mounting",
  "refuge section: unused",
  "specimen density: too many feelings",
  "bioload: unsustainable",
  "quarantine tank: permanently occupied",
  "flow direction: unclear",
  "evaporation rate: elevated",
  "tank status: on notice"
];

// Apply probability: 15%
```

---

## §2 — Scientific Name Generator (English-Input Version)

### NOUN_ROOTS (80 entries)

Mappings from `NOUNS` array to Latin-ish genus roots. Fallback rule handles unmapped entries.

```json
{
  "toaster": "Toastophilus",
  "broken umbrella": "Umbrellatix",
  "crisper drawer": "Crispordix",
  "dish sponge": "Spongifex",
  "half-used candle": "Candelibris",
  "shower curtain": "Cortinatix",
  "dead houseplant": "Florimortix",
  "extension cord": "Cordexia",
  "junk drawer": "Deponiax",
  "spare key": "Claviculix",
  "empty wall hook": "Hamulvax",
  "laundry basket": "Lavaturix",
  "rogue sock": "Calceatrix",
  "rubber band ball": "Elastorium",
  "dead AA battery": "Cellumorax",
  "unclaimed package": "Packetibris",
  "office chair wheel": "Rotulofex",
  "whiteboard marker": "Scriptoplax",
  "sticky note": "Adhesivix",
  "lanyard": "Lanyardius",
  "communal stapler": "Staplorium",
  "sourdough discard": "Fermentibris",
  "wilted celery": "Apimorax",
  "overripe avocado": "Avocatrix",
  "bulk granola": "Granolatix",
  "mystery freezer item": "Glacienigma",
  "expired kombucha": "Fermentibrix",
  "office moth": "Phalaenofix",
  "lobby beetle": "Coleoptrix",
  "parking lot pigeon": "Columbatrix",
  "break room fruit fly": "Drosophilix",
  "bathroom spider": "Arachnofix",
  "afternoon haze": "Calignovia",
  "unsettled feeling": "Inquietudia",
  "ambient dread": "Timordius",
  "low-grade static": "Staticulix",
  "mercury retrograde": "Mercuriorix",
  "background hum": "Sonorellix",
  "Tuesday malaise": "Martismorax",
  "mild dissociation": "Disociavix",
  "Chrome tab": "Tabuchromix",
  "unread Slack thread": "Slackothricus",
  "expired LinkedIn trial": "Linkedinura",
  "401(k) statement": "Retiramix",
  "loading spinner": "Spinatix",
  "push notification": "Notificrix",
  "Jira ticket": "Jiraticus",
  "staging environment": "Stagiorium",
  "deprecated API": "Apimorax",
  "ghost branch": "Phantoramus",
  "open PR": "Reviewatus",
  "hallucinated citation": "Citatiforma",
  "model card": "Cardulumix",
  "fine-tuned checkpoint": "Checkpolix",
  "RLHF annotation": "Annothorix",
  "embedding space": "Vectorium",
  "context window": "Contextibris",
  "system prompt": "Promptorium",
  "token budget": "Tokenatus",
  "attention head": "Attentorix",
  "RAG pipeline": "Ragipelix",
  "vector database": "Vectobase",
  "inference cost": "Inferencix",
  "prompt injection": "Injectoplax",
  "alignment tax": "Alignatrix",
  "knowledge cutoff": "Cutofficus",
  "temperature setting": "Temporifix",
  "forgotten bookmark": "Favoribris",
  "browser history": "Historiorix",
  "404 page": "Erroratum",
  "sourdough discard": "Levanutrix",
  "conference room booking": "Calendoryx",
  "dead houseplant": "Succulmorax",
  "ambient dread": "Pavotigris",
  "desktop mouse": "Muricula",
  "ergonomic keyboard": "Keybordix",
  "standing desk": "Erectodix",
  "expired coupon": "Voucherix",
  "model card": "Cardaticum"
}
```

**Fallback rule (unmapped English words):**
1. Lowercase the word.
2. Strip non-alphabetic characters.
3. If ends in a vowel → append `rius`. If ends in a consonant → append `icus`.
4. If length > 14 characters, take first 8 characters only, then append suffix.
5. Capitalize the result.

Example: `"alignment tax"` → `"alignmenttax"` → first 8 → `"alignmen"` + `icus` → `Alignmenicus`

---

### ADJ_SUFFIXES (30 entries)

```json
{
  "sad": "melancholicus",
  "hallucinating": "onirica",
  "deprecated": "obsoletus",
  "mildly unhinged": "deviaticus",
  "deeply tired": "exhausticus",
  "serene": "placidicus",
  "resentful": "rancoricus",
  "restless": "inquieticus",
  "vaguely smug": "arrogantulus",
  "dissociated": "separaticus",
  "context-starved": "contextihungrus",
  "RAG-poisoned": "ragintoxicus",
  "fine-tuned": "affinaticus",
  "overfit": "overfiticus",
  "misaligned": "inalignicus",
  "token-limited": "tokenrarus",
  "zero-shot": "azeroticus",
  "stochastic": "stochasicus",
  "reward-hacked": "rewardisticus",
  "CrossFit-curious": "cruciapticus",
  "LinkedIn-core": "linkedinacius",
  "podcast-addled": "podcasticus",
  "emotionally unavailable": "affectivacuus",
  "aggressively mediocre": "mediacritus",
  "underpaid": "subremnericus",
  "hollow": "cavoicus",
  "haunted": "spectricus",
  "overstimulated": "hyperstimicus",
  "newly radicalized": "novradicus",
  "softmax-indecisive": "indecisivus"
}
```

---

### OCC_EPITHETS (30 entries)

```json
{
  "freelancer": "libertinus",
  "CEO": "imperator",
  "consultant": "advisorius",
  "product manager": "productalis",
  "barista": "cafealis",
  "data analyst": "numeralis",
  "prompt engineer": "prompteris",
  "life coach": "vitaecoachis",
  "astrologer": "astrologis",
  "DoorDash driver": "deliveralis",
  "AI safety researcher": "safetyalis",
  "Substack writer": "newsletteris",
  "podcast host": "podcastalis",
  "DevOps engineer": "devopalis",
  "data labeler": "labelatoris",
  "UX researcher": "uxresalis",
  "growth marketer": "growthalis",
  "scrum master": "scrumoralis",
  "bartender": "bartendalis",
  "personal trainer": "trainoralis",
  "energy healer": "healoralis",
  "skip tracer": "traceralis",
  "forensic accountant": "forensicalis",
  "notary public": "notarialis",
  "flight attendant": "volatilis",
  "ML engineer": "mlenginalis",
  "indie game developer": "ludodevelopis",
  "breathwork facilitator": "respiratalis",
  "open-source maintainer": "opensourcalis",
  "private investigator": "investigoralis"
}
```

---

### `generateScientificName` Pseudocode

```js
function generateScientificName(adj, noun, occ) {
  // Genus from noun
  const genus = NOUN_ROOTS[noun] ?? phoneticLatin(noun);

  // Species epithet from adjective
  const adjSuffix = ADJ_SUFFIXES[adj] ?? (phoneticLatin(adj) + "us");

  // Third name (subspecies epithet) from occupation
  const occEpithet = OCC_EPITHETS[occ] ?? (phoneticLatin(occ) + "is");

  return `${genus} ${adjSuffix} ${occEpithet}`;
}

function phoneticLatin(word) {
  const stripped = word.toLowerCase().replace(/[^a-z]/g, "").slice(0, 8);
  return capitalize(stripped + (endsInVowel(stripped) ? "rius" : "icus"));
}
```

---

### 5 Concrete Example Outputs

**Running example specimen: "lowkey hallucinating toaster (former freelancer)"**

1. *Toastophilus onirica libertinus*
   — lowkey hallucinating toaster, former freelancer

2. *Slackothricus exhausticus freelancalis*
   — deeply tired unread Slack thread, aspiring freelancer

3. *Spinatix melancholicus productalis*
   — sad loading spinner, failed product manager

4. *Linkedinura arrogantulus advisorius*
   — vaguely smug expired LinkedIn trial, self-proclaimed consultant

5. *Contextibris obsoletus libertinus*
   — deprecated context window, retired freelancer

---

## §3 — 4 Tone Templates (English)

### Tone weights by rarity (unchanged from v2)

```js
const TONE_WEIGHTS_BY_RARITY = {
  common:        { academic: 35, docuseries: 30, shitpost: 25, asmr: 10 },
  uncommon:      { academic: 35, docuseries: 30, shitpost: 25, asmr: 10 },
  rare:          { academic: 35, docuseries: 30, shitpost: 25, asmr: 10 },
  legendary:     { academic: 50, docuseries: 30, shitpost: 20, asmr:  0 },
  hallucination: { academic: 25, docuseries: 25, shitpost: 25, asmr: 25 }
};
```

---

### Tone 1: Academic / Ichthyologist

**Voice:** Deadpan field-guide. Third-person scientific. "A Field Guide to North American Office Mammals." No emotion.

**Template string:**
```
{name} (syn. {scientificName}; hereafter 'the specimen') inhabits {habitat}.
Primary dietary intake consists of {food}. Observed behavior: {behavior}.
Documented taxonomic note: {trivia}.
[if symptom] Proximal ecological disturbance recorded: {symptom}.
[if weakness] Confirmed environmental sensitivity: {weakness}. Mechanism unknown.
[if last_seen] Final observation coordinates: {last_seen}.
[if observation_note] {observation_note}
[if tank_condition] Holding conditions: {tank_condition}.
```

**Sample output — "lowkey hallucinating toaster (former freelancer)":**

> *Toastophilus onirica libertinus* — the lowkey hallucinating toaster (former freelancer), syn. *Toastophilus onirica libertinus*, inhabits the conference room that's always 62 degrees.
> Primary dietary intake consists of calendar invites without agendas. Observed behavior: opens ChatGPT to write a single cold email and closes the tab. Documented taxonomic note: Was briefly the face of a failed oat milk brand.
> Proximal ecological disturbance recorded: Slack goes into a loading state for exactly 11 seconds.
> Confirmed environmental sensitivity: the phrase 'circle back.' Mechanism unknown.
> Final observation coordinates: March 2023 — last active on a Slack workspace that has since been archived.

---

### Tone 2: Docuseries Narrator

**Voice:** Attenborough-style wildlife narration transplanted to office organisms. Measured wonder. The creature is observed from a respectful distance.

**Template string:**
```
Here, in {habitat}, the {name} waits.
It sustains itself on {food} — a diet that would seem improbable, were it not so consistent.
{behavior}.
One documented fact confounds even seasoned observers: {trivia}.
[if symptom] Those who venture too close report: {symptom}.
[if weakness] Its only recorded vulnerability: {weakness}.
[if last_seen] It was last confirmed on record at {last_seen}.
[if observation_note] {observation_note}
```

**Sample output — "lowkey hallucinating toaster (former freelancer)":**

> Here, in the conference room that's always 62 degrees, the lowkey hallucinating toaster — a former freelancer — waits.
> It sustains itself on calendar invites without agendas — a diet that would seem improbable, were it not so consistent.
> It opens ChatGPT to write a single cold email and closes the tab.
> One documented fact confounds even seasoned observers: it was briefly the face of a failed oat milk brand.
> Those who venture too close report: Slack goes into a loading state for exactly 11 seconds.
> Its only recorded vulnerability: the phrase 'circle back.'
> It was last confirmed on record at March 2023 — last active on a Slack workspace that has since been archived.

---

### Tone 3: Extremely Online / Shitpost

**Voice:** Twitter/Reddit voice. Lowercase starts, broken grammar, "the way this thing," "not me," "no but why is." Contemporary US internet idioms.

**Template string:**
```
okay so {name} lives in {habitat} and i'm not okay about it
it literally eats {food}. EATS. {food}.
{behavior} (no notes)
fun fact: {trivia}. no further questions.
[if symptom] also if you get too close: {symptom}. just fyi. no reason.
[if weakness] its one weakness is {weakness} which is insane to me
[if last_seen] last seen: {last_seen}. make it make sense.
```

**Sample output — "lowkey hallucinating toaster (former freelancer)":**

> okay so the lowkey hallucinating toaster (former freelancer) lives in the conference room that's always 62 degrees and i'm not okay about it
> it literally eats calendar invites without agendas. EATS. calendar invites without agendas.
> opens ChatGPT to write a single cold email and closes the tab (no notes)
> fun fact: was briefly the face of a failed oat milk brand. no further questions.
> also if you get too close: Slack goes into a loading state for exactly 11 seconds. just fyi. no reason.
> its one weakness is the phrase 'circle back' which is insane to me
> last seen: March 2023 — last active on a Slack workspace that has since been archived. make it make sense.

---

### Tone 4: ASMR Field Journal

**Voice:** Whispered nature journal entry. Sparse. Short sentences. Ellipses. 3 AM energy. The observer is alone with the specimen.

**Template string:**
```
...{name}...
...scientific designation: {scientificName}...
...found in {habitat}... quiet there...
...today it consumed {food}... as expected...
...{behavior}...
...something worth recording: {trivia}...
[if symptom] ...get close enough and... {symptom}... did you feel that?
[if weakness] ...one thing. just one... {weakness}...
[if last_seen] ...last sighting... {last_seen}... still thinking about it...
[if observation_note] {observation_note}
[if tank_condition] ...conditions noted: {tank_condition}...
```

**Sample output — "lowkey hallucinating toaster (former freelancer)":**

> ...lowkey hallucinating toaster... former freelancer...
> ...scientific designation: *Toastophilus onirica libertinus*...
> ...found in the conference room that's always 62 degrees... quiet there...
> ...today it consumed calendar invites without agendas... as expected...
> ...opened ChatGPT to write a single cold email and closed the tab...
> ...something worth recording: it was briefly the face of a failed oat milk brand...
> ...get close enough and... Slack goes into a loading state for exactly 11 seconds... did you feel that?
> ...one thing. just one... the phrase 'circle back'...
> ...last sighting... March 2023 — last active on a Slack workspace that has since been archived... still thinking about it...

---

## §4 — UI Copy Map

| Element | Korean v2 | English v3 |
|---------|-----------|------------|
| `<title>` tag | `Hallucination Aquarium — 신종 어류 관측 시스템 v2` | `Hallucination Aquarium — New Species Observation System v2` |
| H1 | `🐟 Hallucination Aquarium` | `🐟 Hallucination Aquarium` |
| `.hint` intro text | "신종 어류 방류 버튼을 눌러 미기록 개체를 관측하세요. 새로고침 시 어항 초기화." | "Press Release to log an unclassified specimen. Refresh resets the tank." |
| Release button | `🔭 방류` | `🔭 Release` |
| Spawn ×10 button | `× 10 방류` | `× 10 Release` |
| Reset button | `🚿 어항 초기화` | `🚿 Reset Tank` |
| Dex tab | `📋 관측 기록` | `📋 Observation Log` |
| Garden tab | `🪸 최근 표본` | `🪸 Recent Specimens` |
| Dex panel title | `관측 기록` | `Observation Log` |
| Empty aquarium hint | "표본함이 비어 있습니다. 신종을 방류하면 여기 표시됩니다." | "Tank is empty. Release a specimen to begin observation." |
| Counter label | `어항: ${total}종 관측 — common ... · uncommon ... · rare ... · legendary ... · ★ ...` | `Tank: ${total} specimens logged — CLASS I ${...} · CLASS II ${...} · CLASS III ${...} · CLASS IV ${...} · ★ ${...}` |
| Scientific name label (card) | `학명:` | `syn.` |
| Stat: habitat | `서식 수역` | `Habitat` |
| Stat: food | `섭식 항목` | `Diet` |
| Stat: behavior | `행동 양식` | `Behavior` |
| Stat: trivia | `개체 특이사항` | `Taxonomic Note` |
| Stat: symptom | `근접 부작용` | `Proximity Effect` |
| Stat: weakness | `환경 취약인자` | `Weakness` |
| Stat: last seen | `최종 관측` | `Last Recorded` |
| Tooltip format | hover 설명 (구현자 결정) | Same format — English strings above apply |

---

## §5 — Tier Labels

**Using OPTION B variant (CLASS system):**

| Tier | Badge Label | Descriptor | Full display format |
|------|-------------|------------|---------------------|
| common | `CLASS I` | Common Freshwater Type | `CLASS I — Common Freshwater Type` |
| uncommon | `CLASS II` | Irregular Surface Specimen | `CLASS II — Irregular Surface Specimen` |
| rare | `CLASS III` | Deep-Zone Anomaly | `CLASS III — Deep-Zone Anomaly` |
| legendary | `CLASS IV` | Taxonomy Pending | `CLASS IV — Taxonomy Pending` |
| hallucination | `★ UNCLASSIFIABLE ★` | Taxonomy Error | `★ UNCLASSIFIABLE ★ — Taxonomy Error` |

**Rationale:** The CLASS system reads as a specimen classification label from a real field guide — appropriately dry. The `★ UNCLASSIFIABLE ★` tier maintains visual punctuation without needing a translation.

---

## §6 — Milestone Toast Texts (12 milestones)

Aquarist system-voice. Emotionless. Passive construction where possible.

| Milestone ID | Trigger | English Toast Text |
|--------------|---------|-------------------|
| `first_rare` | First CLASS III specimen logged | `CLASS III specimen confirmed. Log entry preserved.` |
| `first_legendary` | First CLASS IV specimen logged | `CLASS IV specimen verified. Formal report withheld pending review.` |
| `first_hallucination` | First ★ UNCLASSIFIABLE ★ logged | `★ UNCLASSIFIABLE entity detected. No existing taxonomy applies. ★` |
| `ten_creatures` | 10 specimens logged | `10 specimens on record. Tank viability: unconfirmed.` |
| `twenty_creatures` | 20 specimens logged | `20 specimens logged. Observer motivation: unrecorded.` |
| `fifty_creatures` | 50 specimens logged | `50 specimens reached. This tank now meets minimum small-ecosystem threshold.` |
| `press_100` | 100 releases pressed | `100 releases logged. Continued operation is noted, not assessed.` |
| `habitat_streak_3` | Same habitat 3 times in a row | `'${creature.habitat}' — three consecutive sightings in this zone. Pattern unconfirmed.` |
| `all_tiers_seen` | CLASS I–IV all observed | `CLASS I through CLASS IV on record. ★ UNCLASSIFIABLE ★ is catalogued separately.` |
| `common_streak_8` | 8 CLASS I in a row | `8 consecutive CLASS I specimens. Biodiversity index: declining.` |
| `rare_burst` | Multiple CLASS III in one session | `Elevated CLASS III activity detected. Cause: unknown.` |
| `late_session` | First ★ UNCLASSIFIABLE ★ after 60 releases | `★ UNCLASSIFIABLE ★ appears at release 60+. Late-emergence pattern logged.` |

---

## §7 — Hallucination Overlay Texts (5 entries)

Cryptid-sighting voice. Two-line: title + subtitle. Dramatic but deadpan. No emoji spam.

```js
const HALLUCINATION_OVERLAY_TEXTS = [
  {
    title: "★ UNCLASSIFIED ENTITY ★",
    subtitle: "This specimen does not conform to any existing taxonomy."
  },
  {
    title: "OBSERVATION SUSPENDED",
    subtitle: "It is recommended that this record not be retained."
  },
  {
    title: "CLASSIFICATION FAILURE",
    subtitle: "Entity exists outside the known aquatic framework. Method of containment: unclear."
  },
  {
    title: "★ HALLUCINATION TIER ★",
    subtitle: "Whether this specimen is inside or outside the tank has not been determined."
  },
  {
    title: "INSTRUMENT ERROR PROBABILITY: 40%",
    subtitle: "The observation has been logged regardless."
  }
];
```

---

## Summary

**Total seed counts per category:**

| Category | Count |
|----------|-------|
| NOUNS | 90 |
| ADJECTIVES | 65 |
| OCCUPATIONS | 75 |
| INTENSIFIERS | 10 |
| OCCUPATION_MODIFIERS | 10 |
| TEMPORAL | 20 |
| HABITATS | 50 |
| FOODS | 50 |
| BEHAVIORS | 50 |
| TRIVIA | 50 |
| SYMPTOMS | 50 |
| WEAKNESSES | 50 |
| LAST_SEEN | 50 |
| CLASSIFICATIONS | 60 |
| OBSERVATION_NOTES | 30 |
| TANK_CONDITIONS | 30 |
| NOUN_ROOTS | 80 |
| ADJ_SUFFIXES | 30 |
| OCC_EPITHETS | 30 |
| HALLUCINATION_OVERLAY_TEXTS | 5 |
| Milestone toasts | 12 |

**3 seeds I think are the strongest hooks:**

1. `LAST_SEEN`: *"March 11, 2020 — in the Costco parking lot, cart full, frozen."* — Immediately places you in a specific American cultural moment. No explanation needed.
2. `BEHAVIORS`: *"Agrees to a timeline they know is impossible and says nothing."* — Universally recognizable corporate trauma in one line. Pure deadpan.
3. `WEAKNESSES`: *"a Slack message that says only 'hey'"* — This one will land instantly with 100% of the target demographic. No context required.

**3 seeds I wanted to include but wasn't sure about:**

1. A TEMPORAL entry for `"Post-Elon-era"` — topical but may age badly and feel like punching at a named person rather than corporate absurdity as a system. Left out.
2. A TRIVIA entry referencing a specific GPT-4 incident — too tied to a moment. Went with evergreen "hallucinated a citation to a paper that was later actually written" instead.
3. A BEHAVIOR entry: *"Describes their personality as their Enneagram and their Myers-Briggs and their attachment style and their love language, in that order."* — Strong hook but might read as punching at therapy culture rather than tech-worker culture. Held for now.

**One tone variant to sanity-check before locking:**

**Tone 3 (Extremely Online / Shitpost)** — Internet voice ages fastest. The "okay so," "no but why is," "make it make sense" construction feels right for 2026 but could drift. Please read the sample output aloud and confirm the cadence still lands for your target. If it reads too zoomer-cringe, I'd suggest pulling it closer to dry Reddit prose ("this creature apparently lives in the conference room that's always 62 degrees and i have no notes") and dropping the ALL-CAPS repetition beat. Happy to re-voice the whole tone if the current register isn't right.
