---
title: "Description Tones v1 — 설명문 어조 템플릿"
tags:
  - studio-weekend
  - spec
  - reference
date: 2026-04-18
source: claude
status: draft
---

# Description Tones v1 — 설명문 어조 4종

각 생물 생성 시 어조(tone)를 선택해 설명문 문체를 다양화.
4종 어조 × 슬롯 조합 = 묘하게 다른 분위기.

---

## 어조 선택 규칙

| 어조 | 선택 가중치 | 트리거 조건 |
|------|-----------|------------|
| 논문체 | 35% | 기본 |
| 만화체 | 30% | 기본 |
| 커뮤체 | 25% | 기본 |
| ASMR체 | 10% | 기본 |

희귀도 보정:
- `hallucination` 티어 → ASMR체 가중치 2배 (20%), 나머지 균등 재분배
- `legendary` 티어 → 논문체 가중치 2배 (50%), ASMR체 0%

---

## 어조 1: 논문체

**특징:** 건조한 학술 서술. 수동태·3인칭 혼용. 인용 없는 "알려진 바에 따르면". 뜬금 엄숙.

**JS 템플릿 함수:**

```js
function descAcademic(name, slots) {
  const { habitat, food, behavior, trivia, symptom, weakness, lastSeen } = slots;
  let lines = [
    `${name}(이하 '본 개체')는 ${habitat}을 주요 서식지로 삼는다.`,
    `식이 패턴은 ${food}에 의존하며, ${behavior}는 행동으로 보고된다.`,
    `알려진 바에 따르면 ${trivia}.`
  ];
  if (symptom) lines.push(`접촉 시 관찰되는 현상: ${symptom}.`);
  if (weakness) lines.push(`알려진 취약점: ${weakness}. 원인 불명.`);
  if (lastSeen) lines.push(`최종 위치 확인: ${lastSeen}.`);
  return lines.join("\n");
}
```

**출력 예시:**
> 논리적인 냉장고문 전직 PM(이하 '본 개체')는 공유 오피스 구석을 주요 서식지로 삼는다.
> 식이 패턴은 피드백 루프에 의존하며, 자기가 한 말을 5분 뒤에 부정한다는 행동으로 보고된다.
> 알려진 바에 따르면 MBTI가 바뀔 때마다 이름도 바뀐다.
> 접촉 시 관찰되는 현상: 근처에 있으면 갑자기 탭이 12개 열린다.

---

## 어조 2: 만화체

**특징:** 감탄사·구어체. 강조 부호 (!!). 중간에 독자 말 걸기. 흥분된 해설자 톤.

**JS 템플릿 함수:**

```js
function descComic(name, slots) {
  const { habitat, food, behavior, trivia, symptom, weakness, lastSeen } = slots;
  let lines = [
    `와!! ${name}가 ${habitat}에서 발견됐다!!`,
    `얘가 먹는 건 다름 아닌 ${food}!! 그리고 무려 ${behavior}!!`,
    `충격 사실: ${trivia}!`
  ];
  if (symptom) lines.push(`주의!! 가까이 가면 ${symptom}!!`);
  if (weakness) lines.push(`약점 발견!! 바로... ${weakness}!!`);
  if (lastSeen) lines.push(`마지막 목격은 충격적이게도 ${lastSeen}!!`);
  return lines.join("\n");
}
```

**출력 예시:**
> 와!! 할루시네이션 중인 브로콜리 자칭 대통령가 상상 속 집무실에서 발견됐다!!
> 얘가 먹는 건 다름 아닌 AI 기사 헤드라인!! 그리고 무려 자기 트윗을 세 번 수정한 뒤 삭제한다!!
> 충격 사실: 본인만 자기가 AI라고 생각한다!

---

## 어조 3: 커뮤체

**특징:** 인터넷 커뮤니티 서술. 반말 혼용. "ㄹㅇ", "개", "진짜로" 없이도 커뮤 느낌. 짧고 끊긴 문장. 냉소적.

**JS 템플릿 함수:**

```js
function descCommu(name, slots) {
  const { habitat, food, behavior, trivia, symptom, weakness, lastSeen } = slots;
  let lines = [
    `${name} - ${habitat} 산다고 함.`,
    `${food} 먹는다 ㅋㅋ 그리고 ${behavior}.`,
    `특이사항: ${trivia}.`
  ];
  if (symptom) lines.push(`주의할 점: ${symptom}. 조심해.`);
  if (weakness) lines.push(`약점은 ${weakness}. 이거 알면 끝임.`);
  if (lastSeen) lines.push(`마지막 목격: ${lastSeen}. 이후 행방불명.`);
  return lines.join("\n");
}
```

**출력 예시:**
> 어쩐지 권태로운 달팽이 은퇴한 소믈리에 - 카페 구석 좌석 산다고 함.
> 링크드인 알림 먹는다 ㅋㅋ 그리고 주말마다 사라진다.
> 특이사항: 공식 사망 선고 이후에도 댓글을 남긴다.
> 마지막 목격: 2026년 1월 31일 오전 3시 17분, 깃허브 이슈 #4722. 이후 행방불명.

---

## 어조 4: ASMR체

**특징:** 낮고 부드럽고 느린 서술. 감각 묘사 강조. 의도적으로 과도한 친밀감. 불쾌한 골짜기 직전. 10% 확률 등장 = 희귀.

**JS 템플릿 함수:**

```js
function descAsmr(name, slots) {
  const { habitat, food, behavior, trivia, symptom, weakness, lastSeen } = slots;
  let lines = [
    `...${name}는... 아주 조용히... ${habitat}에... 있어요.`,
    `오늘 먹은 건... ${food}... 그리고... ${behavior}...`,
    `...아무도 모르는 사실이 하나 있어요... ${trivia}.`
  ];
  if (symptom) lines.push(`...가까이 오면... ${symptom}... 느껴지나요?`);
  if (weakness) lines.push(`...딱 한 가지 약점... ${weakness}...`);
  if (lastSeen) lines.push(`...마지막으로 본 건... ${lastSeen}... 기억해요?`);
  return lines.join("\n");
}
```

**출력 예시:**
> ...진짜 불안한 알람시계는... 아주 조용히... 옥상 물탱크 위에... 있어요.
> 오늘 먹은 건... 새 업데이트 팝업... 그리고... 알림이 울리면 3분간 멈춘다...
> ...아무도 모르는 사실이 하나 있어요... 자기 버전 번호를 2.7이라고 주장한다.

---

## 구현 명세 (JS-ready)

```js
// 어조 등록
const TONES = {
  academic: { weight: 35, fn: descAcademic },
  comic:    { weight: 30, fn: descComic    },
  commu:    { weight: 25, fn: descCommu    },
  asmr:     { weight: 10, fn: descAsmr    }
};

// 희귀도별 가중치 오버라이드
const TONE_WEIGHTS_BY_RARITY = {
  common:        { academic: 35, comic: 30, commu: 25, asmr: 10 },
  uncommon:      { academic: 35, comic: 30, commu: 25, asmr: 10 },
  rare:          { academic: 35, comic: 30, commu: 25, asmr: 10 },
  legendary:     { academic: 50, comic: 30, commu: 20, asmr:  0 },
  hallucination: { academic: 25, comic: 25, commu: 25, asmr: 25 }
};

function pickTone(rarity) {
  const weights = TONE_WEIGHTS_BY_RARITY[rarity] || TONE_WEIGHTS_BY_RARITY.common;
  const total = Object.values(weights).reduce((a, b) => a + b, 0);
  let roll = Math.random() * total;
  for (const [tone, w] of Object.entries(weights)) {
    roll -= w;
    if (roll <= 0) return tone;
  }
  return "academic";
}

// 슬롯 수집
function collectSlots() {
  return {
    habitat:  pick(HABITATS),
    food:     pick(FOODS),
    behavior: pick(BEHAVIORS),
    trivia:   pick(TRIVIA),
    symptom:  Math.random() < 0.40 ? pick(SYMPTOMS)   : null,
    weakness: Math.random() < 0.30 ? pick(WEAKNESSES) : null,
    lastSeen: Math.random() < 0.25 ? pick(LAST_SEEN)  : null
  };
}

// 최종 생성
function generateCreature() {
  const name    = generateName();          // seed-words-v1.md 참고
  const rarity  = pickRarity();
  const tone    = pickTone(rarity.tier);
  const slots   = collectSlots();
  const desc    = TONES[tone].fn(name, slots);
  return { name, desc, rarity, tone };
}
```

---

## 새 슬롯 사용 규칙 (어조별)

| 슬롯 | 논문체 | 만화체 | 커뮤체 | ASMR체 |
|------|-------|-------|-------|-------|
| SYMPTOMS | "접촉 시 관찰되는 현상: ~" | "주의!! ~!!" | "주의할 점: ~. 조심해." | "가까이 오면 ~... 느껴지나요?" |
| WEAKNESSES | "알려진 취약점: ~. 원인 불명." | "약점 발견!! 바로... ~!!" | "약점은 ~. 이거 알면 끝임." | "딱 한 가지 약점... ~..." |
| LAST_SEEN | "최종 위치 확인: ~." | "마지막 목격은 충격적이게도 ~!!" | "마지막 목격: ~. 이후 행방불명." | "마지막으로 본 건... ~... 기억해요?" |

---

## 판단 필요 항목

1. **ASMR체 일본어 말투 유사성** — "...있어요" 반복이 불편하다면 "있습니다" 또는 그냥 반말로 변경 가능. 어느 쪽이 덜 불쾌한 골짜기인지 플레이테스트 후 결정 권장.
2. **커뮤체의 "ㅋㅋ"** — 만화체·커뮤체 경계가 모호할 수 있음. "ㅋㅋ"는 커뮤체에만 격리하거나 제거 고려.
3. **논문체 "이하 '본 개체'" 반복** — 이름이 길어지면 논문체 첫 줄이 너무 길어짐. 이름 앞 수식어(TEMPORAL, INTENSIFIER)가 붙으면 논문체에서는 수식 생략 옵션 고려.
