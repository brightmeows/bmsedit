# bmsedit

## 项目定位

供大模型调用的 BMS/BMSON 谱面编辑 CLI，是 `bmsrs` facade 的下游消费者。

## 命令

```bash
cargo fmt --all --check
cargo clippy --all-targets -- -D warnings
cargo test --quiet
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps
pre-commit run --all-files
```

## 依赖

`bmsrs` 以 git `rev` 锁定。升级步骤：

1. 修改 `Cargo.toml` 中的 `rev`
2. 运行 `cargo update -p bmsrs`
3. 运行 `cargo test`

本地联调时，复制 `.cargo/config.toml.example` 为 `.cargo/config.toml`，
把 `[patch]` 指向本地 `bmsrs` checkout。`.cargo/config.toml` 不提交。

## 约定

- 只通过 `bmsrs` facade 使用库 API，不直连子 crate。
- `[lints]`、`clippy.toml`、`.pre-commit-config.yaml` 复制自 `bmsrs`；
  `bmsrs` 调整时同步更新。
- `publish = false`：本仓只维护源码，不做 Release / crates.io 分发。
