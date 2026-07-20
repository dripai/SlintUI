# Dropdown

成熟度：`Alpha`。源码：`crates/slint-ui/ui/navigation/dropdown.slint`。公开名称：`Dropdown`。

用于按钮触发的一组菜单命令；单值表单选择使用 Select，可编辑建议使用 ComboBox。

## 公开 API

### 数据类型与枚举

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
| `entries` | `in` | `[PopupMenuEntry]` | `类型默认值` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `enabled` | `in` | `bool` | `true` | 调用方 | false 时阻止用户操作；程序设置不触发用户事件 |
| `size` | `in` | `ControlSize` | `ControlSize.medium` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `accessible-name` | `in` | `string` | `text` | 调用方 | 无可见文字的交互组件必须提供本地化名称 |
| `open` | `out` | `bool` | `menu.open` | 组件派生 | 打开状态的所有权和关闭原因见行为规范 |

### 事件

| 名称 | 参数 | 触发语义与边界 |
|---|---|---|
| `selected` | `index: int` | 用户选择有效且可用项目后触发一次 |
| `closed` | `无` | 组件完成关闭流程后触发 |

### 公开方法

| 名称 | 参数 | 状态、事件与边界 |
|---|---|---|
| `toggle()` | `无` | 当前实现约束：`if (!enabled) { return; } if (open) { menu.close(); } else { menu.show(); }` |
| `activate()` | `index: int` | 当前实现约束：`if (enabled) { menu.activate(index); }` |

## 视觉规范

### 组成结构

当前实现由 `Button`、`PopupMenu` 组成。内部元素不是公开 API，调用方不得依赖其名称或坐标。

### 变体、尺寸与状态外观

视觉控制入口：`size`。状态组合以公开属性和行为规范为准，不为 Gallery 虚构额外状态。

### Theme Token

实际消费：`Theme.control-height-large`、`Theme.control-height-medium`、`Theme.control-height-small`、`Theme.control-min-width`。不得在调用方通过任意颜色绕过这些语义 Token。

### 内容、布局与环境

遵循全局排版、密度、长文本、100%–200% 缩放、浅色/深色/高对比度和 LTR/RTL 规则；组件特有差异见本页组成结构及行为规范。

## 行为规范

### 状态与交互

覆盖 Closed、Open、Hover、Pressed、Focused、Disabled、禁用菜单项、点击外部和 Escape；越界或禁用项不回调。

### 无障碍与本地化

触发器暴露 expandable/expanded，菜单复用 PopupMenu 的项语义与键盘行为。快捷键文字由宿主本地化。

### 验证、宿主职责与限制

Gallery“导航 / Dropdown”展示正常、勾选和禁用项；Harness 覆盖正常、禁用和越界。弹层定位与关闭策略复用 Slint PopupWindow。

全局约束：[`foundations.md`](../../foundations.md)、[`interaction.md`](../../interaction.md)、[`accessibility.md`](../../accessibility.md)、[`content-and-localization.md`](../../content-and-localization.md)。
