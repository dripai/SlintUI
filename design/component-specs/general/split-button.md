# SplitButton

成熟度：`Alpha`。源码：`crates/slint-ui/ui/controls/split-button.slint`。公开名称：`SplitButton`。

组合一个高频主操作和同组低频菜单；多个平级操作使用 ButtonGroup。

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

#### `PopupMenuEntry`

| 字段或值 | 类型/语义 |
|---|---|
| `text` | `string` |
| `shortcut` | `string` |
| `enabled` | `bool` |
| `checked` | `bool` |
| `separator` | `bool` |
| `has-submenu` | `bool` |

### 属性

| 名称 | 方向 | 类型 | 默认值 | 所有者 | 说明与边界 |
|---|---|---|---|---|---|
| `text` | `in` | `string` | `""` | 调用方 | 程序赋值不伪造用户编辑事件 |
| `variant` | `in` | `ButtonVariant` | `ButtonVariant.default` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `size` | `in` | `ControlSize` | `ControlSize.medium` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `enabled` | `in` | `bool` | `true` | 调用方 | false 时阻止用户操作；程序设置不触发用户事件 |
| `loading` | `in` | `bool` | `false` | 调用方 | true 时显示进行中状态并阻止重复操作 |
| `menu-entries` | `in` | `[PopupMenuEntry]` | `类型默认值` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `accessible-name` | `in` | `string` | `text` | 调用方 | 无可见文字的交互组件必须提供本地化名称 |
| `menu-accessible-name` | `in` | `string` | `""` | 调用方 | 取值范围、联动和非法值处理见行为规范 |

### 事件

| 名称 | 参数 | 触发语义与边界 |
|---|---|---|
| `clicked` | `无` | 一次被接受的用户激活完成后触发一次 |
| `menu-selected` | `index: int` | 由该组件定义的有效用户操作或等效公开方法触发；阻止条件见行为规范 |

### 公开方法

| 名称 | 参数 | 状态、事件与边界 |
|---|---|---|
| `activate()` | `无` | 当前实现约束：`if (enabled && !loading) { root.clicked(); }` |
| `activate-menu()` | `index: int` | 当前实现约束：`menu.activate(index);` |

## 视觉规范

### 组成结构

当前实现由 `HorizontalLayout`、`Button`、`PopupMenu` 组成。内部元素不是公开 API，调用方不得依赖其名称或坐标。

### 变体、尺寸与状态外观

视觉控制入口：`variant`、`size`、`loading`。状态组合以公开属性和行为规范为准，不为 Gallery 虚构额外状态。

### Theme Token

实际消费：`Theme.border-width`、`Theme.control-height-large`、`Theme.control-height-medium`、`Theme.control-height-small`。不得在调用方通过任意颜色绕过这些语义 Token。

### 内容、布局与环境

遵循全局排版、密度、长文本、100%–200% 缩放、浅色/深色/高对比度和 LTR/RTL 规则；组件特有差异见本页组成结构及行为规范。

## 行为规范

### 状态与交互

覆盖 Button 的默认、Hover、Pressed、Focused、Disabled、Loading，以及菜单打开和禁用条目。主操作与菜单回调分离。

### 无障碍与本地化

根为 groupbox，两个按钮分别命名；文本由调用方本地化，方向图标使用 SVG。

### 验证、宿主职责与限制

Gallery“通用 / SplitButton”页展示主操作与菜单组合；Harness 验证主操作、菜单和禁用项。复用 PopupMenu；暂不提供分裂按钮圆角融合和菜单侧自动翻转。

全局约束：[`foundations.md`](../../foundations.md)、[`interaction.md`](../../interaction.md)、[`accessibility.md`](../../accessibility.md)、[`content-and-localization.md`](../../content-and-localization.md)。
