# TextField

成熟度：`Alpha`。源码：`crates/slint-ui/ui/controls/text-field.slint`。公开名称：`TextField`。

用于单行文本、密码和只读输入。多行使用后续 TextArea，格式化数字使用后续 NumberInput；Label 和错误关系由 `FormRow` 提供。

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
| `text` | `in-out` | `string` | `绑定：input.text` | 共享受控状态 | 程序赋值不伪造用户编辑事件 |
| `placeholder-text` | `in` | `string` | `""` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `accessible-name` | `in` | `string` | `""` | 调用方 | 无可见文字的交互组件必须提供本地化名称 |
| `prefix-icon` | `in` | `image` | `类型默认值` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `suffix-icon` | `in` | `image` | `类型默认值` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `clearable` | `in` | `bool` | `false` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `clear-accessible-name` | `in` | `string` | `""` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `enabled` | `in` | `bool` | `true` | 调用方 | false 时阻止用户操作；程序设置不触发用户事件 |
| `read-only` | `in` | `bool` | `false` | 调用方 | true 时允许读取和焦点，不接受用户编辑 |
| `input-type` | `in` | `InputType` | `InputType.text` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `size` | `in` | `ControlSize` | `ControlSize.medium` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `validation` | `in` | `ValidationState` | `ValidationState.normal` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `has-focus` | `out` | `bool` | `绑定：input.has-focus` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |

### 事件

| 名称 | 参数 | 触发语义与边界 |
|---|---|---|
| `edited` | `text: string` | 用户编辑原始值时触发；程序赋值不触发 |
| `accepted` | `text: string` | 用户或等效公开方法明确接受当前内容后触发 |
| `cleared` | `无` | 用户执行清除且清除被接受后触发 |

### 公开方法

| 名称 | 参数 | 状态、事件与边界 |
|---|---|---|
| `clear()` | `无` | 当前实现约束：`if (clearable && enabled && !read-only && text != "") { root.text = ""; root.edited(""); root.cleared(); input.focus(); }` |

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

支持 Default、Focused、Disabled、ReadOnly、四种校验状态和有值清除入口。输入、IME、选择、剪贴板、撤销、密码和 Enter 接受由原生 TextInput 处理。`clear()` 与清除按钮走同一路径，禁用、只读、不可清除或空值时不触发。

### 无障碍与本地化

角色为 text-input，暴露名称、enabled、read-only、placeholder 和 value。Placeholder 不替代 Label；清除入口出现时必须提供本地化名称。组件不内置业务错误文案。

### 验证、宿主职责与限制

Gallery“数据录入 / TextField”页覆盖普通、错误和禁用状态；RTL 和高对比度由全局环境验证。自动测试覆盖清除、edited/cleared 单次回调、重复清除和禁用不变。清除图标使用 `close.svg`。

全局约束：[`foundations.md`](../../foundations.md)、[`interaction.md`](../../interaction.md)、[`accessibility.md`](../../accessibility.md)、[`content-and-localization.md`](../../content-and-localization.md)。
