# NumberInput

成熟度：`Alpha`。源码：`crates/slint-ui/ui/controls/number-input.slint`。公开名称：`NumberInput`。

用于有上下界和步长的整数输入；自由格式数字、货币和单位格式化由宿主层处理。

## 公开 API

### 数据类型与枚举

#### `ControlSize`

| 字段或值 | 类型/语义 |
|---|---|
| `small` | `枚举值` |
| `medium` | `枚举值` |
| `large` | `枚举值` |

### 属性

| 名称 | 方向 | 类型 | 默认值 | 所有者 | 说明与边界 |
|---|---|---|---|---|---|
| `accessible-name` | `in` | `string` | `""` | 调用方 | 无可见文字的交互组件必须提供本地化名称 |
| `size` | `in` | `ControlSize` | `ControlSize.medium` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `value` | `in-out` | `int` | `0` | 共享受控状态 | 取值范围、联动和非法值处理见行为规范；继承自 Slint NativeSpinBox |
| `minimum` | `in` | `int` | `0` | 调用方 | 取值范围、联动和非法值处理见行为规范；继承自 Slint NativeSpinBox |
| `maximum` | `in` | `int` | `100` | 调用方 | 取值范围、联动和非法值处理见行为规范；继承自 Slint NativeSpinBox |
| `step-size` | `in` | `int` | `1` | 调用方 | 取值范围、联动和非法值处理见行为规范；继承自 Slint NativeSpinBox |
| `enabled` | `in` | `bool` | `true` | 调用方 | false 时阻止用户操作；程序设置不触发用户事件；继承自 Slint NativeSpinBox |
| `read-only` | `in` | `bool` | `false` | 调用方 | true 时允许读取和焦点，不接受用户编辑；继承自 Slint NativeSpinBox |

### 事件

| 名称 | 参数 | 触发语义与边界 |
|---|---|---|
| `edited` | `value: int` | 用户编辑原始值时触发；程序赋值不触发；继承自 Slint NativeSpinBox |

### 公开方法

| 名称 | 参数 | 状态、事件与边界 |
|---|---|---|
| `set-value()` | `next: int` | 当前实现约束：`if (root.enabled && !root.read-only && root.value != clamp(next, root.minimum, root.maximum)) { root.value = clamp(next, root.minimum, root.maximum); root.edited(root.value); }` |
| `step-by()` | `direction: int` | 当前实现约束：`root.set-value(root.value + direction * root.step-size);` |

## 视觉规范

### 组成结构

组件本身承担可视结构，不公开额外内部元素；调用方只通过公开属性控制方向和外观。

### 变体、尺寸与状态外观

视觉控制入口：`size`、`step-size`。状态组合以公开属性和行为规范为准，不为 Gallery 虚构额外状态。

### Theme Token

实际消费：`Theme.control-height-large`、`Theme.control-height-medium`、`Theme.control-height-small`、`Theme.control-min-width`。不得在调用方通过任意颜色绕过这些语义 Token。

### 内容、布局与环境

遵循全局排版、密度、长文本、100%–200% 缩放、浅色/深色/高对比度和 LTR/RTL 规则；组件特有差异见本页组成结构及行为规范。

## 行为规范

### 状态与交互

覆盖 Default、Hover、Focused、Disabled、ReadOnly、上下界和步进；程序化设置夹取到范围，同值不重复回调。

### 无障碍与本地化

复用 Slint 原生 spinbox 角色、范围、步长和增减动作。Locale 格式化不在组件内完成。

### 验证、宿主职责与限制

Gallery 展示范围、步长和禁用；Harness 覆盖重复、越界夹取与禁用。当前只支持整数，这是 Slint 原生 SpinBox 的公开类型边界。

全局约束：[`foundations.md`](../../foundations.md)、[`interaction.md`](../../interaction.md)、[`accessibility.md`](../../accessibility.md)、[`content-and-localization.md`](../../content-and-localization.md)。
