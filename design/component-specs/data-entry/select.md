# Select

成熟度：`Alpha`。源码：`crates/slint-ui/ui/controls/select.slint`。公开名称：`Select`。

用于从有限字符串模型中单选。可编辑候选使用后续 ComboBox，多选使用后续 MultiSelect；P0 不伪造搜索 Select。

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
| `model` | `in` | `[string]` | `[]` | 调用方 | 取值范围、联动和非法值处理见行为规范；继承自 Slint StandardComboBox |
| `enabled` | `in` | `bool` | `true` | 调用方 | false 时阻止用户操作；程序设置不触发用户事件；继承自 Slint StandardComboBox |
| `current-index` | `in-out` | `int` | `-1` | 共享受控状态 | -1 表示无选择；越界值的处理见行为规范；继承自 Slint StandardComboBox |
| `current-value` | `out` | `string` | `""` | 组件派生 | 取值范围、联动和非法值处理见行为规范；继承自 Slint StandardComboBox |
| `has-focus` | `out` | `bool` | `false` | 组件派生 | 取值范围、联动和非法值处理见行为规范；继承自 Slint StandardComboBox |

### 事件

| 名称 | 参数 | 触发语义与边界 |
|---|---|---|
| `selected` | `value: string` | 用户选择有效且可用项目后触发一次；继承自 Slint StandardComboBox |

### 公开方法

| 名称 | 参数 | 状态、事件与边界 |
|---|---|---|
| `select-index()` | `index: int` | 当前实现约束：`if (self.enabled && index >= 0 && index < self.model.length && self.current-index != index) { root.current-index = index; root.selected(root.model[index]); }` |

## 视觉规范

### 组成结构

组件本身承担可视结构，不公开额外内部元素；调用方只通过公开属性控制方向和外观。

### 变体、尺寸与状态外观

视觉控制入口：`size`。状态组合以公开属性和行为规范为准，不为 Gallery 虚构额外状态。

### Theme Token

实际消费：`Theme.control-height-large`、`Theme.control-height-medium`、`Theme.control-height-small`、`Theme.control-min-width`。不得在调用方通过任意颜色绕过这些语义 Token。

### 内容、布局与环境

遵循全局排版、密度、长文本、100%–200% 缩放、浅色/深色/高对比度和 LTR/RTL 规则；组件特有差异见本页组成结构及行为规范。

## 行为规范

### 状态与交互

Default、Hover、Pressed、Focused、Disabled、展开、当前选择和空模型行为由原生 ComboBox 提供。点击打开 Popup，方向键选择，Enter 打开/确认，Esc 关闭；`select-index()` 仅在启用、索引有效且发生变化时更新并回调。

### 无障碍与本地化

保留 combobox 的 enabled、expanded、value 和展开动作，补充可访问名称。选项由调用方本地化；RTL、字体、Popup 关闭与焦点恢复沿用原生实现。

### 验证、宿主职责与限制

Gallery“数据录入 / Select”页和全局 Locale 入口分别展示业务选择与语言选择；自动测试覆盖程序化选择、重复/越界和禁用不变。Slint 1.17.1 不支持搜索，且标准控件样式在编译期选择，不能完全跟随运行时 Theme。

全局约束：[`foundations.md`](../../foundations.md)、[`interaction.md`](../../interaction.md)、[`accessibility.md`](../../accessibility.md)、[`content-and-localization.md`](../../content-and-localization.md)。
