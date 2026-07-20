# DatePicker

成熟度：`Alpha`。源码：`crates/slint-ui/ui/controls/date-picker.slint`。公开名称：`DatePicker`。

选择单个公历日期；日期范围、时区和业务禁用规则由宿主处理。

## 公开 API

### 数据类型与枚举

#### `DateValue`

| 字段或值 | 类型/语义 |
|---|---|
| `year` | `int` |
| `month` | `int` |
| `day` | `int` |

### 属性

| 名称 | 方向 | 类型 | 默认值 | 所有者 | 说明与边界 |
|---|---|---|---|---|---|
| `value` | `in-out` | `DateValue` | `类型默认值` | 共享受控状态 | 取值范围、联动和非法值处理见行为规范 |
| `year-options` | `in` | `[string]` | `类型默认值` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `year-values` | `in` | `[int]` | `类型默认值` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `month-options` | `in` | `[string]` | `类型默认值` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `month-values` | `in` | `[int]` | `类型默认值` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `day-options` | `in` | `[string]` | `类型默认值` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `day-values` | `in` | `[int]` | `类型默认值` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `year-index` | `in-out` | `int` | `-1` | 共享受控状态 | 取值范围、联动和非法值处理见行为规范 |
| `month-index` | `in-out` | `int` | `-1` | 共享受控状态 | 取值范围、联动和非法值处理见行为规范 |
| `day-index` | `in-out` | `int` | `-1` | 共享受控状态 | 取值范围、联动和非法值处理见行为规范 |
| `enabled` | `in` | `bool` | `true` | 调用方 | false 时阻止用户操作；程序设置不触发用户事件 |
| `accessible-name` | `in` | `string` | `""` | 调用方 | 无可见文字的交互组件必须提供本地化名称 |
| `year-accessible-name` | `in` | `string` | `""` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `month-accessible-name` | `in` | `string` | `""` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `day-accessible-name` | `in` | `string` | `""` | 调用方 | 取值范围、联动和非法值处理见行为规范 |

### 事件

| 名称 | 参数 | 触发语义与边界 |
|---|---|---|
| `changed` | `value: DateValue` | 组件接受一个不同的新值后触发一次 |

### 公开方法

| 名称 | 参数 | 状态、事件与边界 |
|---|---|---|
| `select()` | `year: int, month: int, day: int` | 当前实现约束：`if (enabled && month >= 1 && month <= 12 && day >= 1 && day <= 31) { root.value = { year: year, month: month, day: day }; root.changed(root.value); }` |

## 视觉规范

### 组成结构

当前实现由 `HorizontalLayout`、`Select` 组成。内部元素不是公开 API，调用方不得依赖其名称或坐标。

### 变体、尺寸与状态外观

视觉控制入口：由内容和全局 Theme 决定。状态组合以公开属性和行为规范为准，不为 Gallery 虚构额外状态。

### Theme Token

实际消费：`Theme.control-height-medium`、`Theme.space-2`。不得在调用方通过任意颜色绕过这些语义 Token。

### 内容、布局与环境

遵循全局排版、密度、长文本、100%–200% 缩放、浅色/深色/高对比度和 LTR/RTL 规则；组件特有差异见本页组成结构及行为规范。

## 行为规范

### 状态与交互

组合三个原生 Select；拒绝月份 1–12、日期 1–31 之外的程序化值。

### 无障碍与本地化

根为 groupbox，各字段独立命名；显示顺序、选项与格式由调用方按 Locale 提供。

### 验证、宿主职责与限制

Gallery 和 Harness 覆盖选择与边界。组件不计算月天数、闰年、日期范围、自然语言输入或弹出月历。

全局约束：[`foundations.md`](../../foundations.md)、[`interaction.md`](../../interaction.md)、[`accessibility.md`](../../accessibility.md)、[`content-and-localization.md`](../../content-and-localization.md)。
