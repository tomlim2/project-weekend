---
title: "Aquarium Reframe v2 — Hallucination Zoo → Hallucination Aquarium"
tags:
  - studio-weekend
  - spec
  - reframe
date: 2026-04-18
source: claude
status: draft
---

# Aquarium Reframe v2 — Hallucination Zoo → Hallucination Aquarium

v1의 "환각 생물 동물원" 프레임을 "신종 어류 연구 어항"으로 전환하는 구현자용 명세.
**시드 데이터는 그대로 유지. 바뀌는 것은 프레임(액자)뿐이다.**

핵심 톤: 진지한 어류학자가 토스터를 어류로 기록하는 상황에서 오는 괴리감. 웃음은 만들지 않는다. 기록은 건조하게.

---

## 1. Product Copy 변경표

v1의 모든 유저 대면 문자열을 어항·어류학 어휘로 교체.

### 1-1. 전역 문자열

| 위치 | v1 원문 | v2 교체안 | 비고 |
|------|---------|----------|------|
| `<title>` 태그 | `Hallucination Zoo — Generator v1` | `Hallucination Aquarium — 신종 어류 관측 시스템 v2` | |
| H1 제목 | `🧬 Hallucination Zoo` | `🐟 Hallucination Aquarium` | 이모지: 🐟 또는 🔬 (구현자 결정) |
| .hint 소개문 | "pen 버튼을 눌러 환각 생물 생성. 새로고침하면 리셋. (v1: 4가지 목소리, 희귀도 개편, 도감/정원 탭 실험)" | "신종 어류 방류 버튼을 눌러 미기록 개체를 관측하세요. 새로고침 시 어항 초기화. (v2: 4종 관측 보고서, 희귀도 체계, 어항/표본 탭)" | |

### 1-2. 버튼

| v1 레이블 | v2 레이블 | JS id (변경 없음) |
|----------|----------|-----------------|
| `🖋️ 생성` | `🔭 방류` | `genBtn` |
| `× 10` | `× 10 방류` | `gen10Btn` |
| `🗑️ 리셋` | `🚿 어항 초기화` | `clearBtn` |

### 1-3. 탭 레이블

| 탭 | v1 | v2 | data-view (변경 없음) |
|----|----|----|----------------------|
| 도감 탭 | `📋 도감` | `📋 관측 기록` | `archive` |
| 정원 탭 | `🌿 정원` | `🪸 최근 표본` | `garden` |

### 1-4. 뷰 플레이스홀더

| 위치 | v1 | v2 |
|------|----|----|
| garden-empty | "정원이 비어 있습니다. 생물을 생성해 보세요." | "표본함이 비어 있습니다. 신종을 방류하면 여기 표시됩니다." |

### 1-5. 카운터 레이블

| v1 | v2 |
|----|----|
| `도감: ${total}마리 — common ${...} · uncommon ${...} · rare ${...} · legendary ${...} · ★ ${...}` | `어항: ${total}종 관측 — common ${...} · uncommon ${...} · rare ${...} · legendary ${...} · ★ ${...}` |

"마리" → "종". "도감:" → "어항:".

### 1-6. 티어 레이블 (RARITIES[].label)

v1 티어 이름 유지 권장 (영어 약어가 어류학 분류표 느낌을 오히려 강화). 단 표기 앞에 분류 접두어 추가 가능.

| tier | v1 label | v2 label (권장) | 비고 |
|------|----------|----------------|------|
| common | `COMMON` | `COMMON` | 유지 |
| uncommon | `UNCOMMON` | `UNCOMMON` | 유지 |
| rare | `RARE` | `RARE` | 유지 |
| legendary | `LEGENDARY` | `LEGENDARY` | 유지 |
| hallucination | `★ HALLUCINATION ★` | `★ HALLUCINATION ★` | 유지 — 어류학 분류 불가 판정 느낌 |

> 대안: `CLASS I / CLASS II / CLASS III / CLASS IV / ★ UNCLASSIFIABLE ★` 로 변경하면 더 차갑지만 인식 장벽이 높아짐. 결정은 유저에게 위임.

### 1-7. 마일스톤 토스트 텍스트 (12개)

v1 코드의 `fired.push(...)` 텍스트를 아래 표로 교체. 어조는 "연구소 알림 시스템" — 감정 없음, 수동태.

| milestone key | v1 텍스트 | v2 텍스트 |
|---------------|----------|----------|
| `first_rare` | "희귀종 포착 — 조용히 관찰하시오" | "희귀 개체 최초 관측 완료. 표본 등록 대기 중." |
| `first_legendary` | "전설이 나타났다. 아무도 안 알려줬지만." | "전설급 개체 확인됨. 학계 보고는 보류됩니다." |
| `first_hallucination` | "★ 이건... 뭔가 달라 ★" | "★ 분류 불가 개체 감지됨. 기존 어류 체계로 설명되지 않습니다. ★" |
| `ten_creatures` | "10마리 수집. 의미는 없지만 숫자는 늘었다" | "10종 관측 기록. 어항 유의미성 미확인." |
| `twenty_creatures` | "20마리. 계속 누르는 이유는 본인만 안다" | "20종 기록 완료. 관측자 동기는 파악되지 않음." |
| `fifty_creatures` | "50마리. 오늘 어딘가 도달했다" | "50종 달성. 본 어항은 소형 생태계 기준을 충족합니다." |
| `press_100` | "100번 눌렀다. 대단하다고 말해야 할까?" | "방류 횟수 100회 돌파. 연구 지속 여부 검토 권장." |
| `habitat_streak_3` | "서식지 '${creature.habitat}' — 집단 서식 확인됨" | "'${creature.habitat}' 수역 — 동일 생태 구역 3회 연속 관측됨." |
| `all_tiers_seen` | "4종 티어 도감 완성. Hallucination은 어디에?" | "COMMON~LEGENDARY 전 등급 관측 완료. HALLUCINATION 등급은 별도 분류 체계에 있습니다." |
| `common_streak_8` | "Common이 8마리 연속... 왜 이러는 걸까" | "COMMON 등급 8종 연속 출현. 생물 다양성 지수 하락 중." |
| `rare_burst` | "Rare 연속 등장. 오늘 운 좋은 날이다" | "RARE 등급 집중 출현 감지. 원인 불명." |
| `late_session` | "늦게 나타난 별종. 기다린 보람이 있다" | "방류 60회 이후 HALLUCINATION 출현. 지연 출현 패턴 기록됨." |

---

## 2. 학명 (Latin-style pseudo-binomial) 생성 시스템

### 2-1. 설계 원칙

모든 개체는 **속명(Genus) + 종소명(species epithet)** 2단어 구조의 가짜 이명법 학명을 가진다.

```
속명 = NOUN_ROOT + "us" / "a" / "ia" / "ator"
종소명 = ADJ_SUFFIX + "_" + OCC_EPITHET (선택 조합)
```

- NOUN → 라틴 스타일 어근으로 변환 (아래 매핑 테이블)
- ADJECTIVE → 라틴 접미어 형태로 변환
- OCCUPATION → 종소명 에피텟으로 변환
- 매핑 없는 단어: 한국어 로마자 음역 + `-us`/`-a`/`-um` 어미

**표기 예시:** *Toastophilus melancholicus freelanceris* — 우울한 토스터 프리랜서

---

### 2-2. NOUNS → Genus Root 매핑 (100개)

아래 JSON 키는 `NOUNS` 배열 값, 값은 라틴 어근 (어미 제외, `combine` 함수로 어미 선택).

```json
{
  "참새": "Passerix",
  "비둘기": "Columbix",
  "거북이": "Testudix",
  "도마뱀": "Lacertix",
  "햄스터": "Cricetix",
  "라쿤": "Procyonix",
  "수달": "Lutrax",
  "고양이": "Felinax",
  "돼지": "Porcix",
  "양": "Ovinax",
  "염소": "Caprix",
  "소": "Bovix",
  "코끼리": "Elephantix",
  "미어캣": "Suricatix",
  "올빼미": "Strigix",
  "독수리": "Aquilax",
  "펭귄": "Sphenicix",
  "문어": "Octopix",
  "오징어": "Calamarax",
  "해파리": "Medusix",
  "불가사리": "Asterix",
  "달팽이": "Cochleax",
  "지네": "Scolopix",
  "풍뎅이": "Scarabix",
  "토스터": "Toastophilus",
  "믹서기": "Mixerator",
  "전기주전자": "Kettelax",
  "에어컨": "Frigidax",
  "선풍기": "Ventilax",
  "USB케이블": "Usbilax",
  "멀티탭": "Multiplex",
  "헤드폰": "Auricula",
  "마우스": "Muricula",
  "스팀청소기": "Vaporatix",
  "로봇청소기": "Vacuumix",
  "프린터": "Typographix",
  "에스프레소머신": "Espressix",
  "밥솥": "Orizator",
  "전자레인지": "Microundix",
  "식기세척기": "Lavoplax",
  "냉장고문": "Frigoportax",
  "주방후드": "Hoodvenix",
  "알람시계": "Alarmatix",
  "바나나": "Mussax",
  "토마토": "Tomatix",
  "브로콜리": "Brocculix",
  "감자": "Solanum",
  "양파": "Cepix",
  "마늘": "Alliatix",
  "두부": "Tobulax",
  "계란": "Ovulax",
  "누룽지": "Nurungix",
  "순대": "Soonix",
  "떡": "Tteokix",
  "호빵": "Hopanix",
  "감자튀김": "Frittatix",
  "마카롱": "Macaronix",
  "크루아상": "Croissatix",
  "라면면발": "Ramenatix",
  "김밥속재료": "Gimbatix",
  "소금": "Salinax",
  "구름": "Nebulax",
  "돌멩이": "Lapidix",
  "먼지": "Pulverix",
  "그림자": "Umbrax",
  "소음": "Sonatix",
  "적막": "Silentrax",
  "새벽공기": "Aurorix",
  "빗물": "Pluviatix",
  "낙엽": "Foliax",
  "포자": "Sporax",
  "곰팡이": "Fungitix",
  "이끼": "Mussatix",
  "모래알": "Arenatix",
  "눈송이": "Nivix",
  "지진파": "Seismix",
  "엑셀셀": "Cellatix",
  "파워포인트슬라이드": "Sliderix",
  "PDF": "Pdfatix",
  "탭10개": "Tabuplex",
  "Jira티켓": "Jiratix",
  "Slack알림": "Slackix",
  "캘린더알람": "Calendarix",
  "크롬확장": "Extensix",
  "북마크": "Favoritix",
  "휴지통아이콘": "Trashix",
  "로딩스피너": "Spinatix"
}
```

> 어미 선택 규칙: 어근 끝 자음 → `-us`. 어근 끝 `-a` → `-a` 유지. `-ix`로 끝나면 `-ix` 그대로.  
> 구현 예: `Toastophilus`, `Columbix`, `Solanum`

---

### 2-3. ADJECTIVES → 종소명 접미어 (30개)

| 한국어 형용사 (대표) | Latin suffix 어근 | 완성 종소명 예 |
|-------------------|-----------------|-------------|
| 우울한 | melancholicus | melancholicus |
| 흥분한 | excitaticus | excitaticus |
| 불안한 | anxieticus | anxieticus |
| 지친 | fatigatus | fatigatus |
| 권태로운 | tediomicus | tediomicus |
| 분노한 | iraticus | iraticus |
| 평온한 | placidicus | placidicus |
| 들뜬 | elevaticus | elevaticus |
| 시무룩한 | tristicus | tristicus |
| 허무한 | vanidicus | vanidicus |
| 내향적인 | introverticus | introverticus |
| 외향적인 | extroverticus | extroverticus |
| 완벽주의인 | perfectionicus | perfectionicus |
| 산만한 | distracticus | distracticus |
| 젖은 | humidicus | humidicus |
| 말라비틀어진 | aridicus | aridicus |
| 뜨거운 | calidicus | calidicus |
| 차가운 | frigidicus | frigidicus |
| 끈적이는 | viscidicus | viscidicus |
| 바삭한 | crispidicus | crispidicus |
| 비건인 | veganicus | veganicus |
| 미니멀한 | minimicus | minimicus |
| 힙스터인 | hipsteraticus | hipsteraticus |
| 레트로한 | retroicus | retroicus |
| 은둔자인 | heremiticus | heremiticus |
| 할루시네이션 중인 | hallucinaticus | hallucinaticus |
| 파인튜닝된 | finetunicus | finetunicus |
| 디프리케이트된 | deprecaticus | deprecaticus |
| 캐시 히트한 | cachaticus | cachaticus |
| 논리적인 | logicicus | logicicus |

---

### 2-4. OCCUPATIONS → 종소명 에피텟 (30개)

| 한국어 직업 (대표) | species epithet |
|-----------------|----------------|
| 프리랜서 | freelanceris |
| 바리스타 | baristaris |
| 인턴 | internalis |
| PM | pmanagis |
| 팀장 | teamleadis |
| 의사 | medicalis |
| 변호사 | advocatis |
| 교수 | professoris |
| 초등교사 | pedagogis |
| 연구원 | researcheris |
| 스타트업CEO | ceostratis |
| 마케터 | marketaris |
| 카피라이터 | copywritis |
| HR | humanreslis |
| 풀스택개발자 | fullstackis |
| DevOps | devopalis |
| 유튜버 | youtuberis |
| 스트리머 | streameris |
| 팟캐스터 | podcasteris |
| 소믈리에 | sommelieris |
| 플로리스트 | floristis |
| 요가강사 | yogasteris |
| 시인 | poetalis |
| 소설가 | novelistis |
| 대통령 | presidentis |
| 점성술사 | astrologis |
| 타로마스터 | tarotis |
| 연금술사 | alchemistis |
| 프롬프트엔지니어 | prompteris |
| 데이터라벨러 | labelatoris |

---

### 2-5. 매핑 없는 단어 폴백 규칙

매핑 테이블에 없는 NOUN/ADJECTIVE/OCCUPATION 조합에는 아래 음역 규칙 적용:

```
1. 한국어 단어를 Romanization (RR 방식) 으로 변환
   예: "햇살" → "haetsal"
2. 모음으로 끝나면 + "ius" / 자음으로 끝나면 + "us"
   예: "haetsal" → "haetsalus"
3. 길이 > 12자이면 앞 6자만 사용 후 어미
   예: "microundixus" → 유지 (12자 이하)
4. 전체 학명: "{NOUN_ROOT} {ADJ_SUFFIX}_{OCC_EPITHET}"
   언더스코어는 공백으로 렌더링 금지. 이탤릭체 한 덩어리로 표기.
```

---

### 2-6. 구체 예시 (v1 시드 개체 3개)

**예시 1: 우울한 토스터 프리랜서**
- Genus: `Toastophilus` (토스터 → Toastophilus)
- species: `melancholicus freelanceris` (우울한 + 프리랜서)
- 학명: *Toastophilus melancholicus freelanceris*

**예시 2: 논리적인 냉장고문 전직 PM**
- Genus: `Frigoportax` (냉장고문 → Frigoportax)
- species: `logicicus pmanagis` (논리적인 + PM, OCCUPATION_MODIFIER "전직"은 학명에 미반영)
- 학명: *Frigoportax logicicus pmanagis*

**예시 3: 불안한 로딩스피너 자칭 연금술사**
- Genus: `Spinatix` (로딩스피너 → Spinatix)
- species: `anxieticus alchemistis`
- 학명: *Spinatix anxieticus alchemistis*

---

### 2-7. 학명 렌더링 규칙

- 학명은 이탤릭체로 표시 (`<em>`)
- 카드 name 아래, tone-badge 위에 렌더링
- CSS: `font-size: 11px; color: #8a7a6a; font-style: italic; margin-bottom: 2px;`
- 학명 생성 함수 시그니처:

```js
function generateScientificName(noun, adjective, occupation) {
  const genus   = NOUN_ROOTS[noun]     ?? phoneticLatin(noun);
  const adjSfx  = ADJ_SUFFIXES[adjective] ?? phoneticLatin(adjective);
  const occEpt  = OCC_EPITHETS[occupation] ?? phoneticLatin(occupation) + "is";
  return `${genus} ${adjSfx} ${occEpt}`;
}
```

---

## 3. 4종 어조 템플릿 — 어항 리프레임

기존 어조의 **목소리**만 바뀐다. 슬롯 사용 구조는 유지.

`habitat`은 "관측 수역 / 생태 구역" 으로 재해석:
- "재택 부엌" → "담수 생태계 — 재택 부엌형"
- "클라우드 어딘가" → "부유 생태계 — 클라우드 부표형"
- "깃허브 이슈" → "가상 수역 — 코드 저장소형"

단, 이 변환을 JS에 하드코딩할 필요는 없다. 어조 템플릿 안에서 "수역" 앞치마를 붙이는 방식으로 처리.

---

### 3-1. 어조 1: 논문체 → 어류학 보고서체 (魚類學報告)

**특징:** 어류학 조사 논문 형식. "본 개체" → "본 표본". habitat → "관측 수역". food → "섭식 항목". 그 외 무감정.

```js
function descAcademic(name, scientificName, s) {
  let lines = [
    `${name}(학명: ${scientificName}, 이하 '본 표본')는 ${s.habitat} 수역을 주요 서식처로 삼는다.`,
    `섭식 항목은 ${s.food}로 확인되며, ${s.behavior}는 행동 양식이 반복 관측된다.`,
    `알려진 형태학적 특이사항: ${s.trivia}.`
  ];
  if (s.symptom)  lines.push(`근접 관측 시 보고된 생태 교란: ${s.symptom}.`);
  if (s.weakness) lines.push(`확인된 환경 취약인자: ${s.weakness}. 기전 미상.`);
  if (s.lastSeen) lines.push(`최종 관측 좌표 기록: ${s.lastSeen}.`);
  return lines.join("\n");
}
```

**슬롯 표현 규칙:**

| 슬롯 | 표현 래퍼 |
|------|----------|
| habitat | `${habitat} 수역을 주요 서식처로 삼는다` |
| food | `섭식 항목은 ${food}로 확인되며` |
| behavior | `${behavior}는 행동 양식이 반복 관측된다` |
| trivia | `알려진 형태학적 특이사항: ${trivia}` |
| symptom | `근접 관측 시 보고된 생태 교란: ${symptom}` |
| weakness | `확인된 환경 취약인자: ${weakness}. 기전 미상.` |
| lastSeen | `최종 관측 좌표 기록: ${lastSeen}` |

---

### 3-2. 어조 2: 만화체 → 어류 다큐 내레이터체

**특징:** 흥분된 자연다큐 해설자. "오! 놀라운 장면입니다!" 류의 흥분. 어류·바다 어휘 삽입.

```js
function descComic(name, scientificName, s) {
  let lines = [
    `오! ${name}(${scientificName})가 ${s.habitat} 수역에서 포착됐습니다!!`,
    `이 개체의 먹이는 놀랍게도 ${s.food}!! 그리고 무려 ${s.behavior}!!`,
    `신종 특이사항: ${s.trivia}!`
  ];
  if (s.symptom)  lines.push(`주의! 근접 시 ${s.symptom}!! 접근에 유의하십시오!!`);
  if (s.weakness) lines.push(`약점 발견!! 바로... ${s.weakness}!!`);
  if (s.lastSeen) lines.push(`마지막 목격은 ${s.lastSeen}!! 이후 수면 아래로 사라짐!!`);
  return lines.join("\n");
}
```

**슬롯 표현 규칙:**

| 슬롯 | 표현 래퍼 |
|------|----------|
| habitat | `${habitat} 수역에서 포착됐습니다!!` |
| food | `먹이는 놀랍게도 ${food}!!` |
| behavior | `무려 ${behavior}!!` |
| trivia | `신종 특이사항: ${trivia}!` |
| symptom | `근접 시 ${symptom}!! 접근에 유의하십시오!!` |
| weakness | `약점 발견!! 바로... ${weakness}!!` |
| lastSeen | `마지막 목격은 ${lastSeen}!! 이후 수면 아래로 사라짐!!` |

---

### 3-3. 어조 3: 커뮤체 → 수족관 커뮤 목격담체

**특징:** 수족관 동호회 게시판 글투. "이거 어제 목격함", "조심하셈". 반말 + 건조함.

```js
function descCommu(name, scientificName, s) {
  let lines = [
    `${name}(${scientificName}) - ${s.habitat} 수역에 있다고 함.`,
    `${s.food} 먹는다고 ㅋㅋ 그리고 ${s.behavior}.`,
    `특이사항: ${s.trivia}.`
  ];
  if (s.symptom)  lines.push(`근처 가면 ${s.symptom}. 조심해.`);
  if (s.weakness) lines.push(`약점은 ${s.weakness}. 알면 됨.`);
  if (s.lastSeen) lines.push(`마지막 관측: ${s.lastSeen}. 이후 행방불명.`);
  return lines.join("\n");
}
```

**슬롯 표현 규칙:**

| 슬롯 | 표현 래퍼 |
|------|----------|
| habitat | `${habitat} 수역에 있다고 함` |
| food | `${food} 먹는다고 ㅋㅋ` |
| behavior | `그리고 ${behavior}` |
| trivia | `특이사항: ${trivia}` |
| symptom | `근처 가면 ${symptom}. 조심해.` |
| weakness | `약점은 ${weakness}. 알면 됨.` |
| lastSeen | `마지막 관측: ${lastSeen}. 이후 행방불명.` |

---

### 3-4. 어조 4: ASMR체 → 수족관 야간 관리사체

**특징:** 야간 수족관 혼자 순찰하는 관리사 톤. 낮고 느림. 어항 유리 너머를 보는 시점.

```js
function descAsmr(name, scientificName, s) {
  let lines = [
    `...${name}는... 학명은 ${scientificName}... ${s.habitat} 수역 깊은 곳에... 있어요.`,
    `오늘 섭취한 건... ${s.food}... 그리고... ${s.behavior}...`,
    `...아무도 알아채지 못한 게 하나 있어요... ${s.trivia}.`
  ];
  if (s.symptom)  lines.push(`...가까이 오면... ${s.symptom}... 느껴지시나요?`);
  if (s.weakness) lines.push(`...딱 한 가지 취약점... ${s.weakness}...`);
  if (s.lastSeen) lines.push(`...마지막 관측은... ${s.lastSeen}... 기억하세요?`);
  return lines.join("\n");
}
```

**슬롯 표현 규칙:**

| 슬롯 | 표현 래퍼 |
|------|----------|
| habitat | `${habitat} 수역 깊은 곳에... 있어요` |
| food | `오늘 섭취한 건... ${food}...` |
| behavior | `그리고... ${behavior}...` |
| trivia | `아무도 알아채지 못한 게 하나 있어요... ${trivia}` |
| symptom | `가까이 오면... ${symptom}... 느껴지시나요?` |
| weakness | `딱 한 가지 취약점... ${weakness}...` |
| lastSeen | `마지막 관측은... ${lastSeen}... 기억하세요?` |

---

### 3-5. 어조 가중치 — 변경 없음

v1과 동일. 변경 불필요.

```js
const TONE_WEIGHTS_BY_RARITY = {
  common:        { academic: 35, comic: 30, commu: 25, asmr: 10 },
  uncommon:      { academic: 35, comic: 30, commu: 25, asmr: 10 },
  rare:          { academic: 35, comic: 30, commu: 25, asmr: 10 },
  legendary:     { academic: 50, comic: 30, commu: 20, asmr:  0 },
  hallucination: { academic: 25, comic: 25, commu: 25, asmr: 25 }
};
```

---

## 4. 신규 어항 특화 시드 배열

**주의:** HABITATS, FOODS, BEHAVIORS, TRIVIA, SYMPTOMS, WEAKNESSES, LAST_SEEN, INTENSIFIERS, OCCUPATION_MODIFIERS, TEMPORAL, NOUNS, ADJECTIVES, OCCUPATIONS는 seed-words-v1.md 그대로 유지. 아래 배열만 추가.

---

### 4-1. CLASSIFICATIONS — 수도분류 접두어 (60개)

> 카드 상단 뱃지 또는 학명 위에 소분류 레이블로 사용.
> 형식: "[수계/분류층] [특성어]어" — 실제 어류 분류학 용어 느낌의 조어.

```js
const CLASSIFICATIONS = [
  // 수층별 분류 (12개)
  "심해 사무어",
  "표층 갱생어",
  "중층 대기어",
  "담수 감정어",
  "기수역 경계어",
  "조간대 협업어",
  "열수공 번아웃어",
  "빙하 하층 자책어",
  "부유 생태 관성어",
  "저서 고독어",
  "심저 무기력어",
  "외해 노마드어",

  // 용도/생태계별 분류 (15개)
  "관상용 밈어",
  "식용 불가 불안어",
  "방생 금지 자책어",
  "외래 침입 번아웃어",
  "보호종 지정 직전 어류",
  "양식 실패 재고어",
  "수족관 탈출 경력어",
  "전시 보류 판단 불가어",
  "산란기 미정 미분류어",
  "표본 처리 대기어",
  "냉동 보존 한계어",
  "학명 미등재 어류",
  "기재 논문 철회 후 재관측어",
  "IUCN 등재 실패어",
  "어획량 공식 0 기록어",

  // 서식지 특화 분류 (15개)
  "민물 감정어",
  "도시 하수 적응어",
  "클라우드 부표어",
  "가상 수역 정착어",
  "디지털 습지어",
  "알림 조류 부유어",
  "Wi-Fi 수계 표층어",
  "서버 심층어",
  "캐시 층위 부유어",
  "404 수역 실종어",
  "재택 담수 고립어",
  "공유오피스 조류어",
  "PC방 저서어",
  "고시원 반지하 혐기어",
  "편의점 야광어",

  // 행동 특성 기반 분류 (18개)
  "야행성 자책어",
  "계절성 소멸어",
  "군집 기피 단독어",
  "반복 이동 패턴어",
  "비번식기 고정 개체",
  "자기 위협 표출 경보어",
  "마감 반응형 활성어",
  "칭찬 역반응어",
  "무한 스크롤 공생어",
  "알림 기생어",
  "탭 과다 적재어",
  "로그아웃 반복 어류",
  "3인칭 자기지칭 특이종",
  "수정-삭제 반복행동어",
  "늦은 답장 주기어",
  "빈 캘린더 선호어",
  "야근 유발 어류",
  "보고서 47페이지 목격 어류"
];
```

---

### 4-2. OBSERVATION_NOTES — 관측 기록 접두사 (30개)

> 설명문 맨 앞 또는 카드 푸터에 붙이는 필드 노트 헤더. 어조와 무관하게 동일하게 적용 가능. 사용 확률 권장: 20~30%.

```js
const OBSERVATION_NOTES = [
  "최근 관측 기록:",
  "개체 특이사항:",
  "2026년 4월 보고서:",
  "수족관 담당자 메모:",
  "현장 관측 노트:",
  "표본 채취 기록:",
  "이상 행동 보고:",
  "생태 교란 경보:",
  "관측 횟수: 1회 / 신뢰도: 낮음",
  "야간 순찰 기록:",
  "무인 카메라 포착:",
  "우발 관측 — 비공개 처리 예정:",
  "오류 가능성 있음 / 재확인 요망:",
  "제보 수신 — 검증 중:",
  "3차 관측 결과:",
  "수계 이상 감지 후 추적 기록:",
  "표본 번호 미부여 개체:",
  "학명 검토 전 임시 기록:",
  "신뢰도: 미확인 / 목격자: 1인:",
  "비정기 관측 일지:",
  "익명 제보 기반 기록:",
  "긴급 보고 — 담당자 연락 필요:",
  "미기록 수역 발견 후 등록:",
  "6개월 이상 미관측 개체 재등장:",
  "계절 외 출현 특이 사례:",
  "동일 개체 여부 불명:",
  "표본 손상으로 동정 불완전:",
  "장비 오류 가능 — 참고용:",
  "비전문가 목격 보고 (원문 보존):",
  "분류 보류 — 학명 추후 부여 예정:"
];
```

---

### 4-3. TANK_CONDITIONS — 수조 상태 수식어 (30개)

> 옵션 슬롯. 어조 설명 시 "수조 상태" 또는 마일스톤 서브 텍스트로 활용. 사용 확률 권장: 15%.
> 형식: 수조 환경 묘사 1-2어절.

```js
const TANK_CONDITIONS = [
  "pH 불명 수계",
  "수온 측정 불가",
  "여과 시스템 경보",
  "용존 산소 임계치",
  "염도 불균형 수역",
  "조명 24시간 점등",
  "먹이 공급 단절 7일차",
  "표본 밀도 초과",
  "외래종 혼입 의심",
  "수조 균열 보수 중",
  "펌프 고장 대기",
  "수질 악화 경보",
  "생태계 불균형 진행 중",
  "정전 후 재가동",
  "백화현상 진행 중",
  "부유 입자 과다",
  "번식기 미도래 추정",
  "동면 유도 조건 설정됨",
  "격리 수조 배정 대기",
  "자연 채광 차단 환경",
  "소음 노출 실험 수조",
  "무산소 층 형성 중",
  "생물량 지수 하강 중",
  "공생 생물 부재 확인",
  "먹이사슬 최하위 배치",
  "야간 순찰 감지 구역",
  "수족관 폐관 전 최후 수조",
  "수리 중 — 임시 표본 보관",
  "온도 급변 기록 수조",
  "학명 미부여 개체 전용 수조"
];
```

---

## 5. 희귀도 이벤트 비주얼 카피

카드 등장 시 오버레이 또는 토스트 영역에 표시하는 짧은 발견 선언문. 어조: 냉정한 시스템 알림 + 어류학 권위.

### 5-1. COMMON

```
신종 관측
```
또는 없음 (무인 처리 가능).

### 5-2. UNCOMMON

```
비정기 출현 개체 기록됨
```

### 5-3. RARE

```
희귀 개체 포착
관측 기록 보존 중
```

```
RARE SPECIMEN DETECTED
관측 확인 요망
```

### 5-4. LEGENDARY

```
전설급 개체
학계 보고 보류

LEGENDARY CLASS
분류 보류 중
```

### 5-5. HALLUCINATION — 크립티드 목격 수준

```
★ UNCLASSIFIED ENTITY ★
기존 어류 분류 체계로 설명되지 않습니다

HALLUCINATION TIER
어항 외부에서 관측된 것으로 추정됩니다
어류가 아닐 수 있습니다
```

추가 HALLUCINATION 대안 문구 (랜덤 선택 가능):

```js
const HALLUCINATION_OVERLAY_TEXTS = [
  "★ UNCLASSIFIED ★\n이것은 어류가 아닐 수 있습니다.",
  "비공개 개체 감지.\n이 기록은 남기지 않는 것을 권장합니다.",
  "분류 실패 — 어류 체계 외 존재.\n표본 처리 방법 불명.",
  "★ HALLUCINATION ★\n어항 내부인지 외부인지 확인 불가.",
  "관측자 장비 이상 가능성 40%.\n그럼에도 기록은 보존됩니다."
];
```

---

## 6. 재생성 금지 항목 확인

아래 배열은 **seed-words-v1.md에서 변경 없이 그대로 참조**. v2에서 복사·수정 금지.

- `HABITATS` — 서식지 시드 50개
- `FOODS` — 섭식 항목 50개
- `BEHAVIORS` — 행동 양식 50개
- `TRIVIA` — 개체 특이사항 50개
- `SYMPTOMS` — 접촉 부작용 50개
- `WEAKNESSES` — 환경 취약 인자 50개
- `LAST_SEEN` — 최종 관측 기록 50개
- `INTENSIFIERS` — 강도 수식어 10개
- `OCCUPATION_MODIFIERS` — 직업 수식어 10개
- `TEMPORAL` — 시간 수식어 20개
- `NOUNS` — 개체 명사 90개+
- `ADJECTIVES` — 형용사 50개+
- `OCCUPATIONS` — 직업 60개+

---

## 요약

**신규 카테고리 수량:**
- NOUN_ROOTS JSON: 90개 (NOUNS 배열 커버리지 100%)
- ADJ_SUFFIXES: 30개
- OCC_EPITHETS: 30개
- CLASSIFICATIONS: 60개
- OBSERVATION_NOTES: 30개
- TANK_CONDITIONS: 30개
- HALLUCINATION_OVERLAY_TEXTS: 5개
- 마일스톤 토스트 텍스트: 12개 (전량 교체)
- 어조 템플릿: 4종 (전량 리프레임, 슬롯 구조 유지)

**모호한 사항:**
- 학명 어미 선택 (`-us`/`-a`/`-ix`) 세부 규칙은 일관성을 위해 구현자가 단일 함수로 정리할 것.
- `TANK_CONDITIONS`는 선택 적용 슬롯으로 남겨 두었으나 어느 어조에 어떻게 삽입할지 미결.

**유저에게 확인 요청할 단 하나의 카피 결정:**
티어 레이블 `COMMON / UNCOMMON / RARE / LEGENDARY / ★ HALLUCINATION ★`를 **그대로 유지**할지, 아니면 `CLASS I / CLASS II / CLASS III / CLASS IV / ★ UNCLASSIFIABLE ★`처럼 어류학 분류 번호 체계로 교체할지. 이 결정이 카드 뱃지 디자인과 토스트 문구 전체에 영향을 줍니다.
