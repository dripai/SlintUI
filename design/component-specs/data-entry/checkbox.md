# Checkbox

成熟度：`Alpha`。源码：`crates/slint-ui/ui/controls/checkbox.slint`。公开名称：`Checkbox`。

用于独立或列表中的选择状态。立即生效的开关使用 `Switch`，互斥选择使用 `SegmentedControl` 或后续 RadioGroup。

## 公开 API

### 数据类型与枚举

#### `CheckState`

| 字段或值 | 类型/语义 |
|---|---|
| `unchecked` | `枚举值` |
| `checked` | `枚举值` |
| `indeterminate` | `枚举值` |

### 属性

| 名称 | 方向 | 类型 | 默认值 | 所有者 | 说明与边界 |
|---|---|---|---|---|---|
| `text` | `in` | `string` | `""` | 调用方 | 程序赋值不伪造用户编辑事件 |
| `state` | `in-out` | `CheckState` | `CheckState.unchecked` | 共享受控状态 | 取值范围、联动和非法值处理见行为规范 |
| `enabled` | `in` | `bool` | `true` | 调用方 | false 时阻止用户操作；程序设置不触发用户事件 |
| `accessible-name` | `in` | `string` | `text` | 调用方 | 无可见文字的交互组件必须提供本地化名称 |
| `has-focus` | `out` | `bool` | `绑定：focus-scope.has-focus` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |

### 事件

| 名称 | 参数 | 触发语义与边界 |
|---|---|---|
| `toggled` | `state: CheckState` | 由该组件定义的有效用户操作或等效公开方法触发；阻止条件见行为规范 |

### 公开方法

| 名称 | 参数 | 状态、事件与边界 |
|---|---|---|
| `toggle()` | `无` | 当前实现约束：`if (enabled) { root.state = root.state == CheckState.checked ? CheckState.unchecked : CheckState.checked; root.toggled(root.state); }` |

## 视觉规范

### 组成结构

当前实现由 `HorizontalLayout`、`Rectangle`、`Image`、`FocusRing`、`Text`、`TouchArea`、`FocusScope` 组成。内部元素不是公开 API，调用方不得依赖其名称或坐标。

### 变体、尺寸与状态外观

视觉控制入口：`state`。状态组合以公开属性和行为规范为准，不为 Gallery 虚构额外状态。

### Theme Token

实际消费：`Theme.bg-control`、`Theme.bg-disabled`、`Theme.border-default`、`Theme.border-width`、`Theme.color-primary-fill`、`Theme.color-primary-hover`、`Theme.control-height-small`、`Theme.focus-outline`、`Theme.font-size-body`、`Theme.icon-size-small`、`Theme.icon-size-x-small`、`Theme.line-height-body`、`Theme.radius-small`、`Theme.space-2`、`Theme.text-disabled`、`Theme.text-on-accent`、`Theme.text-primary`。不得在调用方通过任意颜色绕过这些语义 Token。

### 内容、布局与环境

遵循全局排版、密度、长文本、100%–200% 缩放、浅色/深色/高对比度和 LTR/RTL 规则；组件特有差异见本页组成结构及行为规范。

## 行为规范

### 状态与交互

支持 Hover、Focused、Disabled 和三态视觉。点击或 Space 调用 `toggle()`，用户切换从 indeterminate/unchecked 进入 checked、从 checked 进入 unchecked，并在更新后触发一次回调；禁用不变化。

### 无障碍与本地化

角色为 checkbox，暴露 enabled、checkable、checked 和默认动作。checked/indeterminate 使用 `check.svg`/`minus.svg`，不依赖字体。Slint 1.17.1 只能公开布尔 checked，mixed 读屏状态仍受上游限制。

### 验证、宿主职责与限制

Gallery“数据录入 / Checkbox”页覆盖 checked 和多语言；indeterminate、disabled 由交互 Harness 与组件规格验证。完整规则见四份全局规范。

全局约束：[`foundations.md`](../../foundations.md)、[`interaction.md`](../../interaction.md)、[`accessibility.md`](../../accessibility.md)、[`content-and-localization.md`](../../content-and-localization.md)。
