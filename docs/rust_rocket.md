# Rust로 REST api 구축하기

rocket이라는 rust server library를 이용한다.

https://rocket.rs/v0.4/guide/getting-started/

> 주의! rocket를 실행하기 위해선 rust nightly 버전이 필요하다.
> ```sh
> rustup default nightly
> rustup override set nightly
> ```

```sh
cargo new rustweb --bin
cd rustweb
```