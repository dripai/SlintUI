# List

成熟度：`Alpha`。源码：`crates/slint-ui/ui/data-display/list.slint`。公开名称：`List`。

用于单列可选择数据；层级数据使用 Tree，表格列数据使用 Table/DataGrid。

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
| `items` | `in` | `[ListItemData]` | `类型默认值` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `current-index` | `in-out` | `int` | `-1` | 共享受控状态 | -1 表示无选择；越界值的处理见行为规范 |
| `enabled` | `in` | `bool` | `true` | 调用方 | false 时阻止用户操作；程序设置不触发用户事件 |
| `accessible-name` | `in` | `string` | `""` | 调用方 | 无可见文字的交互组件必须提供本地化名称 |
| `empty-text` | `in` | `string` | `""` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `has-focus` | `out` | `bool` | `绑定：focus.has-focus` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |

### 事件

| 名称 | 参数 | 触发语义与边界 |
|---|---|---|
| `selected` | `index:int` | 用户选择有效且可用项目后触发一次 |

### 公开方法

| 名称 | 参数 | 状态、事件与边界 |
|---|---|---|
| `select()` | `index:int` | 当前实现约束：`if(enabled && index>=0 && index<items.length && items[index].enabled && current-index!=index){root.current-index=index;root.selected(index);}` |
| `select-next()` | `step: int` | 当前实现约束：`if (items.length > 0) { root.select(clamp(current-index + step, 0, items.length - 1)); }` |

## 视觉规范

### 组成结构

当前实现由 `NativeListView`、`ListItem`、`Text`、`FocusScope` 组成。内部元素不是公开 API，调用方不得依赖其名称或坐标。

### 变体、尺寸与状态外观

视觉控制入口：由内容和全局 Theme 决定。状态组合以公开属性和行为规范为准，不为 Gallery 虚构额外状态。

### Theme Token

实际消费：`Theme.bg-surface`、`Theme.border-default`、`Theme.border-width`、`Theme.card-min-width`、`Theme.empty-state-min-height`、`Theme.font-size-body`、`Theme.radius-medium`、`Theme.space-4`、`Theme.space-8`、`Theme.text-secondary`。不得在调用方通过任意颜色绕过这些语义 Token。

### 内容、布局与环境

遵循全局排版、密度、长文本、100%–200% 缩放、浅色/深色/高对比度和 LTR/RTL 规则；组件特有差异见本页组成结构及行为规范。

## 行为规范

### 状态与交互

覆盖 Default、Hover、Pressed、Focused、Selected、Disabled item、Empty、长文本和边界索引；同值与禁用项不回调。

### 无障碍与本地化

根使用 list 并暴露项数，条目使用 list-item。文字和说明由宿主本地化，源码顺序即朗读顺序。

### 验证、宿主职责与限制

Gallery 展示说明、选中和禁用项；Harness 覆盖正常、重复、禁用与越界。复用 Slint ListView 的重复项视口行为，不承诺可变高度海量数据性能。

全局约束：[`foundations.md`](../../foundations.md)、[`interaction.md`](../../interaction.md)、[`accessibility.md`](../../accessibility.md)、[`content-and-localization.md`](../../content-and-localization.md)。
