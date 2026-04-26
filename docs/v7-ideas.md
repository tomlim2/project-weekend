---
title: v7 아이디어 박스 (미구현)
tags:
  - studio-weekend
  - backlog
date: 2026-04-18
source: claude
status: ideas
---

# v7 아이디어 박스

v6 리뷰 후 실제 적용 여부 결정 예정. 지금은 기록만.

---

## 추가 후보 종 (총 +22)

### 🪸 산호 (6종) → `plantedSway` 또는 `stationary`
- Brain Coral
- Zoanthid
- Mushroom Coral
- Xenia
- Torch Coral
- Candy Cane Coral

### 🌿 수초·해초 (6종) → `plantedSway`
- Amazon Sword
- Java Fern
- Anubias
- Red Kelp
- Seagrass
- Water Wisteria

### 🪨 돌·하드스케이프 (4종) → `stationary` (움직임 0)
- Seiryu Stone
- Lava Rock
- Dragon Stone
- Petrified Wood

### 🦀 게 (3종) → `bottomWalker`
- Hermit Crab
- Red Crab
- Fiddler Crab

### 🫧 수면종 (3종) → `float` (신규 타입)
- Hatchetfish
- Gourami
- Killifish

---

## 움직임 타입 확장

| 타입 | 움직임 | 대상 |
|------|--------|------|
| `stationary` | 거의 정지 (±1px) | 돌 |
| `plantedSway` (신규) | 수류 스웨이 ±5~10px | 산호·수초 |
| `float` (신규) | 수면 상단 15%, 느린 수평 + 출렁 | 수면종 |
| `bottomWalker` | 바닥만 좌우 (기존) | 게 |

---

## 메커니즘 변경 후보

- **최대 마리수** 20 → 40~50 (어항 "가득" 느낌)
- **NFT 카드 대상 확장**: 돌·수초도 "specimens"로 간주 → Edition # 유효
- **학명 표 확장**: 새 22종의 genus 매핑 추가 (식물·광물은 유사 학명 규칙 별도)
- **도감 축 추가**: "Hardscape collected / Plants collected" 축 분리

---

## 보류 이유

v6 리뷰 후 실제 움직임 체감 보고 결정. 스웨이 강도, 스폰 밀도, 시각적 균형 등 브라우저로 확인 후 진행.
