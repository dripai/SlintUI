# Divider

成熟度：`Alpha`。源码：`crates/slint-ui/ui/primitives/divider.slint`。公开名称：`Divider`。

用于相邻内容的视觉分隔，不用于表达标题层级、可点击拖动条或 SplitPane。

## 公开 API

### 数据类型与枚举

#### `DividerOrientation`

| 字段或值 | 类型/语义 |
|---|---|
| `horizontal` | `枚举值` |
| `vertical` | `枚举值` |

### 属性

| 名称 | 方向 | 类型 | 默认值 | 所有者 | 说明与边界 |
|---|---|---|---|---|---|
| `orientation` | `in` | `DividerOrientation` | `DividerOrientation.horizontal` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `subtle` | `in` | `bool` | `true` | 调用方 | 取值范围、联动和非法值处理见行为规范 |

## 视觉规范

### 组成结构

组件本身承担可视结构，不公开额外内部元素；调用方只通过公开属性控制方向和外观。

### 变体、尺寸与状态外观

视觉控制入口：`orientation`、`subtle`。状态组合以公开属性和行为规范为准，不为 Gallery 虚构额外状态。

### Theme Token

实际消费：`Theme.border-default`、`Theme.border-secondary`、`Theme.border-width`、`Theme.layout-unbounded-extent`、`Theme.separator-min-extent`。不得在调用方通过任意颜色绕过这些语义 Token。

### 内容、布局与环境

遵循全局排版、密度、长文本、100%–200% 缩放、浅色/深色/高对比度和 LTR/RTL 规则；组件特有差异见本页组成结构及行为规范。

## 行为规范

### 状态与交互

分隔不改变阅读顺序。厚度与颜色来自 Theme，高对比度下随边框 Token 增强。

### 验证、宿主职责与限制

Gallery“布局 / Divider”页集中展示水平和垂直组合；由编译和截图验收。本组件不提供文字标签，完整规则见 [`foundations.md`](../../foundations.md) 和 [`accessibility.md`](../../accessibility.md)。

全局约束：[`foundations.md`](../../foundations.md)、[`interaction.md`](../../interaction.md)、[`accessibility.md`](../../accessibility.md)、[`content-and-localization.md`](../../content-and-localization.md)。
