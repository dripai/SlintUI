# RadioGroup

成熟度：`Alpha`。源码：`crates/slint-ui/ui/controls/radio.slint`。公开名称：`RadioGroup`。

用于必须至多选择一个值的选项组；多选使用 CheckboxGroup，少量视图模式切换可使用 SegmentedControl。

## 公开 API

### 数据类型与枚举

#### `RadioGroupItem`

| 字段或值 | 类型/语义 |
|---|---|
| `text` | `string` |
| `value` | `string` |
| `enabled` | `bool` |

### 属性

| 名称 | 方向 | 类型 | 默认值 | 所有者 | 说明与边界 |
|---|---|---|---|---|---|
| `items` | `in` | `[RadioGroupItem]` | `类型默认值` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `current-index` | `in-out` | `int` | `-1` | 共享受控状态 | -1 表示无选择；越界值的处理见行为规范 |
| `enabled` | `in` | `bool` | `true` | 调用方 | false 时阻止用户操作；程序设置不触发用户事件 |
| `accessible-name` | `in` | `string` | `""` | 调用方 | 无可见文字的交互组件必须提供本地化名称 |
| `error-text` | `in` | `string` | `""` | 调用方 | 取值范围、联动和非法值处理见行为规范 |

### 事件

| 名称 | 参数 | 触发语义与边界 |
|---|---|---|
| `changed` | `index: int, value: string` | 组件接受一个不同的新值后触发一次 |

### 公开方法

| 名称 | 参数 | 状态、事件与边界 |
|---|---|---|
| `select()` | `index: int` | 当前实现约束：`if (enabled && index >= 0 && index < items.length && items[index].enabled && current-index != index) { root.current-index = index; root.changed(index, items[index].value); }` |
| `select-next()` | `step: int` | 当前实现约束：`if (items.length > 0) { root.select(Math.mod(current-index + step + items.length, items.length)); }` |

## 视觉规范

### 组成结构

当前实现由 `VerticalLayout`、`Radio`、`Text` 组成。内部元素不是公开 API，调用方不得依赖其名称或坐标。

### 变体、尺寸与状态外观

视觉控制入口：由内容和全局 Theme 决定。状态组合以公开属性和行为规范为准，不为 Gallery 虚构额外状态。

### Theme Token

实际消费：`Theme.color-error`、`Theme.font-size-caption`、`Theme.space-2`。不得在调用方通过任意颜色绕过这些语义 Token。

### 内容、布局与环境

遵循全局排版、密度、长文本、100%–200% 缩放、浅色/深色/高对比度和 LTR/RTL 规则；组件特有差异见本页组成结构及行为规范。

## 行为规范

### 状态与交互

覆盖 Unchecked、Checked、Hover、Focused、Disabled、组错误和长文本；同值、禁用项和越界不回调。

### 无障碍与本地化

Radio 使用 radio-button，组使用 groupbox；暴露 checked、enabled、名称和错误说明。显示文字与稳定 value 分离。

### 验证、宿主职责与限制

Gallery 展示互斥选择和禁用项；Harness 覆盖正常、重复、禁用和越界。Slint 1.17.1 不能统一接管重复子项焦点，方向键由 `select-next()` 协议承接。

全局约束：[`foundations.md`](../../foundations.md)、[`interaction.md`](../../interaction.md)、[`accessibility.md`](../../accessibility.md)、[`content-and-localization.md`](../../content-and-localization.md)。
