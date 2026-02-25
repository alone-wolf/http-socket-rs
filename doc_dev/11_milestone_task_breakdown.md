# M0-M5 实施任务拆分（模块级）

本文件将 `03_milestone_and_acceptance.md` 进一步拆分为“可执行任务 + 产出物 + 依赖关系”。

## M0：工程骨架与边界固定

### 任务清单

1. 定义目录与模块骨架（`core/protocol/transport/api/extension/error`）。
2. 在 `Cargo.toml` 定义 feature 开关（`ws/sse/poll/client/server/tower/tracing/metrics`）。
3. 建立最小公共类型（`SessionId`、`ProtocolVersion`、`TransportKind`、`CapabilityKey`）。
4. 约束依赖方向（禁止 core 反向依赖 api）。
5. 加入基础 CI 编译检查（默认 feature + 最小 feature）。

### 产出物

- 模块目录与 `mod.rs`。
- feature 列表与默认组合声明。
- 基础公共类型定义文档注释。

### 依赖

- 无（首阶段）。

## M1：协商协议与能力契约

### 任务清单

1. 定义 `ClientAdvertise` / `ServerSelect` 数据结构。
2. 实现“版本交集 + transport 交集 + capability 子集”选择逻辑。
3. 定义协商策略接口（优先级、强制能力、降级规则）。
4. 实现协商结果 `CapabilityContract`。
5. 增加协商失败错误分类（无版本交集、无传输交集、必需能力缺失）。

### 产出物

- 协商核心函数与策略接口。
- `CapabilityContract` 类型及验证方法。
- 协商单元测试与反例测试。

### 依赖

- 依赖 M0 的基础类型。

## M2：SessionCore 与状态机

### 任务清单

1. 定义 runtime 状态枚举（`Connecting/Negotiating/Active/Resuming/Draining/Closed`）。
2. 实现状态迁移守卫（合法迁移表）。
3. 建立 `SessionCore`（协议状态、契约、发送队列引用）。
4. 将协商结果绑定到 session 契约。
5. 对非法迁移输出结构化错误与状态上下文。

### 产出物

- 状态机迁移实现与 guard。
- `SessionCore` 基础实现。
- 状态机测试（合法/非法迁移）。

### 依赖

- 依赖 M1 的协商产物。

## M3：Transport 绑定与重连交换

### 任务清单

1. 定义 `TransportHandle` 抽象与 `TransportRegistry`。
2. 实现 `SessionCore` 的 attach/swap 原子流程。
3. 实现 resume token 校验接口（可由扩展实现）。
4. 定义重连时契约兼容检查。
5. 处理 swap 失败回滚路径（保持旧连接或进入可恢复态）。

### 产出物

- attach/swap 生命周期代码。
- 重连流程测试（同 transport / 跨 transport / 拒绝 attach）。
- 并发安全说明与临界区策略。

### 依赖

- 依赖 M2 状态机与 session 主体。

## M4：扩展面接入（auth/store/obs/middleware）

### 任务清单

1. 定义 `Authenticator` trait 与调用时机。
2. 定义 `SessionStore` trait（保存/读取/过期语义）。
3. 定义 observability hook（事件、span、metrics 抽象）。
4. 定义 middleware 接入边界（tower feature 启用时可用）。
5. 统一扩展错误到 `FrameworkError`。

### 产出物

- 扩展 trait 集与默认空实现。
- 扩展调用路径与失败处理策略。
- 扩展集成测试（成功/可恢复失败/致命失败）。

### 依赖

- 依赖 M3 的 session/runtime 主流程。

## M5：稳定性、文档、发布准备

### 任务清单

1. 补齐测试矩阵覆盖（协商、状态机、重连、扩展、feature 组合）。
2. 完成示例与 API 文档（client/server 最小可运行样例）。
3. 进行 feature 裁剪验证（启用/禁用组合）。
4. 整理错误码与观测事件字典。
5. 发布前兼容性检查与版本说明。

### 产出物

- 可复现测试矩阵报告。
- 开发文档与示例同步更新。
- 发布检查清单结果。

### 依赖

- 依赖 M0-M4 全部产物。

## 并行执行建议

- 可并行 A：M1 协商数据结构 与 M2 状态机框架（先对齐接口）。
- 可并行 B：M3 transport 抽象 与 M4 扩展 trait 定义。
- 不可并行：M5 必须在前置里程碑稳定后进行。
