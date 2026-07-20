# Statistic

成熟度：`Alpha`。源码：`crates/slint-ui/ui/data-display/statistic.slint`。公开名称：`Statistic`。

突出显示单个关键数值及辅助趋势说明，不替代图表。

## 公开 API

### 数据类型与枚举

#### `StatisticTone`

| 字段或值 | 类型/语义 |
|---|---|
| `neutral` | `枚举值` |
| `positive` | `枚举值` |
| `warning` | `枚举值` |
| `negative` | `枚举值` |

### 属性

| 名称 | 方向 | 类型 | 默认值 | 所有者 | 说明与边界 |
|---|---|---|---|---|---|
| `title` | `in` | `string` | `""` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `value` | `in` | `string` | `""` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `prefix` | `in` | `string` | `""` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `suffix` | `in` | `string` | `""` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `detail` | `in` | `string` | `""` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `tone` | `in` | `StatisticTone` | `StatisticTone.neutral` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `accessible-name` | `in` | `string` | `title` | 调用方 | 无可见文字的交互组件必须提供本地化名称 |

## 视觉规范

### 组成结构

当前实现由 `VerticalLayout`、`Text`、`HorizontalLayout` 组成。内部元素不是公开 API，调用方不得依赖其名称或坐标。

### 变体、尺寸与状态外观

视觉控制入口：`tone`。状态组合以公开属性和行为规范为准，不为 Gallery 虚构额外状态。

### Theme Token

实际消费：`Theme.card-min-width`、`Theme.color-error`、`Theme.color-success`、`Theme.color-warning`、`Theme.font-size-body`、`Theme.font-size-caption`、`Theme.font-size-title-large`、`Theme.font-size-title-small`、`Theme.font-weight-semibold`、`Theme.space-1`、`Theme.text-primary`、`Theme.text-secondary`、`Theme.text-tertiary`。不得在调用方通过任意颜色绕过这些语义 Token。

### 内容、布局与环境

遵循全局排版、密度、长文本、100%–200% 缩放、浅色/深色/高对比度和 LTR/RTL 规则；组件特有差异见本页组成结构及行为规范。

## 行为规范

### 状态与交互

非交互组件；支持 neutral、positive、warning、negative 语义色。

### 无障碍与本地化

暴露组合后的 accessible-value；数字、单位、正负号和趋势文案必须由宿主按 Locale 格式化。

### 验证、宿主职责与限制

Gallery 展示百分比与正向状态；编译和截图验收。当前不内建数字动画、等宽字体选择和趋势图标。

全局约束：[`foundations.md`](../../foundations.md)、[`interaction.md`](../../interaction.md)、[`accessibility.md`](../../accessibility.md)、[`content-and-localization.md`](../../content-and-localization.md)。
