# ListItem

成熟度：`Alpha`。源码：`crates/slint-ui/ui/data-display/list.slint`。公开名称：`ListItem`。

呈现 List 中的单条记录及其可选操作。仅用于行级展示和激活；复杂表格、分栏排序或单元格编辑应使用 Table/DataGrid。

## 公开 API

### 数据类型与枚举

#### `ListItemData`

| 字段或值 | 类型/语义 |
|---|---|
| `text` | `string` |
| `description` | `string` |
| `enabled` | `bool` |

### 属性

| 名称 | 方向 | 类型 | 默认值 | 所有者 | 说明与边界 |
|---|---|---|---|---|---|
| `item` | `in` | `ListItemData` | `类型默认值` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `selected` | `in` | `bool` | `false` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `focused` | `in` | `bool` | `false` | 调用方 | 取值范围、联动和非法值处理见行为规范 |

### 事件

| 名称 | 参数 | 触发语义与边界 |
|---|---|---|
| `activated` | `无` | 由该组件定义的有效用户操作或等效公开方法触发；阻止条件见行为规范 |

## 视觉规范

### 组成结构

当前实现由 `VerticalLayout`、`Text`、`TouchArea`、`FocusRing` 组成。内部元素不是公开 API，调用方不得依赖其名称或坐标。

### 变体、尺寸与状态外观

视觉控制入口：`selected`。状态组合以公开属性和行为规范为准，不为 Gallery 虚构额外状态。

### Theme Token

实际消费：`Theme.fill-hover`、`Theme.fill-pressed`、`Theme.font-size-body`、`Theme.font-size-caption`、`Theme.radius-small`、`Theme.row-height`、`Theme.selection-bg`、`Theme.space-1`、`Theme.space-2`、`Theme.space-3`、`Theme.text-disabled`、`Theme.text-primary`、`Theme.text-secondary`。不得在调用方通过任意颜色绕过这些语义 Token。

### 内容、布局与环境

遵循全局排版、密度、长文本、100%–200% 缩放、浅色/深色/高对比度和 LTR/RTL 规则；组件特有差异见本页组成结构及行为规范。

## 行为规范

### 状态与交互

默认、Hover、Focused、Selected 和 Disabled 由 selected、enabled 与焦点状态决定。activate() 只在 enabled 时发出 clicked()。

### 无障碍与本地化

使用 list-item 角色并暴露名称、可用和选中状态；标题和说明由宿主提供本地化文本。

### 验证、宿主职责与限制

ListItem 不持有集合索引，也不直接修改 List 模型；宿主或 List 负责将激活映射到稳定数据项。

全局约束：[`foundations.md`](../../foundations.md)、[`interaction.md`](../../interaction.md)、[`accessibility.md`](../../accessibility.md)、[`content-and-localization.md`](../../content-and-localization.md)。
