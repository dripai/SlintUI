# Table

成熟度：`Alpha`。源码：`crates/slint-ui/ui/data-display/table.slint`。公开名称：`Table`。

展示可排序、单选的规则二维数据；大数据虚拟化、编辑和复杂列能力属于 DataGrid。

## 公开 API

### 属性

| 名称 | 方向 | 类型 | 默认值 | 所有者 | 说明与边界 |
|---|---|---|---|---|---|
| `rows` | `in` | `[[StandardListViewItem]]` | `类型默认值` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `columns` | `in-out` | `[TableColumn]` | `类型默认值` | 共享受控状态 | 取值范围、联动和非法值处理见行为规范 |
| `current-row` | `in-out` | `int` | `-1` | 共享受控状态 | 取值范围、联动和非法值处理见行为规范 |
| `enabled` | `in` | `bool` | `true` | 调用方 | false 时阻止用户操作；程序设置不触发用户事件 |
| `accessible-name` | `in` | `string` | `""` | 调用方 | 无可见文字的交互组件必须提供本地化名称 |
| `empty-text` | `in` | `string` | `""` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `has-focus` | `out` | `bool` | `绑定：native.has-focus` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |

### 事件

| 名称 | 参数 | 触发语义与边界 |
|---|---|---|
| `sort-ascending` | `column: int` | 由该组件定义的有效用户操作或等效公开方法触发；阻止条件见行为规范 |
| `sort-descending` | `column: int` | 由该组件定义的有效用户操作或等效公开方法触发；阻止条件见行为规范 |
| `current-row-changed` | `row: int` | 由该组件定义的有效用户操作或等效公开方法触发；阻止条件见行为规范 |
| `row-pointer-event` | `row: int, event: PointerEvent, position: Point` | 由该组件定义的有效用户操作或等效公开方法触发；阻止条件见行为规范 |

### 公开方法

| 名称 | 参数 | 状态、事件与边界 |
|---|---|---|
| `select-row()` | `index: int` | 当前实现约束：`if (enabled && index >= 0 && index < rows.length && current-row != index) { native.set-current-row(index); }` |

## 视觉规范

### 组成结构

当前实现由 `NativeTableView`、`Rectangle`、`Text` 组成。内部元素不是公开 API，调用方不得依赖其名称或坐标。

### 变体、尺寸与状态外观

视觉控制入口：由内容和全局 Theme 决定。状态组合以公开属性和行为规范为准，不为 Gallery 虚构额外状态。

### Theme Token

实际消费：`Theme.bg-surface`、`Theme.border-default`、`Theme.border-width`、`Theme.card-min-width`、`Theme.empty-state-min-height`、`Theme.font-size-body`、`Theme.radius-medium`、`Theme.space-4`、`Theme.space-8`、`Theme.text-secondary`。不得在调用方通过任意颜色绕过这些语义 Token。

### 内容、布局与环境

遵循全局排版、密度、长文本、100%–200% 缩放、浅色/深色/高对比度和 LTR/RTL 规则；组件特有差异见本页组成结构及行为规范。

## 行为规范

### 状态与交互

复用 StandardTableView 的表头排序、行选择、键盘焦点和滚动；空模型显示明确空状态，同值和越界选择不产生额外状态。

### 无障碍与本地化

Role 为 table，暴露名称、可用状态和项数。数据格式、数值对齐、空值文案由宿主按 Locale 提供。

### 验证、宿主职责与限制

Gallery“数据展示 / Table”页覆盖双列数据。原生控件样式在编译期选择，当前不承诺固定列、虚拟化、自定义单元格或编辑。

全局约束：[`foundations.md`](../../foundations.md)、[`interaction.md`](../../interaction.md)、[`accessibility.md`](../../accessibility.md)、[`content-and-localization.md`](../../content-and-localization.md)。
