# Calendar

成熟度：`Alpha`。源码：`crates/slint-ui/ui/data-display/calendar.slint`。公开名称：`Calendar`。

展示月视图并选择单日；排期、范围选择和日程数据由更高层组合。

## 公开 API

### 数据类型与枚举

#### `CalendarDay`

| 字段或值 | 类型/语义 |
|---|---|
| `label` | `string` |
| `accessible-name` | `string` |
| `in-current-month` | `bool` |
| `selected` | `bool` |
| `today` | `bool` |
| `enabled` | `bool` |

### 属性

| 名称 | 方向 | 类型 | 默认值 | 所有者 | 说明与边界 |
|---|---|---|---|---|---|
| `weekday-labels` | `in` | `[string]` | `类型默认值` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `days` | `in` | `[CalendarDay]` | `类型默认值` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `title` | `in` | `string` | `""` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `accessible-name` | `in` | `string` | `title` | 调用方 | 无可见文字的交互组件必须提供本地化名称 |
| `previous-accessible-name` | `in` | `string` | `""` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `next-accessible-name` | `in` | `string` | `""` | 调用方 | 取值范围、联动和非法值处理见行为规范 |

### 事件

| 名称 | 参数 | 触发语义与边界 |
|---|---|---|
| `selected` | `index: int` | 用户选择有效且可用项目后触发一次 |
| `previous-requested` | `无` | 请求宿主执行操作，不表示操作已经成功 |
| `next-requested` | `无` | 请求宿主执行操作，不表示操作已经成功 |

### 公开方法

| 名称 | 参数 | 状态、事件与边界 |
|---|---|---|
| `select()` | `index: int` | 当前实现约束：`if (index >= 0 && index < days.length && days[index].enabled) { root.selected(index); }` |

## 视觉规范

### 组成结构

当前实现由 `VerticalLayout`、`HorizontalLayout`、`Rectangle`、`Image`、`TouchArea`、`Text` 组成。内部元素不是公开 API，调用方不得依赖其名称或坐标。

### 变体、尺寸与状态外观

视觉控制入口：由内容和全局 Theme 决定。状态组合以公开属性和行为规范为准，不为 Gallery 虚构额外状态。

### Theme Token

实际消费：`Theme.bg-surface`、`Theme.border-default`、`Theme.border-width`、`Theme.calendar-cell-size`、`Theme.color-primary`、`Theme.color-primary-fill`、`Theme.control-height-medium`、`Theme.control-height-small`、`Theme.fill-hover`、`Theme.font-size-body`、`Theme.font-size-caption`、`Theme.font-weight-medium`、`Theme.icon-default`、`Theme.icon-size-small`、`Theme.radius-medium`、`Theme.space-2`、`Theme.space-4`、`Theme.text-disabled`、`Theme.text-on-accent`、`Theme.text-primary`、`Theme.text-secondary`、`Theme.text-tertiary`。不得在调用方通过任意颜色绕过这些语义 Token。

### 内容、布局与环境

遵循全局排版、密度、长文本、100%–200% 缩放、浅色/深色/高对比度和 LTR/RTL 规则；组件特有差异见本页组成结构及行为规范。

## 行为规范

### 状态与交互

覆盖当前月、非当前月、Today、Selected、Hover、Disabled；日期模型完全受控。

### 无障碍与本地化

根为 groupbox，每日提供完整 accessible-name；周起始日、标题和数字由宿主按 Locale/历法生成。

### 验证、宿主职责与限制

Gallery 与 Harness 覆盖选择和禁用。组件不计算 42 格日期、农历、范围选择或键盘二维漫游，当前固定七列六行几何。

全局约束：[`foundations.md`](../../foundations.md)、[`interaction.md`](../../interaction.md)、[`accessibility.md`](../../accessibility.md)、[`content-and-localization.md`](../../content-and-localization.md)。
