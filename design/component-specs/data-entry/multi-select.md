# MultiSelect

成熟度：`Alpha`。源码：`crates/slint-ui/ui/controls/multi-select.slint`。公开名称：`MultiSelect`。

从有限候选集中选择多个值；复杂实体搜索由 AutoComplete 与业务数据层组合。

## 公开 API

### 数据类型与枚举

#### `ControlSize`

| 字段或值 | 类型/语义 |
|---|---|
| `small` | `枚举值` |
| `medium` | `枚举值` |
| `large` | `枚举值` |

#### `MultiSelectOption`

| 字段或值 | 类型/语义 |
|---|---|
| `text` | `string` |
| `selected` | `bool` |
| `enabled` | `bool` |

### 属性

| 名称 | 方向 | 类型 | 默认值 | 所有者 | 说明与边界 |
|---|---|---|---|---|---|
| `options` | `in` | `[MultiSelectOption]` | `类型默认值` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `selected-labels` | `in` | `[string]` | `类型默认值` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `selected-summary` | `in` | `string` | `""` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `placeholder-text` | `in` | `string` | `""` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `accessible-name` | `in` | `string` | `""` | 调用方 | 无可见文字的交互组件必须提供本地化名称 |
| `size` | `in` | `ControlSize` | `ControlSize.medium` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `enabled` | `in` | `bool` | `true` | 调用方 | false 时阻止用户操作；程序设置不触发用户事件 |
| `open` | `out` | `bool` | `popup.is-open` | 组件派生 | 打开状态的所有权和关闭原因见行为规范 |

### 事件

| 名称 | 参数 | 触发语义与边界 |
|---|---|---|
| `selection-requested` | `index: int, selected: bool` | 请求宿主执行操作，不表示操作已经成功 |

### 公开方法

| 名称 | 参数 | 状态、事件与边界 |
|---|---|---|
| `show()` | `无` | 当前实现约束：`if (enabled && options.length > 0) { popup.show(); }` |
| `close()` | `无` | 当前实现约束：`popup.close();` |
| `request-selection()` | `index: int, selected: bool` | 当前实现约束：`if (enabled && index >= 0 && index < options.length && options[index].enabled && options[index].selected != selected) { root.selection-requested(index, selected); }` |

## 视觉规范

### 组成结构

当前实现由 `HorizontalLayout`、`Text`、`Image`、`TouchArea`、`PopupWindow`、`Rectangle`、`VerticalLayout` 组成。内部元素不是公开 API，调用方不得依赖其名称或坐标。

### 变体、尺寸与状态外观

视觉控制入口：`selected-labels`、`selected-summary`、`size`。状态组合以公开属性和行为规范为准，不为 Gallery 虚构额外状态。

### Theme Token

实际消费：`Theme.bg-control`、`Theme.bg-disabled`、`Theme.bg-elevated`、`Theme.border-default`、`Theme.border-width`、`Theme.color-primary`、`Theme.control-height-large`、`Theme.control-height-medium`、`Theme.control-height-small`、`Theme.control-min-width`、`Theme.control-padding-horizontal`、`Theme.fill-hover`、`Theme.focus-outline`、`Theme.font-size-body`、`Theme.icon-default`、`Theme.icon-size-small`、`Theme.icon-size-x-small`、`Theme.popup-max-height`、`Theme.popup-min-width`、`Theme.radius-large`、`Theme.radius-medium`、`Theme.row-height`、`Theme.space-1`、`Theme.space-2`、`Theme.space-3`、`Theme.text-disabled`、`Theme.text-placeholder`、`Theme.text-primary`。不得在调用方通过任意颜色绕过这些语义 Token。

### 内容、布局与环境

遵循全局排版、密度、长文本、100%–200% 缩放、浅色/深色/高对比度和 LTR/RTL 规则；组件特有差异见本页组成结构及行为规范。

## 行为规范

### 状态与交互

覆盖默认、打开、Hover、Checked、Disabled；模型由宿主更新，不静默修改数据。

### 无障碍与本地化

根为 combobox，选项暴露 checked；摘要和文案由调用方按 Locale 生成。

### 验证、宿主职责与限制

Gallery 和 Harness 覆盖多选与禁用。当前以摘要显示已选项，不绘制独立 Tag、不内建搜索和 Tag 溢出计算。

全局约束：[`foundations.md`](../../foundations.md)、[`interaction.md`](../../interaction.md)、[`accessibility.md`](../../accessibility.md)、[`content-and-localization.md`](../../content-and-localization.md)。
