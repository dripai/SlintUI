# ContextMenu

成熟度：`Alpha`。源码：`crates/slint-ui/ui/navigation/context-menu.slint`。公开名称：`ContextMenu`。

在对象或指针位置打开上下文命令；必须保留按钮、菜单或键盘等价入口。

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
| `anchor-x` | `in` | `length` | `0px` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `anchor-y` | `in` | `length` | `0px` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `entries` | `in` | `[PopupMenuEntry]` | `类型默认值` | 调用方 | 取值范围、联动和非法值处理见行为规范；继承自 PopupMenu |
| `accessible-name` | `in` | `string` | `""` | 调用方 | 无可见文字的交互组件必须提供本地化名称；继承自 PopupMenu |
| `popup-width` | `in` | `length` | `max(self.width, Theme.popup-min-width)` | 调用方 | 取值范围、联动和非法值处理见行为规范；继承自 PopupMenu |
| `popup-height` | `in` | `length` | `min(Theme.popup-max-height, entries.length * Theme.row-height + Theme.space-2)` | 调用方 | 取值范围、联动和非法值处理见行为规范；继承自 PopupMenu |
| `open` | `out` | `bool` | `menu.is-open` | 组件派生 | 打开状态的所有权和关闭原因见行为规范；继承自 PopupMenu |
| `current-index` | `in-out` | `int` | `-1` | 共享受控状态 | -1 表示无选择；越界值的处理见行为规范；继承自 PopupMenu |

### 事件

| 名称 | 参数 | 触发语义与边界 |
|---|---|---|
| `selected` | `index: int` | 用户选择有效且可用项目后触发一次；继承自 PopupMenu |
| `submenu-requested` | `index: int` | 请求宿主执行操作，不表示操作已经成功；继承自 PopupMenu |
| `closed` | `无` | 组件完成关闭流程后触发；继承自 PopupMenu |

### 公开方法

| 名称 | 参数 | 状态、事件与边界 |
|---|---|---|
| `show()` | `无` | 当前实现约束：`menu.show();`；继承自 PopupMenu |
| `close()` | `无` | 当前实现约束：`menu.close();`；继承自 PopupMenu |
| `activate()` | `index: int` | 当前实现约束：`if (index >= 0 && index < entries.length && entries[index].enabled && !entries[index].separator) { if (entries[index].has-submenu) { root.submenu-requested(index); } else { root...`；继承自 PopupMenu |

## 视觉规范

### 组成结构

当前实现由 `PopupWindow`、`Rectangle`、`ScrollView`、`VerticalLayout`、`PopupMenuItem`、`FocusScope` 组成。内部元素不是公开 API，调用方不得依赖其名称或坐标。

### 变体、尺寸与状态外观

视觉控制入口：由内容和全局 Theme 决定。状态组合以公开属性和行为规范为准，不为 Gallery 虚构额外状态。

### Theme Token

实际消费：`Theme.bg-elevated`、`Theme.border-default`、`Theme.border-width`、`Theme.popup-max-height`、`Theme.popup-min-width`、`Theme.radius-large`、`Theme.row-height`、`Theme.space-1`、`Theme.space-2`。不得在调用方通过任意颜色绕过这些语义 Token。

### 内容、布局与环境

遵循全局排版、密度、长文本、100%–200% 缩放、浅色/深色/高对比度和 LTR/RTL 规则；组件特有差异见本页组成结构及行为规范。

## 行为规范

### 状态与交互

打开、选择、关闭和键盘行为与 PopupMenu 一致；调用方在右键或 Shift+F10 时更新锚点并调用 `show()`。

### 无障碍与本地化

沿用 PopupMenu 语义；右键不能成为唯一入口，RTL 下锚点仍使用窗口坐标。

### 验证、宿主职责与限制

Gallery 与 PopupMenu 共用场景，编译和冒烟覆盖。多显示器工作区翻转由原生 PopupWindow 负责，焦点恢复目标由宿主记录。

全局约束：[`foundations.md`](../../foundations.md)、[`interaction.md`](../../interaction.md)、[`accessibility.md`](../../accessibility.md)、[`content-and-localization.md`](../../content-and-localization.md)。
