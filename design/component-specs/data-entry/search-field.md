# SearchField

成熟度：`Alpha`。源码：`crates/slint-ui/ui/controls/search-field.slint`。公开名称：`SearchField`。

用于输入并提交查询；普通文本使用 TextField，候选建议使用 AutoComplete。

## 公开 API

### 数据类型与枚举

#### `ControlSize`

| 字段或值 | 类型/语义 |
|---|---|
| `small` | `枚举值` |
| `medium` | `枚举值` |
| `large` | `枚举值` |

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
| `text` | `in-out` | `string` | `绑定：input.text` | 共享受控状态 | 程序赋值不伪造用户编辑事件；继承自 TextField |
| `placeholder-text` | `in` | `string` | `""` | 调用方 | 取值范围、联动和非法值处理见行为规范；继承自 TextField |
| `accessible-name` | `in` | `string` | `""` | 调用方 | 无可见文字的交互组件必须提供本地化名称；继承自 TextField |
| `prefix-icon` | `in` | `image` | `类型默认值` | 调用方 | 取值范围、联动和非法值处理见行为规范；继承自 TextField |
| `suffix-icon` | `in` | `image` | `类型默认值` | 调用方 | 取值范围、联动和非法值处理见行为规范；继承自 TextField |
| `clearable` | `in` | `bool` | `false` | 调用方 | 取值范围、联动和非法值处理见行为规范；继承自 TextField |
| `clear-accessible-name` | `in` | `string` | `""` | 调用方 | 取值范围、联动和非法值处理见行为规范；继承自 TextField |
| `enabled` | `in` | `bool` | `true` | 调用方 | false 时阻止用户操作；程序设置不触发用户事件；继承自 TextField |
| `read-only` | `in` | `bool` | `false` | 调用方 | true 时允许读取和焦点，不接受用户编辑；继承自 TextField |
| `input-type` | `in` | `InputType` | `InputType.text` | 调用方 | 取值范围、联动和非法值处理见行为规范；继承自 TextField |
| `size` | `in` | `ControlSize` | `ControlSize.medium` | 调用方 | 取值范围、联动和非法值处理见行为规范；继承自 TextField |
| `validation` | `in` | `ValidationState` | `ValidationState.normal` | 调用方 | 取值范围、联动和非法值处理见行为规范；继承自 TextField |
| `has-focus` | `out` | `bool` | `绑定：input.has-focus` | 组件派生 | 取值范围、联动和非法值处理见行为规范；继承自 TextField |

### 事件

| 名称 | 参数 | 触发语义与边界 |
|---|---|---|
| `searched` | `query: string` | 由该组件定义的有效用户操作或等效公开方法触发；阻止条件见行为规范 |
| `edited` | `text: string` | 用户编辑原始值时触发；程序赋值不触发；继承自 TextField |
| `accepted` | `text: string` | 用户或等效公开方法明确接受当前内容后触发；继承自 TextField |
| `cleared` | `无` | 用户执行清除且清除被接受后触发；继承自 TextField |

### 公开方法

| 名称 | 参数 | 状态、事件与边界 |
|---|---|---|
| `submit()` | `无` | 当前实现约束：`if (root.enabled) { root.searched(root.text); }` |
| `clear()` | `无` | 当前实现约束：`if (clearable && enabled && !read-only && text != "") { root.text = ""; root.edited(""); root.cleared(); input.focus(); }`；继承自 TextField |

## 视觉规范

### 组成结构

当前实现由 `Image`、`Text`、`TextInput`、`Rectangle`、`TouchArea`、`FocusRing` 组成。内部元素不是公开 API，调用方不得依赖其名称或坐标。

### 变体、尺寸与状态外观

视觉控制入口：`size`、`validation`。状态组合以公开属性和行为规范为准，不为 Gallery 虚构额外状态。

### Theme Token

实际消费：`Theme.bg-control`、`Theme.bg-disabled`、`Theme.border-default`、`Theme.border-width`、`Theme.color-error`、`Theme.color-success`、`Theme.color-warning`、`Theme.control-height-large`、`Theme.control-height-medium`、`Theme.control-height-small`、`Theme.control-min-width`、`Theme.control-padding-horizontal`、`Theme.field-affix-slot`、`Theme.fill-hover`、`Theme.focus-outline`、`Theme.font-size-body`、`Theme.font-weight-regular`、`Theme.icon-default`、`Theme.icon-size-medium`、`Theme.icon-size-small`、`Theme.icon-size-x-small`、`Theme.radius-medium`、`Theme.radius-small`、`Theme.selection-bg`、`Theme.text-disabled`、`Theme.text-placeholder`、`Theme.text-primary`。不得在调用方通过任意颜色绕过这些语义 Token。

### 内容、布局与环境

遵循全局排版、密度、长文本、100%–200% 缩放、浅色/深色/高对比度和 LTR/RTL 规则；组件特有差异见本页组成结构及行为规范。

## 行为规范

### 状态与交互

覆盖 Default、Focused、Populated、Empty、Clear、Disabled、Validation 和长文本；Enter 与 `submit()` 触发搜索，连续提交保留显式用户意图。

### 无障碍与本地化

继承 text-input 语义；`accessible-name`、placeholder 和清除按钮名称必须本地化。

### 验证、宿主职责与限制

Gallery 展示有值和禁用；Harness 覆盖重复提交。延迟输入由宿主去抖，不内建计时和数据请求。

全局约束：[`foundations.md`](../../foundations.md)、[`interaction.md`](../../interaction.md)、[`accessibility.md`](../../accessibility.md)、[`content-and-localization.md`](../../content-and-localization.md)。
