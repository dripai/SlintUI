# PopupMenuItem

成熟度：`Alpha`。源码：`crates/slint-ui/ui/navigation/popup-menu.slint`。公开名称：`PopupMenuItem`。

呈现 PopupMenu 中的一条命令、分隔项或可勾选项。它不独立负责弹层定位和关闭生命周期。

## 公开 API

### 数据类型与枚举

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
| `entry` | `in` | `PopupMenuEntry` | `类型默认值` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `accessible-name` | `in` | `string` | `entry.text` | 调用方 | 无可见文字的交互组件必须提供本地化名称 |
| `highlighted` | `in` | `bool` | `false` | 调用方 | 取值范围、联动和非法值处理见行为规范 |

### 事件

| 名称 | 参数 | 触发语义与边界 |
|---|---|---|
| `triggered` | `无` | 由该组件定义的有效用户操作或等效公开方法触发；阻止条件见行为规范 |
| `submenu-requested` | `无` | 请求宿主执行操作，不表示操作已经成功 |

### 公开方法

| 名称 | 参数 | 状态、事件与边界 |
|---|---|---|
| `activate()` | `无` | 当前实现约束：`if (entry.enabled && !entry.separator) { if (entry.has-submenu) { root.submenu-requested(); } else { root.triggered(); } }` |

## 视觉规范

### 组成结构

当前实现由 `Rectangle`、`HorizontalLayout`、`Image`、`Text`、`TouchArea` 组成。内部元素不是公开 API，调用方不得依赖其名称或坐标。

### 变体、尺寸与状态外观

视觉控制入口：由内容和全局 Theme 决定。状态组合以公开属性和行为规范为准，不为 Gallery 虚构额外状态。

### Theme Token

实际消费：`Theme.border-secondary`、`Theme.border-width`、`Theme.color-primary`、`Theme.fill-hover`、`Theme.font-size-body`、`Theme.font-size-caption`、`Theme.icon-default`、`Theme.icon-size-small`、`Theme.icon-size-x-small`、`Theme.popup-min-width`、`Theme.row-height`、`Theme.space-2`、`Theme.space-3`、`Theme.space-4`、`Theme.text-disabled`、`Theme.text-primary`、`Theme.text-tertiary`。不得在调用方通过任意颜色绕过这些语义 Token。

### 内容、布局与环境

遵循全局排版、密度、长文本、100%–200% 缩放、浅色/深色/高对比度和 LTR/RTL 规则；组件特有差异见本页组成结构及行为规范。

## 行为规范

### 状态与交互

默认、Hover、Focused、Checked 和 Disabled 由条目数据与焦点状态决定。activate() 只对可见、可用且非分隔项发出 triggered()。

### 无障碍与本地化

使用 menu-item 语义，按条目类型暴露名称、可用和选中状态；快捷键文字仅作提示，实际快捷键由宿主注册。

### 验证、宿主职责与限制

选择后的关闭、焦点恢复及命令执行结果由 PopupMenu 和宿主协议负责。

全局约束：[`foundations.md`](../../foundations.md)、[`interaction.md`](../../interaction.md)、[`accessibility.md`](../../accessibility.md)、[`content-and-localization.md`](../../content-and-localization.md)。
