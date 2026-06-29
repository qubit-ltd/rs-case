# Qubit Case

[![Rust CI](https://github.com/qubit-ltd/rs-case/actions/workflows/ci.yml/badge.svg)](https://github.com/qubit-ltd/rs-case/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/endpoint?url=https://qubit-ltd.github.io/rs-case/coverage-badge.json)](https://qubit-ltd.github.io/rs-case/coverage/)
[![Crates.io](https://img.shields.io/crates/v/qubit-case.svg?color=blue)](https://crates.io/crates/qubit-case)
[![Rust](https://img.shields.io/badge/rust-1.94+-blue.svg?logo=rust)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![English Document](https://img.shields.io/badge/Document-English-blue.svg)](README.md)

面向 Rust 的命名风格检测和转换工具库。

## 概述

Qubit Naming Style 用于在 Java、C++、Python、XML、JSON 和 Rust 工具链常见的 ASCII 标识符命名风格之间转换，提供紧凑的 Rust 枚举 API。

## 设计目标

- **小型 API 表面**：只暴露一个 `CaseStyle` 枚举和五个常量。
- **一致行为**：对受支持的 ASCII 标识符转换保持稳定，兼容常见命名风格。
- **严格匹配**：检测字符串是否符合某种命名风格。
- **尽力转换**：即使输入并不严格符合源风格，也会尽量给出转换结果。
- **无运行时依赖**：保持库轻量。

## 支持的风格

- `LOWER_HYPHEN`: `lower-hyphen`
- `LOWER_UNDERSCORE`: `lower_underscore`
- `LOWER_CAMEL`: `lowerCamel`
- `UPPER_CAMEL`: `UpperCamel`
- `UPPER_UNDERSCORE`: `UPPER_UNDERSCORE`

## 安装

在 `Cargo.toml` 中添加：

```toml
[dependencies]
qubit-case = "0.1"
```

## 快速开始

```rust
use qubit_case::{
    LOWER_CAMEL,
    LOWER_HYPHEN,
    UPPER_UNDERSCORE,
};

let value = LOWER_HYPHEN.to(LOWER_CAMEL, "hello-world");
assert_eq!(value, "helloWorld");

let constant = LOWER_CAMEL.to(UPPER_UNDERSCORE, "http2Client");
assert_eq!(constant, "HTTP_2_CLIENT");

assert!(UPPER_UNDERSCORE.matches("HTTP_2_CLIENT"));
```

## API 参考

### 类型

- [`CaseStyle`](https://docs.rs/qubit-case/latest/qubit_case/enum.CaseStyle.html) -
  支持的命名风格和转换方法。
- [`CaseStyleError`](https://docs.rs/qubit-case/latest/qubit_case/struct.CaseStyleError.html) -
  解析未知风格名称时返回的错误。

### 常量

- `LOWER_HYPHEN`
- `LOWER_UNDERSCORE`
- `LOWER_CAMEL`
- `UPPER_CAMEL`
- `UPPER_UNDERSCORE`

### 方法

- `CaseStyle::values()` 按参考实现顺序返回全部风格。
- `CaseStyle::of(name)` 解析风格名称，忽略大小写，并把连字符和下划线视为等价。
- `CaseStyle::name()` 返回规范的小写连字符风格名称。
- `CaseStyle::word_separator()` 返回该风格使用的单词分隔符。
- `CaseStyle::to(target, value)` 把标识符转换为目标风格。
- `CaseStyle::matches(value)` 检查标识符是否严格符合该风格。

## 测试与代码覆盖率

本项目覆盖成功转换、风格解析、风格匹配、CamelCase 缩写边界、数字边界、空输入和尽力转换行为。

### 运行测试

```bash
# 运行所有测试
cargo test

# 运行覆盖率报告
./coverage.sh

# 生成文本格式报告
./coverage.sh text

# 运行 CI 检查（格式化、clippy、测试、覆盖率、audit）
./ci-check.sh
```

### 覆盖率指标

详细的覆盖率统计请参见 [COVERAGE.zh_CN.md](COVERAGE.zh_CN.md)。

## 依赖项

本 crate 没有运行时依赖。

## 许可证

Copyright (c) 2025 - 2026. Haixing Hu.

根据 Apache 许可证 2.0 版（"许可证"）授权；
除非遵守许可证，否则您不得使用此文件。
您可以在以下位置获取许可证副本：

    http://www.apache.org/licenses/LICENSE-2.0

除非适用法律要求或书面同意，否则根据许可证分发的软件
按"原样"分发，不附带任何明示或暗示的担保或条件。
有关许可证下的特定语言管理权限和限制，请参阅许可证。

完整的许可证文本请参阅 [LICENSE](LICENSE)。

## 贡献

欢迎贡献！请随时提交 Pull Request。

### 开发指南

- 遵循 Rust API 指南。
- 保持全面的测试覆盖。
- 在文档能帮助理解时，为公共 API 提供示例。
- 提交 PR 前运行 `./ci-check.sh`。

## 作者

Haixing Hu

## 相关项目

Qubit 旗下的更多 Rust 库发布在 GitHub 组织 [qubit-ltd](https://github.com/qubit-ltd)。

---

仓库地址：[https://github.com/qubit-ltd/rs-case](https://github.com/qubit-ltd/rs-case)
