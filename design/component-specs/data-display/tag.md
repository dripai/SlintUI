# Tag

成熟度：`Alpha`。源码：`crates/slint-ui/ui/data-display/tag.slint`。公开名称：`Tag`。

用于短标签、状态、可选择筛选或可关闭条目；计数使用 Badge，复杂对象使用 Card/ListItem。

## 公开 API

### 数据类型与枚举

#### `TagTone`

| 字段或值 | 类型/语义 |
|---|---|
| `neutral` | `枚举值` |
| `info` | `枚举值` |
| `success` | `枚举值` |
| `warning` | `枚举值` |
| `error` | `枚举值` |

### 属性

| 名称 | 方向 | 类型 | 默认值 | 所有者 | 说明与边界 |
|---|---|---|---|---|---|
| `text` | `in` | `string` | `""` | 调用方 | 程序赋值不伪造用户编辑事件 |
| `tone` | `in` | `TagTone` | `TagTone.neutral` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `closable` | `in` | `bool` | `false` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `selectable` | `in` | `bool` | `false` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `selected` | `in-out` | `bool` | `false` | 共享受控状态 | 取值范围、联动和非法值处理见行为规范 |
| `enabled` | `in` | `bool` | `true` | 调用方 | false 时阻止用户操作；程序设置不触发用户事件 |
| `accessible-name` | `in` | `string` | `text` | 调用方 | 无可见文字的交互组件必须提供本地化名称 |
| `has-focus` | `out` | `bool` | `绑定：focus.has-focus` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |

### 事件

| 名称 | 参数 | 触发语义与边界 |
|---|---|---|
| `toggled` | `selected: bool` | 由该组件定义的有效用户操作或等效公开方法触发；阻止条件见行为规范 |
| `close-requested` | `无` | 请求宿主执行操作，不表示操作已经成功 |

### 公开方法

| 名称 | 参数 | 状态、事件与边界 |
|---|---|---|
| `toggle()` | `无` | 当前实现约束：`if (enabled && selectable) { root.selected = !root.selected; root.toggled(root.selected); }` |
| `close()` | `无` | 当前实现约束：`if (enabled && closable) { root.close-requested(); }` |

## 视觉规范

### 组成结构

当前实现由 `HorizontalLayout`、`Text`、`Rectangle`、`Image`、`TouchArea`、`FocusScope`、`FocusRing` 组成。内部元素不是公开 API，调用方不得依赖其名称或坐标。

### 变体、尺寸与状态外观

视觉控制入口：`tone`、`selected`。状态组合以公开属性和行为规范为准，不为 Gallery 虚构额外状态。

### Theme Token

实际消费：`Theme.bg-control`、`Theme.bg-disabled`、`Theme.border-default`、`Theme.border-width`、`Theme.color-error-bg`、`Theme.color-error-border`、`Theme.color-primary-bg`、`Theme.color-primary-border`、`Theme.color-success-bg`、`Theme.color-success-border`、`Theme.color-warning-bg`、`Theme.color-warning-border`、`Theme.fill-hover`、`Theme.font-size-caption`、`Theme.icon-default`、`Theme.icon-size-small`、`Theme.icon-size-x-small`、`Theme.radius-small`、`Theme.selection-bg`、`Theme.space-1`、`Theme.space-2`、`Theme.tag-height`、`Theme.text-disabled`、`Theme.text-primary`。不得在调用方通过任意颜色绕过这些语义 Token。

### 内容、布局与环境

遵循全局排版、密度、长文本、100%–200% 缩放、浅色/深色/高对比度和 LTR/RTL 规则；组件特有差异见本页组成结构及行为规范。

## 行为规范

### 状态与交互

覆盖 Default、Hover、Focused、Selected、Closable、Disabled 和五种 tone；禁用阻止切换/关闭，每次有效切换显式回调。

### 无障碍与本地化

使用可检查 button 语义；关闭按钮单独可访问。标签必须简短，本地化长文本应改用其他组件。

### 验证、宿主职责与限制

Gallery 展示 tone、选择、关闭和禁用；Harness 覆盖重复切换、关闭与禁用。关闭只发请求，不直接修改外部模型。

全局约束：[`foundations.md`](../../foundations.md)、[`interaction.md`](../../interaction.md)、[`accessibility.md`](../../accessibility.md)、[`content-and-localization.md`](../../content-and-localization.md)。
