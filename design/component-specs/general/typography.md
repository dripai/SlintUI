# Typography

成熟度：`Alpha`。源码：`crates/slint-ui/ui/primitives/typography.slint`。公开名称：`Typography`。

用于 caption、body、label 和三级标题。字段名称使用 `Label`，可交互链接使用后续 `Link`，不要用 Typography 模拟控件。

## 公开 API

### 数据类型与枚举

#### `TypographyStyle`

| 字段或值 | 类型/语义 |
|---|---|
| `caption` | `枚举值` |
| `body` | `枚举值` |
| `label` | `枚举值` |
| `title-small` | `枚举值` |
| `title-medium` | `枚举值` |
| `title-large` | `枚举值` |

### 属性

| 名称 | 方向 | 类型 | 默认值 | 所有者 | 说明与边界 |
|---|---|---|---|---|---|
| `style` | `in` | `TypographyStyle` | `TypographyStyle.body` | 调用方 | 取值范围、联动和非法值处理见行为规范 |

## 视觉规范

### 组成结构

组件本身承担可视结构，不公开额外内部元素；调用方只通过公开属性控制方向和外观。

### 变体、尺寸与状态外观

视觉控制入口：由内容和全局 Theme 决定。状态组合以公开属性和行为规范为准，不为 Gallery 虚构额外状态。

### Theme Token

实际消费：`Theme.direction`、`Theme.font-size-body`、`Theme.font-size-caption`、`Theme.font-size-title-large`、`Theme.font-size-title-medium`、`Theme.font-size-title-small`、`Theme.font-weight-medium`、`Theme.font-weight-regular`、`Theme.font-weight-semibold`、`Theme.line-height-body`、`Theme.line-height-caption`、`Theme.line-height-title-large`、`Theme.line-height-title-medium`、`Theme.line-height-title-small`、`Theme.text-primary`。不得在调用方通过任意颜色绕过这些语义 Token。

### 内容、布局与环境

遵循全局排版、密度、长文本、100%–200% 缩放、浅色/深色/高对比度和 LTR/RTL 规则；组件特有差异见本页组成结构及行为规范。

## 行为规范

### 状态与交互

没有 Hover、Pressed、Selected、Disabled 或 Loading 状态，不接收指针和键盘焦点。字号、字重、行高、颜色和 RTL 对齐由 Theme 决定。

### 无障碍与本地化

可见文本由原生 `Text` 暴露。支持换行、长文本、文本缩放和 RTL；业务文案由调用方本地化，组件不内置字符串。

### 验证、宿主职责与限制

Gallery“通用 / Typography”页展示全部字阶，长中文/英文/阿拉伯语和缩放通过全局环境验证；由编译与截图基线验收。全局规则见 [`foundations.md`](../../foundations.md)、[`accessibility.md`](../../accessibility.md) 和 [`content-and-localization.md`](../../content-and-localization.md)。

全局约束：[`foundations.md`](../../foundations.md)、[`interaction.md`](../../interaction.md)、[`accessibility.md`](../../accessibility.md)、[`content-and-localization.md`](../../content-and-localization.md)。
