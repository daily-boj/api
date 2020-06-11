# api

가지고 있는 정보를 가공하고 외부 API와 상호작용해,
컴파일 타임에 프론트엔드에서 호출할 수 있는 모든 API 반환을 생성합니다.

## API 목록

- solved.ac
  - 문제 정보
- GitHub
  - 사용자별 푼 문제 목록
- Daily BOJ
  - 멤버
    - 목록
    - 개인
  - 문제
    - 날짜별
    - 난이도별

## 프로젝트 구조

- src/
  - database/
    - JSON 기반의 자체 DB와 상호작용합니다.
  - domain/
    - {table}/entity.rs
      - 데이터 로우를 표현하는 엔티티입니다.
    - {table}/repository.rs
      - 데이터에 CRUD 연산을 합니다.
  - service/
    - 데이터를 가공합니다.
  - action/
    - route를 정의합니다. request를 받아서 적당한 response를 반환합니다.
  - provider/
    - request에 들어갈 수 있는 정보를 제공합니다.
  - resolver/
    - Provider를 조합해 모든 경우의 수로 request를 만들고,
      이를 Action에 보내 response를 생성합니다.
  - cli/
    - 데이터를 조작하기 위한 cli 앱입니다.
