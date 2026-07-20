# TimePicker

成熟度：`Alpha`。源码：`crates/slint-ui/ui/controls/time-picker.slint`。公开名称：`TimePicker`。

选择本地墙上时间，不包含日期、时区或持续时间语义。

## 公开 API

### 数据类型与枚举

#### `TimeValue`

| 字段或值 | 类型/语义 |
|---|---|
| `hour` | `int` |
| `minute` | `int` |
| `second` | `int` |

### 属性

| 名称 | 方向 | 类型 | 默认值 | 所有者 | 说明与边界 |
|---|---|---|---|---|---|
| `value` | `in-out` | `TimeValue` | `类型默认值` | 共享受控状态 | 取值范围、联动和非法值处理见行为规范 |
| `hour-options` | `in` | `[string]` | `类型默认值` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `hour-values` | `in` | `[int]` | `类型默认值` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `minute-options` | `in` | `[string]` | `类型默认值` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `minute-values` | `in` | `[int]` | `类型默认值` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `second-options` | `in` | `[string]` | `类型默认值` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `second-values` | `in` | `[int]` | `类型默认值` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `hour-index` | `in-out` | `int` | `-1` | 共享受控状态 | 取值范围、联动和非法值处理见行为规范 |
| `minute-index` | `in-out` | `int` | `-1` | 共享受控状态 | 取值范围、联动和非法值处理见行为规范 |
| `second-index` | `in-out` | `int` | `-1` | 共享受控状态 | 取值范围、联动和非法值处理见行为规范 |
| `show-seconds` | `in` | `bool` | `false` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `enabled` | `in` | `bool` | `true` | 调用方 | false 时阻止用户操作；程序设置不触发用户事件 |
| `accessible-name` | `in` | `string` | `""` | 调用方 | 无可见文字的交互组件必须提供本地化名称 |

### 事件

| 名称 | 参数 | 触发语义与边界 |
|---|---|---|
| `changed` | `value: TimeValue` | 组件接受一个不同的新值后触发一次 |

### 公开方法

| 名称 | 参数 | 状态、事件与边界 |
|---|---|---|
| `select()` | `hour: int, minute: int, second: int` | 当前实现约束：`if (enabled && hour >= 0 && hour <= 23 && minute >= 0 && minute <= 59 && second >= 0 && second <= 59) { root.value = { hour: hour, minute: minute, second: second }; root.changed...` |

## 视觉规范

### 组成结构

当前实现由 `HorizontalLayout`、`Select`、`Text` 组成。内部元素不是公开 API，调用方不得依赖其名称或坐标。

### 变体、尺寸与状态外观

视觉控制入口：由内容和全局 Theme 决定。状态组合以公开属性和行为规范为准，不为 Gallery 虚构额外状态。

### Theme Token

实际消费：`Theme.control-height-medium`、`Theme.font-size-body`、`Theme.space-2`、`Theme.text-secondary`。不得在调用方通过任意颜色绕过这些语义 Token。

### 内容、布局与环境

遵循全局排版、密度、长文本、100%–200% 缩放、浅色/深色/高对比度和 LTR/RTL 规则；组件特有差异见本页组成结构及行为规范。

## 行为规范

### 状态与交互

组合原生 Select；拒绝 24:00 及超出 0–59 的分秒值。

### 无障碍与本地化

根为 groupbox；12/24 小时、步长和本地化标签由宿主提供。

### 验证、宿主职责与限制

Gallery 和 Harness 覆盖有效与无效值。当前不内建 AM/PM、时区、文本解析和跨午夜范围。

全局约束：[`foundations.md`](../../foundations.md)、[`interaction.md`](../../interaction.md)、[`accessibility.md`](../../accessibility.md)、[`content-and-localization.md`](../../content-and-localization.md)。
