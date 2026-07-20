# ToggleButton

成熟度：`Alpha`。源码：`crates/slint-ui/ui/controls/toggle-button.slint`。公开名称：`ToggleButton`。

用于可保持按下状态的独立命令；表单布尔值使用 Checkbox，立即生效的设置使用 Switch。

## 公开 API

### 数据类型与枚举

#### `ButtonVariant`

| 字段或值 | 类型/语义 |
|---|---|
| `default` | `枚举值` |
| `primary` | `枚举值` |
| `danger` | `枚举值` |
| `text` | `枚举值` |
| `link` | `枚举值` |

#### `ControlSize`

| 字段或值 | 类型/语义 |
|---|---|
| `small` | `枚举值` |
| `medium` | `枚举值` |
| `large` | `枚举值` |

### 属性

| 名称 | 方向 | 类型 | 默认值 | 所有者 | 说明与边界 |
|---|---|---|---|---|---|
| `text` | `in` | `string` | `""` | 调用方 | 程序赋值不伪造用户编辑事件；继承自 Button |
| `icon` | `in` | `image` | `类型默认值` | 调用方 | 取值范围、联动和非法值处理见行为规范；继承自 Button |
| `variant` | `in` | `ButtonVariant` | `ButtonVariant.default` | 调用方 | 取值范围、联动和非法值处理见行为规范；继承自 Button |
| `size` | `in` | `ControlSize` | `ControlSize.medium` | 调用方 | 取值范围、联动和非法值处理见行为规范；继承自 Button |
| `enabled` | `in` | `bool` | `true` | 调用方 | false 时阻止用户操作；程序设置不触发用户事件；继承自 Button |
| `loading` | `in` | `bool` | `false` | 调用方 | true 时显示进行中状态并阻止重复操作；继承自 Button |
| `checkable` | `in` | `bool` | `false` | 调用方 | 取值范围、联动和非法值处理见行为规范；继承自 Button |
| `checked` | `in-out` | `bool` | `false` | 共享受控状态 | 取值范围、联动和非法值处理见行为规范；继承自 Button |
| `keyboard-focusable` | `in` | `bool` | `true` | 调用方 | 取值范围、联动和非法值处理见行为规范；继承自 Button |
| `radius` | `in` | `length` | `Theme.radius-medium` | 调用方 | 取值范围、联动和非法值处理见行为规范；继承自 Button |
| `accessible-name` | `in` | `string` | `text` | 调用方 | 无可见文字的交互组件必须提供本地化名称；继承自 Button |
| `has-focus` | `out` | `bool` | `绑定：focus-scope.has-focus` | 组件派生 | 取值范围、联动和非法值处理见行为规范；继承自 Button |

### 事件

| 名称 | 参数 | 触发语义与边界 |
|---|---|---|
| `clicked` | `无` | 一次被接受的用户激活完成后触发一次；继承自 Button |

### 公开方法

| 名称 | 参数 | 状态、事件与边界 |
|---|---|---|
| `activate()` | `无` | 当前实现约束：`if (interactive) { if (checkable) { root.checked = !root.checked; } root.clicked(); }`；继承自 Button |

## 视觉规范

### 组成结构

当前实现由 `HorizontalLayout`、`Spinner`、`Image`、`Text`、`TouchArea`、`FocusScope`、`FocusRing` 组成。内部元素不是公开 API，调用方不得依赖其名称或坐标。

### 变体、尺寸与状态外观

视觉控制入口：`variant`、`size`、`loading`、`checked`。状态组合以公开属性和行为规范为准，不为 Gallery 虚构额外状态。

### Theme Token

实际消费：`Theme.bg-control`、`Theme.bg-disabled`、`Theme.bg-layout`、`Theme.border-default`、`Theme.border-width`、`Theme.color-error`、`Theme.color-error-fill`、`Theme.color-error-fill-active`、`Theme.color-error-fill-hover`、`Theme.color-primary`、`Theme.color-primary-fill`、`Theme.color-primary-fill-active`、`Theme.color-primary-fill-hover`、`Theme.control-height-large`、`Theme.control-height-medium`、`Theme.control-height-small`、`Theme.control-padding-horizontal`、`Theme.fill-hover`、`Theme.fill-pressed`、`Theme.font-size-body`、`Theme.font-weight-medium`、`Theme.font-weight-regular`、`Theme.icon-default`、`Theme.icon-size-small`、`Theme.mode`、`Theme.radius-medium`、`Theme.selection-bg`、`Theme.space-2`、`Theme.text-disabled`、`Theme.text-on-accent`、`Theme.text-primary`。不得在调用方通过任意颜色绕过这些语义 Token。

### 内容、布局与环境

遵循全局排版、密度、长文本、100%–200% 缩放、浅色/深色/高对比度和 LTR/RTL 规则；组件特有差异见本页组成结构及行为规范。

## 行为规范

### 状态与交互

覆盖 Default、Hover、Pressed、Focused、Checked、Disabled 和 Loading；每次有效激活切换 checked，禁用和 Loading 阻止回调。

### 无障碍与本地化

使用可检查 button 语义并暴露 checked；无文字时必须提供本地化 `accessible-name`。

### 验证、宿主职责与限制

Gallery 展示选中和禁用；Harness 覆盖两次切换与禁用。它复用 Button，不提供三态。

全局约束：[`foundations.md`](../../foundations.md)、[`interaction.md`](../../interaction.md)、[`accessibility.md`](../../accessibility.md)、[`content-and-localization.md`](../../content-and-localization.md)。
