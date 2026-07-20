# AutoComplete

成熟度：`Alpha`。源码：`crates/slint-ui/ui/controls/auto-complete.slint`。公开名称：`AutoComplete`。

在自由输入时给出建议，不等同于必须从集合中选择的 Select。

## 公开 API

### 数据类型与枚举

#### `AutoCompleteOption`

| 字段或值 | 类型/语义 |
|---|---|
| `text` | `string` |
| `detail` | `string` |
| `enabled` | `bool` |

#### `ValidationState`

| 字段或值 | 类型/语义 |
|---|---|
| `normal` | `枚举值` |
| `success` | `枚举值` |
| `warning` | `枚举值` |
| `error` | `枚举值` |

### 属性

| 名称 | 方向 | 类型 | 默认值 | 所有者 | 说明与边界 |
|---|---|---|---|---|---|
| `text` | `in-out` | `string` | `类型默认值` | 共享受控状态 | 程序赋值不伪造用户编辑事件 |
| `suggestions` | `in` | `[AutoCompleteOption]` | `类型默认值` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `placeholder-text` | `in` | `string` | `""` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `accessible-name` | `in` | `string` | `""` | 调用方 | 无可见文字的交互组件必须提供本地化名称 |
| `enabled` | `in` | `bool` | `true` | 调用方 | false 时阻止用户操作；程序设置不触发用户事件 |
| `validation` | `in` | `ValidationState` | `ValidationState.normal` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `current-index` | `in-out` | `int` | `-1` | 共享受控状态 | -1 表示无选择；越界值的处理见行为规范 |
| `open` | `out` | `bool` | `popup.is-open` | 组件派生 | 打开状态的所有权和关闭原因见行为规范 |

### 事件

| 名称 | 参数 | 触发语义与边界 |
|---|---|---|
| `edited` | `text: string` | 用户编辑原始值时触发；程序赋值不触发 |
| `selected` | `index: int, value: string` | 用户选择有效且可用项目后触发一次 |
| `accepted` | `text: string` | 用户或等效公开方法明确接受当前内容后触发 |

### 公开方法

| 名称 | 参数 | 状态、事件与边界 |
|---|---|---|
| `show()` | `无` | 当前实现约束：`if (enabled && suggestions.length > 0) { popup.show(); }` |
| `close()` | `无` | 当前实现约束：`popup.close();` |
| `choose()` | `index: int` | 当前实现约束：`if (index >= 0 && index < suggestions.length && suggestions[index].enabled) { root.text = suggestions[index].text; root.current-index = index; root.selected(index, root.text); r...` |

## 视觉规范

### 组成结构

当前实现由 `TextField`、`PopupWindow`、`Rectangle`、`VerticalLayout`、`Text`、`TouchArea` 组成。内部元素不是公开 API，调用方不得依赖其名称或坐标。

### 变体、尺寸与状态外观

视觉控制入口：`validation`。状态组合以公开属性和行为规范为准，不为 Gallery 虚构额外状态。

### Theme Token

实际消费：`Theme.bg-elevated`、`Theme.border-default`、`Theme.border-width`、`Theme.control-min-width`、`Theme.fill-hover`、`Theme.font-size-body`、`Theme.font-size-caption`、`Theme.popup-max-height`、`Theme.popup-min-width`、`Theme.radius-large`、`Theme.row-height`、`Theme.space-1`、`Theme.space-3`、`Theme.text-disabled`、`Theme.text-primary`、`Theme.text-secondary`。不得在调用方通过任意颜色绕过这些语义 Token。

### 内容、布局与环境

遵循全局排版、密度、长文本、100%–200% 缩放、浅色/深色/高对比度和 LTR/RTL 规则；组件特有差异见本页组成结构及行为规范。

## 行为规范

### 状态与交互

输入触发建议弹层；支持选择、提交、禁用项和错误边框。

### 无障碍与本地化

根为 combobox，建议提供名称与说明；匹配、排序和大小写规则由宿主按 Locale 处理。

### 验证、宿主职责与限制

Gallery 与 Harness 覆盖建议选择和禁用。当前不内建防抖、异步加载、方向键高亮循环和模糊匹配。

全局约束：[`foundations.md`](../../foundations.md)、[`interaction.md`](../../interaction.md)、[`accessibility.md`](../../accessibility.md)、[`content-and-localization.md`](../../content-and-localization.md)。
