# crate 模块蓝图与依赖方向

## 1. 推荐模块结构

```text
src/
  lib.rs
  core/
    mod.rs
    session/
    state/
    capability/
  transport/
    mod.rs
    ws/
    sse/
    poll/
    registry/
  protocol/
    mod.rs
    handshake/
    frame/
    codec/
  api/
    mod.rs
    builder/
    client/
    server/
  extension/
    mod.rs
    auth/
    middleware/
    store/
    observability/
  error/
    mod.rs
```

## 2. 依赖方向（单向）

- `api -> core/protocol/transport/extension/error`
- `core -> protocol/error`
- `transport -> protocol/error`
- `extension -> core/protocol/error`
- `protocol -> error`

禁止：

- `core` 依赖 `api`。
- `core` 依赖具体第三方扩展实现。
- `error` 反向依赖其他业务模块。

## 3. 分层说明

- `core`: session 真相状态、状态迁移、生命周期协调。
- `protocol`: 协商消息、能力选择规则、编解码基础。
- `transport`: 具体传输实现与统一 handle 抽象。
- `extension`: 只放接口与适配层，不放业务实现。
- `api`: 对外 Builder 与入口对象，屏蔽内部复杂性。
