# Badge

成熟度：`Alpha`。源码：`crates/slint-ui/ui/data-display/badge.slint`。公开名称：`Badge`。

用于紧凑地显示状态点或数量；详细状态文字使用 StatusBar 或 Alert。

## 公开 API

### 数据类型与枚举

#### `BadgeTone`

| 字段或值 | 类型/语义 |
|---|---|
| `neutral` | `枚举值` |
| `info` | `枚举值` |
| `success` | `枚举值` |
| `warning` | `枚举值` |
| `error` | `枚举值` |

### 属性

| 名称 | 方向 | 类型 | 默认值 | 所有者 | 说明与边界 |
|---|---|---|---|---|---|
| `count` | `in` | `int` | `0` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `max-count` | `in` | `int` | `99` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `show-zero` | `in` | `bool` | `false` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `dot` | `in` | `bool` | `false` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `tone` | `in` | `BadgeTone` | `BadgeTone.error` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `accessible-name` | `in` | `string` | `""` | 调用方 | 无可见文字的交互组件必须提供本地化名称 |
| `display-text` | `out` | `string` | `dot ? "" : count > max-count ? max-count + "+" : count` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |

## 视觉规范

### 组成结构

当前实现由 `Text` 组成。内部元素不是公开 API，调用方不得依赖其名称或坐标。

### 变体、尺寸与状态外观

视觉控制入口：`tone`。状态组合以公开属性和行为规范为准，不为 Gallery 虚构额外状态。

### Theme Token

实际消费：`Theme.badge-min-size`、`Theme.bg-layout`、`Theme.color-error`、`Theme.color-primary`、`Theme.color-success`、`Theme.color-warning`、`Theme.font-size-caption`、`Theme.font-weight-medium`、`Theme.icon-default`、`Theme.mode`、`Theme.space-2`、`Theme.status-indicator-size`、`Theme.text-on-accent`。不得在调用方通过任意颜色绕过这些语义 Token。

### 内容、布局与环境

遵循全局排版、密度、长文本、100%–200% 缩放、浅色/深色/高对比度和 LTR/RTL 规则；组件特有差异见本页组成结构及行为规范。

## 行为规范

### 状态与交互

覆盖零值隐藏/显示、dot、数量、超上限和五种 tone。

### 无障碍与本地化

使用 text 角色并暴露原始 count；dot 必须提供可访问名称。数字格式化需要时由宿主传入相邻说明。

### 验证、宿主职责与限制

Gallery 展示数量、99+ 和状态点；编译与截图验收。`count` 是整数，不承担业务单位或复数规则。

全局约束：[`foundations.md`](../../foundations.md)、[`interaction.md`](../../interaction.md)、[`accessibility.md`](../../accessibility.md)、[`content-and-localization.md`](../../content-and-localization.md)。
