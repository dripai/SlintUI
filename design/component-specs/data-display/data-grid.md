# DataGrid

成熟度：`Alpha`。源码：`crates/slint-ui/ui/data-display/data-grid.slint`。公开名称：`DataGrid`。

承载大数据表格的宿主窗口化、排序、选择、列宽和编辑请求；普通只读数据使用 Table。

## 公开 API

### 属性

| 名称 | 方向 | 类型 | 默认值 | 所有者 | 说明与边界 |
|---|---|---|---|---|---|
| `rows` | `in` | `[[StandardListViewItem]]` | `类型默认值` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `columns` | `in-out` | `[TableColumn]` | `类型默认值` | 共享受控状态 | 取值范围、联动和非法值处理见行为规范 |
| `row-offset` | `in` | `int` | `0` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `total-row-count` | `in` | `int` | `rows.length` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `current-row` | `in-out` | `int` | `-1` | 共享受控状态 | 取值范围、联动和非法值处理见行为规范 |
| `enabled` | `in` | `bool` | `true` | 调用方 | false 时阻止用户操作；程序设置不触发用户事件 |
| `editable` | `in` | `bool` | `false` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `accessible-name` | `in` | `string` | `""` | 调用方 | 无可见文字的交互组件必须提供本地化名称 |
| `empty-text` | `in` | `string` | `""` | 调用方 | 取值范围、联动和非法值处理见行为规范 |

### 事件

| 名称 | 参数 | 触发语义与边界 |
|---|---|---|
| `sort-ascending` | `column: int` | 由该组件定义的有效用户操作或等效公开方法触发；阻止条件见行为规范 |
| `sort-descending` | `column: int` | 由该组件定义的有效用户操作或等效公开方法触发；阻止条件见行为规范 |
| `current-row-changed` | `global-row: int` | 由该组件定义的有效用户操作或等效公开方法触发；阻止条件见行为规范 |
| `edit-requested` | `global-row: int, column: int` | 请求宿主执行操作，不表示操作已经成功 |
| `range-requested` | `start: int, count: int` | 请求宿主执行操作，不表示操作已经成功 |
| `row-pointer-event` | `global-row: int, event: PointerEvent, position: Point` | 由该组件定义的有效用户操作或等效公开方法触发；阻止条件见行为规范 |

### 公开方法

| 名称 | 参数 | 状态、事件与边界 |
|---|---|---|
| `request-range()` | `start: int, count: int` | 当前实现约束：`if (start >= 0 && count > 0 && start < total-row-count) { root.range-requested(start, min(count, total-row-count - start)); }` |
| `select-row()` | `global-row: int` | 当前实现约束：`let local = global-row - row-offset; if (enabled && local >= 0 && local < rows.length) { native.set-current-row(local); }` |
| `request-edit()` | `global-row: int, column: int` | 当前实现约束：`if (enabled && editable && global-row >= 0 && global-row < total-row-count && column >= 0 && column < columns.length) { root.edit-requested(global-row, column); }` |

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

复用 StandardTableView。局部行映射为全局索引，编辑和数据窗口均显式请求宿主。

### 无障碍与本地化

使用 table 角色并声明完整行数；单元格格式化由数据层按 Locale 完成。

### 验证、宿主职责与限制

Gallery 展示 2048 行宿主窗口；Harness 验证范围截断和编辑边界。当前不内建数据缓存、编辑器、固定列或可变行高；虚拟化是宿主窗口化协议。

全局约束：[`foundations.md`](../../foundations.md)、[`interaction.md`](../../interaction.md)、[`accessibility.md`](../../accessibility.md)、[`content-and-localization.md`](../../content-and-localization.md)。
