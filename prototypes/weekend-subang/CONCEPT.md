# Weekend Subang (서방 / 西方)

Touhou-style 탄막 슈터, Bevy 0.18, 키보드 전용. Endless wave + Bomb 메커닉.
도형 only. 540×960 portrait window. 동방 시리즈 오마주 네이밍 — 동방의 패러디로 西方 (Subang).

## Player Fantasy

작은 점 하나가 화면 가득한 탄막 사이를 정밀 회피하며 살아남는다.
긴장감 있는 회피, 점점 짙어지는 탄, 위기에서 X 한 번으로 화면을 비우는 카타르시스.

## Core Loop

이동 → 자동공격이 위로 뻗어 적을 잡음 →
적 BulletEmitter 가 탄을 쏟아냄 →
정밀 회피 (Focus 시 더 정밀) →
위기 시 Bomb (X) → 화면 탄 erase + 무적 →
잔기 0 까지 반복.

## MVP Scope (must-have)

- 540×960 portrait window, 270×480 internal RT 업스케일 (faux-pixel) — *Stage A2/B'에서 추가*
- 화살표 이동, Shift Focus (속도 ½ + 히트박스 점 시각화), X Bomb
- 자동공격 위 방향 (cooldown), Focus 시 spread 좁아짐
- 적 spawn (top edge) → 직선/곡선 비행 → BulletEmitter 2 패턴 (aimed, ring)
- Player tiny hitbox (~2~3px), HP/잔기 3, IFrame 1.5s
- Bomb 시작 3개, 화면 탄 erase + 1.0s invuln + 시각 ring
- HUD: score, hi-score, 잔기, 봄 잔량, 생존 시간
- SFX 4종: shot / hit / bomb / death
- Best score persist (`std::fs` JSON)

## Stretch (시간 남으면)

- Graze (탄 거의 닿을 때 점수 + sfx)
- 보스 1체 + 스펠카드 1~2 (60~90s)
- Second weapon (laser / homing) 시작 시 선택
- BGM (자체 스케줄링 칩튠)
- WASM 빌드 + touch overlay (D-pad + Focus/Bomb 버튼)

## Out of Scope

- 멀티플레이, 로컬 이외 세이브, 설정 화면, 멀티 캐릭터, 멀티 스테이지,
  도전과제, 스프라이트 에셋, 셰이더 효과, 날씨/배경 패럴랙스

## Tech Stack

- Bevy 0.18 (회사 shotloom + weekend-survivor 매칭)
- Rust edition 2021
- 단일 바이너리 크레이트
- 비주얼: `Mesh2d` 프리미티브 (`Circle`, `Rectangle`, `Triangle2d`)
- 오디오: `bevy_audio` (feature `wav`) + procedural sine WAV (weekend-survivor 식)
- 의존성: `bevy`, `rand`, `serde`, `serde_json`

## Acceptance Criteria

- [ ] 540×960 portrait 윈도우 열리고 화살표 이동 작동
- [ ] Focus (Shift) 시 속도 ½, 히트박스 시각화
- [ ] 자동공격 위로 발사, 적 1샷킬, 점수 누적
- [ ] 적 BulletEmitter 가 화면에 가시 탄막 생성 (≥100 발 동시)
- [ ] Player tiny hitbox 가 탄에 맞으면 IFrame + HP 1 감소, 0 → 게임오버
- [ ] Bomb (X) 화면 탄 모두 erase + invuln + 잔량 1 차감
- [ ] R 재시작, hi-score 재실행 후에도 유지
- [ ] 60 FPS 유지 + 1분 이상 플레이 가능

## Build Stages

| 단계 | 목표 | 검증 | 상태 |
|---|---|---|---|
| **A** | 윈도우 + 카메라 + 플레이어 도형 + 화살표 이동 + Focus 속도 변화 | 화면 안에서 움직이고 Shift 시 절반 속도 | `139588c` |
| **A2** | 270×480 RT + 540×960 nearest 업스케일 (faux-pixel) | 도형이 픽셀화돼 보임 | — |
| **B** | Auto-fire 위 방향 + PlayerBullet + lifetime + 화면 밖 despawn + Focus 시 spread 좁아짐 | 1초당 ~10발, fan 3-shot (normal) / parallel 2-shot (focus) | (in progress) |
| C | 적 spawn (top edge) + 직선 하강 + PlayerBullet↔적 1샷킬 + 점수 | 적이 떨어지며 죽고 점수 ↑ | — |
| D | EnemyBulletEmitter — aimed + ring 2 패턴, 적별 다른 패턴 | 화면에 탄막 ≥100 발, 패턴 구분됨 | — |
| E | Player tiny hitbox(2~3px) + HP/잔기 3 + IFrame 1.5s + GameOver state + R 재시작 | 일부러 맞아서 잔기 깎이고 0 → 게임오버 | — |
| F | Bomb (X) 잔량 3 + 화면 탄 erase + 1.0s invuln + 시각 ring | 탄막 한가운데서 X → 살아남음 | — |
| G | SFX 4종 (shot/hit/bomb/death) + HUD 풀 (score/hi/잔기/봄/시간) + best score JSON persist | 게임 끝나면 best score 갱신, 재실행 시 유지 | — |

## Architecture (planned)

각 단계가 Plugin-per-system 분리 (weekend-survivor 패턴 그대로).
모든 게임플레이 시스템은 `GameplaySet.run_if(in_state(Playing))` 으로 묶음.
재시작은 `OnExit(GameState::GameOver)` 트리거 (RestartGame 메시지 ordering race 회피, weekend-survivor 학습).

| 모듈 | 역할 |
|---|---|
| `main.rs` | App, GameState (Playing / Paused / GameOver), 윈도우, plugin 등록 |
| `player.rs` | Player 컴포넌트 (focused: bool), 화살표 이동, Focus 속도, 히트박스 시각화 |
| `weapon.rs` | Auto-fire 위 방향, PlayerBullet, lifetime 누적기 (`f32`), Focus 시 spread 변화 |
| `enemy.rs` | Enemy 스폰 (top edge, rate ↑), 직선/곡선 비행 |
| `emitter.rs` | `BulletEmitter { pattern: Pattern, rate: f32 }`, EnemyBullet, Aimed/Ring 함수 |
| `combat.rs` | PlayerBullet↔Enemy 충돌, Player tiny hitbox↔EnemyBullet 충돌 |
| `hp.rs` | Lives(3), IFrame(1.5s), GameOver 전이, R 재시작 (`OnExit(GameOver)` reset) |
| `bomb.rs` | BombCount(3), X → 화면 탄 erase + invuln 1.0s, 시각 ring 효과 |
| `hud.rs` | Score / Hi / Lives / Bombs / Time, GameOver 오버레이 |
| `sfx.rs` | Procedural sine WAV (shot/hit/bomb/death), `play_oneshot` helper |
| `upscale.rs` | 270×480 RenderTarget → 540×960 nearest 풀스크린 쿼드 (Stage A2) |
| `persist.rs` | best-score JSON (`~/.local/share/weekend-subang/save.json` 또는 옆 파일) load/save |

## Decisions (lock)

- **Cooldown:** `f32` 누적기 (multiplier 가능, weekend-survivor 학습)
- **Bullet lifetime:** `Lifetime(f32)` 컴포넌트 + 화면 밖 despawn AABB
- **Hitbox 분리:** `VisualSprite` (16px) ≠ `Hitbox(Circle r=2)`. collision 은 `Hitbox` 만
- **Focus 효과:** `Player.focused: bool`, movement + weapon 둘 다 읽음. 발사 spread 좁아짐, 속도 절반, 히트박스 점 시각화 활성
- **Bomb erase:** `Query<Entity, With<EnemyBullet>>` despawn + `Invuln(1.0s)` 컴포넌트
- **BulletEmitter:** 데이터 driven `Pattern { kind: Aimed | Ring { count: u32, speed: f32 }, ... }`. 새 패턴 = enum variant 추가
- **재시작:** `OnExit(GameState::GameOver)` 에서 각 모듈 reset (메시지 패턴 X)
- **물리 엔진 미사용:** 원/사각형 hit detection 단순. `bevy_rapier` 등 무거운 의존성 안 씀

## Numbers (Stage A 시작)

- Window: 540 × 960
- Internal RT (Stage A2부터): 270 × 480
- Player visual: 14×18 rectangle, 색 `#d8f0ff`
- Player hitbox (Stage E부터): radius 2.5, 색 `#ff5577` (Focus 시만 표시)
- Player base speed: 220 px/s
- Player focus speed mult: 0.5
- Play field clamp: window 안쪽 (Stage A 시점, RT 도입 후 RT 좌표계 기준 재정의)

수치 상수는 각 모듈 상단의 `const` 에 박힘. 튜닝은 직접 수정.
