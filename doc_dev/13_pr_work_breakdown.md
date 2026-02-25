# PR 工作内容拆分（执行版）

本文件基于 `12_pr_splitting_plan.md`，把每个 PR 拆成“可直接执行”的工作包。

## PR-01 工程骨架与 feature 声明

### 目标

建立稳定目录结构、模块边界与 feature 开关。

### 文件级工作

- `Cargo.toml`
  - 增加 features：`ws/sse/poll/client/server/tower/tracing/metrics`。
  - 定义 default features（建议最小可运行集合）。
- `src/lib.rs`
  - 暴露顶层模块（仅声明，不放实现逻辑）。
- 新建模块目录及 `mod.rs`：
  - `src/core/mod.rs`
  - `src/protocol/mod.rs`
  - `src/transport/mod.rs`
  - `src/api/mod.rs`
  - `src/extension/mod.rs`
  - `src/error/mod.rs`

### 验收

- `cargo check` 通过。
- 未引入业务逻辑。

## PR-02 基础类型与错误骨架

### 目标

完成内核公共类型和错误分层框架。

### 文件级工作

- `src/core/session/types.rs`
  - `SessionId`。
- `src/transport/types.rs`
  - `TransportKind`。
- `src/protocol/version.rs`
  - 协议版本类型。
- `src/protocol/capability.rs`
  - `CapabilityKey/CapabilityValue` 基础定义。
- `src/error/mod.rs`
  - `TransportError/NegotiationError/ProtocolError/AuthError/StoreError/FrameworkError`。

### 测试

- `tests/types_tests.rs`：类型构造与边界测试。
- `tests/error_chain_tests.rs`：错误包装与 `source` 链测试。

## PR-03 协商协议数据结构

### 目标

定义握手数据模型与会话契约载体。

### 文件级工作

- `src/protocol/handshake/message.rs`
  - `ClientAdvertise`、`ServerSelect`。
- `src/protocol/handshake/contract.rs`
  - `CapabilityContract`。
- `src/protocol/handshake/mod.rs`
  - 导出公共类型。

### 测试

- `tests/handshake_message_tests.rs`
  - 结构合法性校验。
- 若启用序列化：增加序列化一致性测试。

## PR-04 协商选择算法

### 目标

实现交集计算、优先级选择、降级规则。

### 文件级工作

- `src/protocol/handshake/select.rs`
  - 版本选择算法。
  - transport 选择算法。
  - capability 子集过滤。
- `src/protocol/handshake/policy.rs`
  - 协商策略接口（偏好顺序、强制能力）。

### 测试

- `tests/negotiation_select_tests.rs`
  - 正常选择。
  - 无版本交集。
  - 无 transport 交集。
  - 必需 capability 缺失。

## PR-05 状态机与 SessionCore 初版

### 目标

实现会话状态迁移与基础核心对象。

### 文件级工作

- `src/core/state.rs`
  - runtime 状态枚举。
  - 迁移守卫函数。
- `src/core/session/core.rs`
  - `SessionCore`（会话 id、状态、契约、基础队列句柄）。
- `src/core/session/mod.rs`
  - 导出与组织。

### 测试

- `tests/session_state_tests.rs`
  - 合法迁移。
  - 非法迁移错误。

## PR-06 transport 抽象与 attach

### 目标

打通“协商成功 -> 绑定 transport -> 进入 Active”。

### 文件级工作

- `src/transport/handle.rs`
  - `TransportHandle` trait/object 边界。
- `src/transport/registry.rs`
  - transport 查找与构建入口。
- `src/core/session/attach.rs`
  - attach 流程（初次绑定）。

### 测试

- `tests/attach_flow_tests.rs`
  - attach 成功路径。
  - attach 前置条件失败。

## PR-07 重连 swap 与回滚

### 目标

实现重连时 transport 原子替换与失败回滚。

### 文件级工作

- `src/core/session/resume.rs`
  - resume token 校验接口调用。
- `src/core/session/swap.rs`
  - transport swap 原子流程。
  - 失败回滚逻辑。
- `src/core/session/consistency.rs`
  - 合约兼容性校验。

### 测试

- `tests/reconnect_swap_tests.rs`
  - 成功重连。
  - 不兼容拒绝。
  - swap 失败回滚。

## PR-08 扩展 trait 接入

### 目标

接入 auth/store/observability/middleware 可插拔扩展面。

### 文件级工作

- `src/extension/auth.rs`
  - `Authenticator` trait。
- `src/extension/store.rs`
  - `SessionStore` trait。
- `src/extension/observability.rs`
  - 事件与指标 hook。
- `src/extension/middleware.rs`
  - `tower` feature 下的 layer 边界。

### 测试

- `tests/extension_integration_tests.rs`
  - 扩展成功。
  - 可恢复失败。
  - 致命失败。

## PR-09 feature 组合与集成测试

### 目标

验证 feature 裁剪与端到端核心流程稳定性。

### 工作内容

- 增加 feature 组合测试脚本或 CI matrix。
- 补齐协商+状态机+重连集成测试。
- 覆盖最小集（如仅 `server`）与常用集（如 `server+ws+tracing`）。

### 验收

- 最小组合通过。
- 常用组合通过。
- 不同组合无编译破坏。

## PR-10 文档、示例与发布清单

### 目标

收尾并形成可发布状态。

### 工作内容

- `examples/` 增加最小 client/server 示例。
- 更新 `doc_design` / `doc_dev` 与代码一致性。
- 增加 `CHANGELOG` 初版与发布 checklist。
- 审查公开 API 注释与使用示例。

### 验收

- 示例可运行。
- 文档可指导新用户完成最小接入。

## 建议执行顺序与并行

- 主线顺序：`PR-01 -> PR-02 -> PR-03 -> PR-04 -> PR-05 -> PR-06 -> PR-07 -> PR-08 -> PR-09 -> PR-10`。
- 可并行点：
  - `PR-03` 与 `PR-05` 可并行起草（接口先对齐）。
  - `PR-08` 可在 `PR-06` 之后并行推进。

## 每个 PR 的统一模板（建议）

1. 范围（做什么）。
2. 非目标（不做什么）。
3. 设计约束引用（对应 `doc_design` 条目）。
4. 测试清单。
5. 回滚策略。
