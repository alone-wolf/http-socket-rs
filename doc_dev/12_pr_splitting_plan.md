# PR 拆分计划（建议 10 个 PR）

本计划将 M0-M5 拆成“小步可审查”的 PR，降低回归风险。

## PR-01：工程骨架与 feature 声明

- 范围：模块目录、`mod.rs`、`Cargo.toml` feature。
- 不包含：任何业务/网络逻辑。
- 验收：默认构建可通过，feature 声明完整。

## PR-02：基础类型与错误骨架

- 范围：`SessionId`、`TransportKind`、版本类型、错误枚举骨架。
- 验收：错误链结构可用，基础类型有单元测试。

## PR-03：协商协议数据结构

- 范围：`ClientAdvertise`、`ServerSelect`、`CapabilityContract`。
- 验收：序列化/反序列化与结构验证测试通过。

## PR-04：协商选择算法

- 范围：交集计算、策略接口、降级规则。
- 验收：无交集/必需能力缺失路径覆盖。

## PR-05：状态机与 SessionCore 初版

- 范围：状态枚举、合法迁移表、`SessionCore` 主字段。
- 验收：状态迁移测试与错误上下文齐全。

## PR-06：transport 抽象与 attach

- 范围：`TransportHandle` 接口、初次 attach 流程。
- 验收：从协商到 Active 的主流程打通。

## PR-07：重连 swap 与回滚

- 范围：resume 校验接口、swap 原子替换、失败回滚。
- 验收：重连场景测试（成功/拒绝/回滚）通过。

## PR-08：扩展 trait 接入

- 范围：`Authenticator`、`SessionStore`、observability hook、middleware 边界。
- 验收：扩展可插拔，失败可分类上抛。

## PR-09：feature 组合与集成测试

- 范围：feature 裁剪测试、协商+状态机+重连集成测试。
- 验收：最小组合与常用组合均通过。

## PR-10：文档、示例与发布清单

- 范围：示例代码、API 文档、变更日志、发布前检查。
- 验收：文档与行为一致，发布 checklist 全部勾选。

## PR 合并规则

1. 每个 PR 必须包含“范围声明 + 非目标声明”。
2. 每个 PR 至少包含一组新增测试。
3. 禁止在单 PR 同时引入“架构变化 + 大量实现细节”。
4. 若发生设计偏移，先更新 `doc_design`/`doc_dev` 再改代码。
