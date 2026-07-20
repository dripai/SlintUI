# ColorPicker

成熟度：`Alpha`。源码：`crates/slint-ui/ui/controls/color-picker.slint`。公开名称：`ColorPicker`。

从预定义语义色板选择颜色；复杂设计工具使用专用编辑器。

## 公开 API

### 数据类型与枚举

#### `ColorOption`

| 字段或值 | 类型/语义 |
|---|---|
| `name` | `string` |
| `value` | `color` |
| `enabled` | `bool` |

### 属性

| 名称 | 方向 | 类型 | 默认值 | 所有者 | 说明与边界 |
|---|---|---|---|---|---|
| `value` | `in-out` | `color` | `transparent` | 共享受控状态 | 取值范围、联动和非法值处理见行为规范 |
| `options` | `in` | `[ColorOption]` | `类型默认值` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `value-label` | `in` | `string` | `""` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `accessible-name` | `in` | `string` | `""` | 调用方 | 无可见文字的交互组件必须提供本地化名称 |
| `enabled` | `in` | `bool` | `true` | 调用方 | false 时阻止用户操作；程序设置不触发用户事件 |
| `open` | `out` | `bool` | `popup.is-open` | 组件派生 | 打开状态的所有权和关闭原因见行为规范 |

### 事件

| 名称 | 参数 | 触发语义与边界 |
|---|---|---|
| `changed` | `index: int, value: color` | 组件接受一个不同的新值后触发一次 |

### 公开方法

| 名称 | 参数 | 状态、事件与边界 |
|---|---|---|
| `show()` | `无` | 当前实现约束：`if (enabled && options.length > 0) { popup.show(); }` |
| `close()` | `无` | 当前实现约束：`popup.close();` |
| `select()` | `index: int` | 当前实现约束：`if (index >= 0 && index < options.length && options[index].enabled) { root.value = options[index].value; root.changed(index, root.value); root.close(); }` |

## 视觉规范

### 组成结构

当前实现由 `HorizontalLayout`、`Rectangle`、`Text`、`Image`、`TouchArea`、`PopupWindow`、`VerticalLayout` 组成。内部元素不是公开 API，调用方不得依赖其名称或坐标。

### 变体、尺寸与状态外观

视觉控制入口：由内容和全局 Theme 决定。状态组合以公开属性和行为规范为准，不为 Gallery 虚构额外状态。

### Theme Token

实际消费：`Theme.bg-control`、`Theme.bg-disabled`、`Theme.bg-elevated`、`Theme.border-default`、`Theme.border-width`、`Theme.control-height-medium`、`Theme.control-min-width`、`Theme.control-padding-horizontal`、`Theme.fill-hover`、`Theme.focus-outline`、`Theme.font-size-body`、`Theme.icon-default`、`Theme.icon-size-medium`、`Theme.icon-size-x-small`、`Theme.popup-min-width`、`Theme.radius-large`、`Theme.radius-medium`、`Theme.radius-small`、`Theme.row-height`、`Theme.space-1`、`Theme.space-2`、`Theme.space-3`、`Theme.text-disabled`、`Theme.text-primary`。不得在调用方通过任意颜色绕过这些语义 Token。

### 内容、布局与环境

遵循全局排版、密度、长文本、100%–200% 缩放、浅色/深色/高对比度和 LTR/RTL 规则；组件特有差异见本页组成结构及行为规范。

## 行为规范

### 状态与交互

覆盖色块、打开、Hover、Disabled 和选项禁用；宿主受控更新格式标签。

### 无障碍与本地化

根为 combobox，当前值以 `value-label` 朗读，色彩不作为唯一信息来源。

### 验证、宿主职责与限制

Gallery 与 Harness 覆盖预设色选择。当前不内建自由取色、透明度滑杆、HEX/RGB 文本解析和吸管。

全局约束：[`foundations.md`](../../foundations.md)、[`interaction.md`](../../interaction.md)、[`accessibility.md`](../../accessibility.md)、[`content-and-localization.md`](../../content-and-localization.md)。
