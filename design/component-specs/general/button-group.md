# ButtonGroup

成熟度：`Alpha`。源码：`crates/slint-ui/ui/controls/button-group.slint`。公开名称：`ButtonGroup`。

用于并列且层级相同的一组操作或模式；单个开关操作使用 ToggleButton，少量互斥视图也可使用 SegmentedControl。

## 公开 API

### 数据类型与枚举

#### `ButtonGroupItem`

| 字段或值 | 类型/语义 |
|---|---|
| `text` | `string` |
| `enabled` | `bool` |
| `checked` | `bool` |

#### `ControlSize`

| 字段或值 | 类型/语义 |
|---|---|
| `small` | `枚举值` |
| `medium` | `枚举值` |
| `large` | `枚举值` |

### 属性

| 名称 | 方向 | 类型 | 默认值 | 所有者 | 说明与边界 |
|---|---|---|---|---|---|
| `items` | `in` | `[ButtonGroupItem]` | `类型默认值` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `current-index` | `in-out` | `int` | `-1` | 共享受控状态 | -1 表示无选择；越界值的处理见行为规范 |
| `enabled` | `in` | `bool` | `true` | 调用方 | false 时阻止用户操作；程序设置不触发用户事件 |
| `size` | `in` | `ControlSize` | `ControlSize.medium` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `accessible-name` | `in` | `string` | `""` | 调用方 | 无可见文字的交互组件必须提供本地化名称 |

### 事件

| 名称 | 参数 | 触发语义与边界 |
|---|---|---|
| `selected` | `index: int` | 用户选择有效且可用项目后触发一次 |

### 公开方法

| 名称 | 参数 | 状态、事件与边界 |
|---|---|---|
| `select()` | `index: int` | 当前实现约束：`if (enabled && index >= 0 && index < items.length && items[index].enabled && current-index != index) { root.current-index = index; root.selected(index); }` |
| `select-next()` | `step: int` | 当前实现约束：`if (items.length > 0) { root.select(Math.mod(current-index + step + items.length, items.length)); }` |

## 视觉规范

### 组成结构

当前实现由 `HorizontalLayout`、`Button` 组成。内部元素不是公开 API，调用方不得依赖其名称或坐标。

### 变体、尺寸与状态外观

视觉控制入口：`size`。状态组合以公开属性和行为规范为准，不为 Gallery 虚构额外状态。

### Theme Token

实际消费：`Theme.bg-control`、`Theme.border-default`、`Theme.border-width`、`Theme.control-height-large`、`Theme.control-height-medium`、`Theme.control-height-small`、`Theme.radius-medium`。不得在调用方通过任意颜色绕过这些语义 Token。

### 内容、布局与环境

遵循全局排版、密度、长文本、100%–200% 缩放、浅色/深色/高对比度和 LTR/RTL 规则；组件特有差异见本页组成结构及行为规范。

## 行为规范

### 状态与交互

覆盖默认、Hover、Pressed、Focused、Selected 和 Disabled；同值、禁用项及越界索引不回调。按钮仍可独立 Tab 聚焦，方向键接入由宿主调用 `select-next()`。

### 无障碍与本地化

根使用 groupbox 角色并暴露项数；条目继承 Button 语义。长文本可省略，调用方提供本地化条目文本。

### 验证、宿主职责与限制

Gallery“通用 / ButtonGroup”展示选中和整组禁用；Harness 覆盖正常、重复、禁用项和越界。Slint 1.17.1 无法枚举任意 `@children` 做通用 roving focus，因此采用显式模型。

全局约束：[`foundations.md`](../../foundations.md)、[`interaction.md`](../../interaction.md)、[`accessibility.md`](../../accessibility.md)、[`content-and-localization.md`](../../content-and-localization.md)。
