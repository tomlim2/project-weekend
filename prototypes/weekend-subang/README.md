# Weekend Subang (서방)

Touhou-style 탄막 슈터, Bevy 0.18, 키보드 전용. 540×960 portrait.

## Controls (planned, MVP 완료 시)

| 키 | 동작 |
|---|---|
| ↑↓←→ | 이동 |
| Shift (hold) | Focus — 속도 ½ + 히트박스 점 표시 + 샷 spread 좁아짐 |
| X | Bomb / 필살기 — 화면 탄 erase + 1.0s 무적, 시작 3개 |
| R | 게임오버 후 재시작 |
| Esc | 일시정지 |

자동공격은 항상 위 방향 (Z 키 없음).

## 실행

```sh
cd prototypes/weekend-subang
cargo run --release
```

## 진행 / 결정 / 회고

`CONCEPT.md` (이 폴더) — 게임 명세 + 빌드 단계 + 아키텍처 + 결정 락
[허브 devlog](../../../../claude/projects/project-weekend/devlog.md) — 옵시디언 vault, 일자별 일지

## 상태

Stage A in progress.
