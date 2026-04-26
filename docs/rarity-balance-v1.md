---
title: "Rarity Balance v1 — Hallucination Zoo"
tags:
  - hallucination-zoo
  - design
  - balance
date: 2026-04-18
source: claude
status: draft
---

# Rarity Balance v1

Hallucination Zoo의 희귀도·피티 시스템·마이크로 마일스톤 구현 스펙.  
대상 구현자: HTML 프로토타입(generator-v0.html) → 향후 Godot 포팅.

---

## 1. 희귀도 가중치 개정

### 핵심 문제: 기존 60/25/10/4/1의 수학적 진단

기존 Hallucination 1% 기준:
- 첫 등장 50% 확률 도달: **69회 누름** (`ln(0.5) / ln(0.99)`)
- 첫 등장 99% 확률 도달: **459회 누름** (`ln(0.01) / ln(0.99)`)

앰비언트 장르에서 세션 길이는 5~20분. 단순 클릭 기준 **30~80회가 일반 세션**이다.  
1% 희귀도는 "99% 세션에서 못 본다"는 뜻이고, Discovery(발견) 미학은 경험되지 않은 채 세션이 끝난다.  
또한 Legendary(4%) 역시 50% 확률 도달에 17회가 필요해, 초반 30회 구간에서 아무것도 없다는 느낌이 강하다.

### 개정안: 43 / 28 / 18 / 8 / 3

| 티어 | 구 가중치 | 신 가중치 | 비율 변화 | 이유 |
|---|---|---|---|---|
| Common | 60% | 43% | ↓ | Common 연속 등장이 지루함의 원인. 적당히 낮춰 다양성 체감을 높임 |
| Uncommon | 25% | 28% | ↑ | Common 감소분을 일부 흡수. "약간 다른" 생물이 더 자주 보여야 함 |
| Rare | 10% | 18% | ↑ | 30회 세션 내에서 3~5마리 나와야 "오늘 Rare 봤다" 서사가 성립함 |
| Legendary | 4% | 8% | ↑ | 50회 세션에서 1~2회 등장. 장르 레퍼런스인 Animal Crossing 금물고기 수준 |
| Hallucination | 1% | 3% | ↑×3 | 아래 별도 계산 |

합계: 43 + 28 + 18 + 8 + 3 = **100%** ✓

### Hallucination 3% 선택 근거

3% 기준 첫 등장 확률 분포:

| 누름 횟수 | 등장 확률 | 해석 |
|---|---|---|
| 23회 | ~50% | 평균 세션 중반에 처음 등장 |
| 46회 | ~75% | 20분 세션이면 기대값 구간 |
| 76회 | ~90% | 긴 세션에서는 거의 확실 |
| 152회 | ~99% | 다회 세션 플레이어 누적 보장 |

**1%와의 비교:**  
- 1%: 세션 80회 기준 등장 확률 55.4% → 절반이 못 봄  
- 3%: 세션 80회 기준 등장 확률 91.1% → "오늘 나왔다" 서사 거의 보장

레퍼런스 비교:
- Pokémon 반짝이(shiny): 1/4096 → Challenge 미학, 완전히 다른 장르
- Vampire Survivors 비밀 아이템: 조건부 but 30분 이내 달성 가능 → Discovery 미학
- Animal Crossing 황금 물고기: 세션당 0~3회 기대값 → Submission 미학 (그냥 두면 온다)

**Hallucination Zoo의 타겟은 Discovery + Submission이다. 3%가 이 두 미학을 동시에 만족한다.**

### 100회 기준 기대 분포 (개정안)

| 티어 | 기대값 | 최소(~5%) | 최대(~95%) |
|---|---|---|---|
| Common | 43마리 | 34 | 52 |
| Uncommon | 28마리 | 21 | 36 |
| Rare | 18마리 | 13 | 24 |
| Legendary | 8마리 | 4 | 13 |
| Hallucination | 3마리 | 0 | 7 |

100회 세션에서 Hallucination을 **하나도 못 보는 확률: 4.8%** (`0.97^100`).  
기존 1%에서는 37%였다. 이 차이가 앰비언트 장르 리텐션에서 결정적이다.

---

## 2. 피티 시스템 스펙

### 설계 원칙

피티는 "보장"이 아니라 "바닥을 친다"는 느낌이어야 한다.  
강제 스포일러가 아니라 자연 확률에 가중치를 더하는 방식 — Genshin Impact식 "소프트 피티(soft pity)" 채택.

### 티어별 피티 발동 임계값

| 티어 | 소프트 피티 시작 | 하드 피티(100% 보장) | 근거 |
|---|---|---|---|
| Rare | 20회 연속 미등장 | 40회 | 18% 확률, 40회 내 미등장 확률 0.06% |
| Legendary | 35회 연속 미등장 | 70회 | 8% 확률, 70회 내 미등장 확률 0.03% |
| Hallucination | 50회 연속 미등장 | 100회 | 세션 단위 보장. 100회면 "오늘 하루" 플레이 |

"연속 미등장"은 **해당 티어 이상이 나오지 않은 횟수**를 기준으로 한다.  
즉 Legendary 피티 카운터는 Legendary 또는 Hallucination이 나와야 리셋된다.

### 소프트 피티 로직

임계값 도달 후 매 누름마다 기본 확률에 선형 가산.  
Godot 포팅에서도 동일하게 구현 가능한 순수 수학 공식:

```
// 소프트 피티: 임계값 초과 후 추가 확률 계산
function getSoftPityBonus(tier, sinceLastSeen) {
  const thresholds = {
    rare:          { soft: 20, hard: 40, baseRate: 0.18 },
    legendary:     { soft: 35, hard: 70, baseRate: 0.08 },
    hallucination: { soft: 50, hard: 100, baseRate: 0.03 }
  };

  const t = thresholds[tier];
  if (!t || sinceLastSeen < t.soft) return 0;

  // 소프트 피티 구간: 0 → 최대 추가율까지 선형 증가
  // 하드 피티 직전에는 (1 - baseRate) 추가 → 사실상 100% 확률
  const progress = (sinceLastSeen - t.soft) / (t.hard - t.soft); // 0.0 ~ 1.0
  const maxBonus = 1 - t.baseRate;
  return Math.min(progress * maxBonus, maxBonus);
}

// 실제 판정 함수
function rollWithPity(state) {
  const roll = Math.random();
  let accumulated = 0;

  for (const tier of ['hallucination', 'legendary', 'rare', 'uncommon', 'common']) {
    const base = BASE_RATES[tier];
    const bonus = (tier === 'rare' || tier === 'legendary' || tier === 'hallucination')
      ? getSoftPityBonus(tier, state.sinceLastSeen[tier])
      : 0;
    const effective = Math.min(base + bonus, 1.0);

    accumulated += effective;
    if (roll < accumulated) {
      // 이 티어 이하 피티 카운터 리셋
      resetPityCounters(state, tier);
      return tier;
    }
  }
  return 'common'; // fallback
}

// 피티 카운터 리셋: 해당 티어와 그 하위 티어만 리셋
function resetPityCounters(state, tier) {
  const tierOrder = ['common', 'uncommon', 'rare', 'legendary', 'hallucination'];
  const idx = tierOrder.indexOf(tier);
  for (let i = 0; i <= idx; i++) {
    state.sinceLastSeen[tierOrder[i]] = 0;
  }
}
```

### 피티 상태 구조체

Godot 포팅 시 동일 구조 사용:

```javascript
// 세션 시작 시 초기화
const pitySate = {
  sinceLastSeen: {
    common: 0,
    uncommon: 0,
    rare: 0,
    legendary: 0,
    hallucination: 0
  },
  totalPresses: 0
};

// 매 생성마다 미등장 카운터 증가
function incrementPityCounters(state, rolledTier) {
  const tierOrder = ['common', 'uncommon', 'rare', 'legendary', 'hallucination'];
  const rolledIdx = tierOrder.indexOf(rolledTier);
  for (let i = rolledIdx + 1; i < tierOrder.length; i++) {
    state.sinceLastSeen[tierOrder[i]]++;
  }
  state.totalPresses++;
}
```

### 피티 적용 세션 경험 예측

| 세션 길이 | 피티 없을 때 | 피티 있을 때 |
|---|---|---|
| 30회 | Hallucination 등장 확률 59%, Legendary 기대값 2.4 | Hallucination 59% (피티 미발동), Legendary ~3 |
| 60회 | Hallucination 등장 확률 84%, Legendary 기대값 4.8 | Hallucination 91% (소프트 피티 발동), Legendary ~5 |
| 100회 | Hallucination 등장 확률 95%, Legendary 기대값 8 | Hallucination **100%** (하드 피티 보장), Legendary 100% |

피티는 30회 세션에서는 거의 느껴지지 않는다. **설계 의도: 피티는 비가시적 안전망이지 보상 전달 메커니즘이 아니다.** 플레이어가 피티를 "계산"하지 않아야 한다.

---

## 3. 마이크로 마일스톤 시스템

### 목적

80~100회 누름 구간에서 발생하는 "왜 계속 누르지" 이탈을 막는다.  
해결책은 게임플레이를 복잡하게 만드는 것이 아니라, **이미 일어난 사건에 이름을 붙이는 것**이다.  
Animal Crossing의 "K.K.가 왔다"처럼 — 특별한 행동 없이 날짜가 지나면 일어나는 일들.

### 구현 조건

- HTML 내 카운터 변수만으로 구현 (localStorage 불필요)
- Godot 포팅 시 동일 로직을 Dictionary로 이식 가능
- 각 마일스톤은 세션 내 1회만 발동 (cooldown = 세션)

### 마일스톤 목록 (12개)

| # | 내부 ID | 발동 조건 | 토스트 텍스트 | 쿨다운 |
|---|---|---|---|---|
| 1 | `first_rare` | 첫 Rare 등장 | "희귀종 포착 — 조용히 관찰하시오" | 세션 1회 |
| 2 | `first_legendary` | 첫 Legendary 등장 | "전설이 나타났다. 아무도 안 알려줬지만." | 세션 1회 |
| 3 | `first_hallucination` | 첫 Hallucination 등장 | "★ 이건... 뭔가 달라 ★" | 세션 1회 |
| 4 | `ten_creatures` | 누적 10마리 | "10마리 수집. 의미는 없지만 숫자는 늘었다" | 세션 1회 |
| 5 | `twenty_creatures` | 누적 20마리 | "20마리. 계속 누르는 이유는 본인만 안다" | 세션 1회 |
| 6 | `fifty_creatures` | 누적 50마리 | "50마리. 오늘 어딘가 도달했다" | 세션 1회 |
| 7 | `habitat_streak_3` | 같은 서식지 3연속 | "서식지 {habitat} — 집단 서식 확인됨" | 5마리 쿨다운 |
| 8 | `all_tiers_seen` | Common~Legendary 4티어 전부 1회 이상 등장 | "4종 티어 도감 완성. Hallucination은 어디에?" | 세션 1회 |
| 9 | `common_streak_8` | Common 8연속 | "Common이 8마리 연속... 왜 이러는 걸까" | 10마리 쿨다운 |
| 10 | `rare_burst` | 5마리 내에 Rare 2마리 이상 | "Rare 연속 등장. 오늘 운 좋은 날이다" | 10마리 쿨다운 |
| 11 | `press_100` | 총 100회 누름 | "100번 눌렀다. 대단하다고 말해야 할까?" | 세션 1회 |
| 12 | `late_session` | 60회 이후 첫 Hallucination | "늦게 나타난 별종. 기다린 보람이 있다" | 세션 1회 |

### 토스트 구현 스펙

```javascript
// 마일스톤 상태 (세션 내 유지)
const milestones = {
  first_rare: false,
  first_legendary: false,
  first_hallucination: false,
  ten_creatures: false,
  twenty_creatures: false,
  fifty_creatures: false,
  habitat_streak_3: false,   // 쿨다운 카운터 별도
  all_tiers_seen: false,
  common_streak_8: false,    // 쿨다운 카운터 별도
  rare_burst: false,         // 쿨다운 카운터 별도
  press_100: false,
  late_session: false
};

// 쿨다운이 있는 마일스톤용 카운터
const milestoneCooldowns = {
  habitat_streak_3: 0,  // 0이면 발동 가능
  common_streak_8: 0,
  rare_burst: 0
};

function checkMilestones(state, newCreature) {
  const fired = [];

  // first_rare / first_legendary / first_hallucination
  if (!milestones.first_rare && newCreature.rarity === 'rare') {
    milestones.first_rare = true;
    fired.push("희귀종 포착 — 조용히 관찰하시오");
  }
  // ... (각 티어 동일 패턴)

  // 카운트 기반
  if (!milestones.ten_creatures && state.totalPresses >= 10) {
    milestones.ten_creatures = true;
    fired.push("10마리 수집. 의미는 없지만 숫자는 늘었다");
  }

  // 서식지 연속 (habitat_streak_3)
  if (milestoneCooldowns.habitat_streak_3 === 0 && state.habitatStreak >= 3) {
    milestoneCooldowns.habitat_streak_3 = 5; // 5마리 쿨다운
    fired.push(`서식지 ${newCreature.habitat} — 집단 서식 확인됨`);
  }

  // 쿨다운 감소
  Object.keys(milestoneCooldowns).forEach(k => {
    if (milestoneCooldowns[k] > 0) milestoneCooldowns[k]--;
  });

  return fired; // 이 배열의 각 항목을 showToast()에 전달
}

// 토스트 표시 (CSS 애니메이션 연동)
function showToast(message) {
  const toast = document.createElement('div');
  toast.className = 'milestone-toast';
  toast.textContent = message;
  document.body.appendChild(toast);
  setTimeout(() => toast.remove(), 3000);
}
```

토스트 CSS (새 스타일 블록에 추가):

```css
.milestone-toast {
  position: fixed;
  bottom: 24px;
  left: 50%;
  transform: translateX(-50%);
  background: #2a2520;
  color: #faf7f2;
  padding: 10px 20px;
  border-radius: 20px;
  font-size: 13px;
  opacity: 0;
  animation: toastIn 0.3s ease forwards, toastOut 0.3s ease 2.7s forwards;
  pointer-events: none;
  z-index: 100;
  white-space: nowrap;
}
@keyframes toastIn {
  from { opacity: 0; transform: translateX(-50%) translateY(8px); }
  to   { opacity: 1; transform: translateX(-50%) translateY(0); }
}
@keyframes toastOut {
  from { opacity: 1; }
  to   { opacity: 0; transform: translateX(-50%) translateY(-4px); }
}
```

---

## 4. Collection UX 신호 수집 — "저장소 vs 생태계" 질문

### 문제 정의

현재 UI는 생성한 생물을 목록(list)으로 쌓는다.  
플레이어가 원하는 것이 두 가지 가설 중 하나다:

- **저장소(Archive) 가설:** "내가 생성한 모든 생물을 리스트로 보고 싶다"  
  → 넘버링된 도감, 중복 포함, 순서대로 쌓임
- **생태계(Ecosystem) 가설:** "내 생물들이 살아있는 공간에서 떠다니길 원한다"  
  → 최신 5~10마리만 화면에 표시, 정원/수조 느낌

이 두 가지는 UI 아키텍처와 Godot 씬 구조에 영향을 준다. v1에서 신호를 수집해야 한다.

### v1 테스트 구현

**탭 2개 추가 — 최소 변경:**

```html
<!-- 기존 버튼 아래에 추가 -->
<div class="view-tabs">
  <button class="tab active" data-view="archive" onclick="switchView('archive')">📋 도감</button>
  <button class="tab" data-view="garden" onclick="switchView('garden')">🌿 정원</button>
</div>
<div id="archive-view"><!-- 기존 list --></div>
<div id="garden-view" style="display:none"><!-- 최근 8마리 떠다니는 뷰 --></div>
```

정원 뷰는 v1에서 복잡한 애니메이션 없이도 OK — 카드 8개를 CSS `flex-wrap`으로 배치해도 된다.  
핵심은 **플레이어가 어느 탭을 먼저 클릭하는지, 어느 탭에서 더 오래 머무는지**다.

**측정할 이벤트 (console.log로 충분):**

```javascript
let activeView = 'archive';
const viewTime = { archive: 0, garden: 0 };
let viewSwitchTime = Date.now();

function switchView(view) {
  // 이전 뷰 체류 시간 누적
  viewTime[activeView] += Date.now() - viewSwitchTime;
  viewSwitchTime = Date.now();
  activeView = view;

  document.getElementById('archive-view').style.display = view === 'archive' ? '' : 'none';
  document.getElementById('garden-view').style.display = view === 'garden' ? '' : 'none';

  // 신호 수집
  console.log(`[UX] 탭 전환: ${view} (archive: ${Math.round(viewTime.archive/1000)}s, garden: ${Math.round(viewTime.garden/1000)}s)`);
}
```

**판정 기준:**
- 첫 탭 클릭이 "정원"인 플레이어가 40% 이상 → 생태계 가설 채택
- archive 체류 시간 > garden × 2 → 저장소 가설 채택
- 탭을 전혀 안 클릭 → 현재 list-only가 충분하다는 신호

---

## 5. 비주얼 FX 티어 스펙

### 설계 원칙

Common = 거의 무음, 거의 무반응. Hallucination = 명백히 다르다.  
Chrome CSS만으로 구현. WebGL 없음.

### 티어별 스펙

#### Common
- **스폰 애니메이션:** `creatureFadeIn` 0.2s ease
- **테두리:** 기존 `1px solid #e0d8cc`
- **배경:** `#fff`
- **글로우:** 없음
- **오디오(미래):** 없음. 완전한 침묵이 Common의 정체성

```css
@keyframes creatureFadeIn {
  from { opacity: 0; transform: translateY(4px); }
  to   { opacity: 1; transform: translateY(0); }
}
.creature { animation: creatureFadeIn 0.2s ease; }
```

#### Uncommon
- **스폰 애니메이션:** `creatureFadeIn` 0.25s ease (Common과 동일하나 약간 느림)
- **테두리:** `1px solid #a0c8a0`
- **배경:** `#f4fbf4`
- **글로우:** 없음
- **오디오(미래):** 낮은 단음 "틱" 0.3s

```css
.creature.uncommon { 
  border-color: #a0c8a0; 
  background: #f4fbf4; 
}
```

#### Rare
- **스폰 애니메이션:** `rareSlideIn` 0.35s ease-out
- **테두리:** `2px solid #6090d0`
- **배경:** `#f0f4ff`
- **글로우:** `box-shadow: 0 0 10px rgba(96,144,208,0.3)`
- **오디오(미래):** 밝은 "딩" 두 음 0.5s

```css
@keyframes rareSlideIn {
  from { opacity: 0; transform: translateY(8px) scale(0.97); }
  to   { opacity: 1; transform: translateY(0) scale(1); }
}
.creature.rare {
  border: 2px solid #6090d0;
  background: #f0f4ff;
  box-shadow: 0 0 10px rgba(96,144,208,0.3);
  animation: rareSlideIn 0.35s ease-out;
}
```

#### Legendary
- **스폰 애니메이션:** `legendaryPop` 0.45s cubic-bezier(0.34,1.56,0.64,1) (오버슈트 바운스)
- **테두리:** `2px solid #d0900a`
- **배경:** `linear-gradient(135deg, #fff9ee 0%, #fff3d0 100%)`
- **글로우:** `box-shadow: 0 0 16px rgba(208,144,10,0.4)`, 미세한 pulsing 추가
- **오디오(미래):** 짧은 팡파레 1.2s

```css
@keyframes legendaryPop {
  from { opacity: 0; transform: scale(0.85); }
  60%  { transform: scale(1.04); }
  to   { opacity: 1; transform: scale(1); }
}
@keyframes legendaryPulse {
  0%, 100% { box-shadow: 0 0 16px rgba(208,144,10,0.4); }
  50%       { box-shadow: 0 0 22px rgba(208,144,10,0.6); }
}
.creature.legendary {
  border: 2px solid #d0900a;
  background: linear-gradient(135deg, #fff9ee 0%, #fff3d0 100%);
  animation: legendaryPop 0.45s cubic-bezier(0.34,1.56,0.64,1),
             legendaryPulse 2s ease-in-out 0.45s infinite;
}
```

#### Hallucination
Common과 Hallucination 사이의 간격이 가장 중요하다. 화면이 "다른 규칙"을 잠깐 따르는 느낌.

- **스폰 애니메이션:** `hallucinationBloom` 0.6s ease-out, 이후 shimmer 무한 반복
- **테두리:** rainbow gradient border (CSS `border-image` 또는 pseudo-element trick)
- **배경:** `linear-gradient(135deg, #fff0f8 0%, #f0fff4 50%, #f8f0ff 100%)`
- **글로우:** 무지개 glow, `box-shadow` 다중 레이어
- **오디오(미래):** 독특한 소리. 기존 UI 소리와 완전히 다른 질감. 0.5~1s. 예시: 유리가 부서지는 역재생 또는 화음 3개가 빠르게 겹치는 코드.

```css
@keyframes hallucinationBloom {
  0%   { opacity: 0; transform: scale(0.7) rotate(-2deg); filter: blur(4px); }
  60%  { transform: scale(1.03) rotate(1deg); filter: blur(0); }
  100% { opacity: 1; transform: scale(1) rotate(0deg); }
}
@keyframes hallucinationShimmer {
  0%   { background-position: 0% 50%; }
  50%  { background-position: 100% 50%; }
  100% { background-position: 0% 50%; }
}
.creature.hallucination {
  border: 2px solid transparent;
  /* rainbow border via background-clip trick */
  background-image: 
    linear-gradient(#fff0f8, #f0fff4, #f8f0ff),
    linear-gradient(135deg, #ff8ac0, #ffcc5a, #8affcc, #8ac0ff, #ff8ac0);
  background-origin: border-box;
  background-clip: padding-box, border-box;
  background-size: 200% 200%;
  box-shadow: 
    0 0 20px rgba(255,138,192,0.35),
    0 0 40px rgba(138,255,204,0.2);
  animation: 
    hallucinationBloom 0.6s ease-out,
    hallucinationShimmer 3s ease-in-out 0.6s infinite;
}
```

### 티어별 FX 강도 요약

| 티어 | 애니메이션 | 지속 효과 | 음향(미래) | 존재감 |
|---|---|---|---|---|
| Common | fade 0.2s | 없음 | 없음 | 배경 |
| Uncommon | fade 0.25s | 없음 | 짧은 틱 | 약간 다른 것 |
| Rare | slide 0.35s | 없음 | 밝은 딩 | "오 이거" |
| Legendary | bounce 0.45s | pulse 무한 | 팡파레 | "와" |
| Hallucination | bloom 0.6s + spin | shimmer 무한 | 완전히 다른 소리 | "이건 뭔가 달라" |

---

## 구현 우선순위 — 3가지 핵심 변경

v1에서 가장 임팩트 큰 변경 순서:

1. **희귀도 가중치를 43/28/18/8/3으로 교체** (코드 1줄, `RARITIES` 배열 수정). Hallucination이 3배 자주 나오면서 Discovery 미학이 즉시 살아난다. 다른 무엇보다 먼저.

2. **Hallucination 전용 bloom FX 추가** (CSS 15줄 + 클래스 조건부 적용). 희귀도 상향 없이 FX만 있으면 공허하고, FX 없이 희귀도만 올리면 "왜 특별한가"가 안 보인다. 이 둘은 함께 구현한다.

3. **마일스톤 토스트 4개 먼저 구현** (`first_hallucination`, `ten_creatures`, `twenty_creatures`, `all_tiers_seen`). 피티와 전체 12개 마일스톤은 이후 단계다. 이 4개만 있어도 80회 이탈 구간이 크게 줄어든다.