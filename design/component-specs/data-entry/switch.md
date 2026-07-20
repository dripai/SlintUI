# Switch

成熟度：`Alpha`。源码：`crates/slint-ui/ui/controls/switch.slint`。公开名称：`Switch`。

用于立即生效的开/关设置。需要提交后才生效或表达多选时使用 Checkbox；互斥模式使用 SegmentedControl。

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
| `text` | `in` | `string` | `""` | 调用方 | 程序赋值不伪造用户编辑事件 |
| `checked` | `in-out` | `bool` | `false` | 共享受控状态 | 取值范围、联动和非法值处理见行为规范 |
| `enabled` | `in` | `bool` | `true` | 调用方 | false 时阻止用户操作；程序设置不触发用户事件 |
| `size` | `in` | `ControlSize` | `ControlSize.medium` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `accessible-name` | `in` | `string` | `text` | 调用方 | 无可见文字的交互组件必须提供本地化名称 |
| `has-focus` | `out` | `bool` | `绑定：focus-scope.has-focus` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |

### 事件

| 名称 | 参数 | 触发语义与边界 |
|---|---|---|
| `toggled` | `checked: bool` | 由该组件定义的有效用户操作或等效公开方法触发；阻止条件见行为规范 |

### 公开方法

| 名称 | 参数 | 状态、事件与边界 |
|---|---|---|
| `toggle()` | `无` | 当前实现约束：`if (enabled) { root.checked = !root.checked; root.toggled(root.checked); }` |

## 视觉规范

### 组成结构

当前实现由 `HorizontalLayout`、`Rectangle`、`FocusRing`、`Text`、`TouchArea`、`FocusScope` 组成。内部元素不是公开 API，调用方不得依赖其名称或坐标。

### 变体、尺寸与状态外观

视觉控制入口：`checked`、`size`。状态组合以公开属性和行为规范为准，不为 Gallery 虚构额外状态。

### Theme Token

实际消费：`Theme.bg-disabled`、`Theme.bg-surface`、`Theme.border-default`、`Theme.color-primary-fill`、`Theme.color-primary-fill-active`、`Theme.color-primary-fill-hover`、`Theme.control-height-small`、`Theme.fill-hover`、`Theme.fill-pressed`、`Theme.font-size-body`、`Theme.line-height-body`、`Theme.motion-fast`、`Theme.space-2`、`Theme.switch-height-large`、`Theme.switch-height-medium`、`Theme.switch-height-small`、`Theme.switch-thumb-inset`、`Theme.switch-width-large`、`Theme.switch-width-medium`、`Theme.switch-width-small`、`Theme.text-disabled`、`Theme.text-primary`。不得在调用方通过任意颜色绕过这些语义 Token。

### 内容、布局与环境

遵循全局排版、密度、长文本、100%–200% 缩放、浅色/深色/高对比度和 LTR/RTL 规则；组件特有差异见本页组成结构及行为规范。

## 行为规范

### 状态与交互

支持 Default、Hover、Pressed、Focused、Disabled、Checked。点击或 Space 切换，在状态更新后触发一次回调；禁用不变化。滑块动画使用 Theme 动效并服从 reduced-motion。

### 无障碍与本地化

角色为 switch，暴露 enabled、checkable、checked 和默认动作。文字由调用方本地化，支持长文本、RTL 和文本缩放；不能只靠轨道颜色表达值。

### 验证、宿主职责与限制

Gallery“数据录入 / Switch”页展示开关状态；自动测试覆盖正常切换和禁用不变。

全局约束：[`foundations.md`](../../foundations.md)、[`interaction.md`](../../interaction.md)、[`accessibility.md`](../../accessibility.md)、[`content-and-localization.md`](../../content-and-localization.md)。
