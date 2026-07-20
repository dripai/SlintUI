# CheckboxGroup

成熟度：`Alpha`。源码：`crates/slint-ui/ui/controls/checkbox-group.slint`。公开名称：`CheckboxGroup`。

用于同一问题下可多选的一组选项；独立布尔值使用 Checkbox，互斥选择使用 RadioGroup。

## 公开 API

### 数据类型与枚举

#### `CheckboxGroupItem`

| 字段或值 | 类型/语义 |
|---|---|
| `text` | `string` |
| `value` | `string` |
| `checked` | `bool` |
| `enabled` | `bool` |

### 属性

| 名称 | 方向 | 类型 | 默认值 | 所有者 | 说明与边界 |
|---|---|---|---|---|---|
| `items` | `in-out` | `[CheckboxGroupItem]` | `类型默认值` | 共享受控状态 | 取值范围、联动和非法值处理见行为规范 |
| `enabled` | `in` | `bool` | `true` | 调用方 | false 时阻止用户操作；程序设置不触发用户事件 |
| `accessible-name` | `in` | `string` | `""` | 调用方 | 无可见文字的交互组件必须提供本地化名称 |
| `error-text` | `in` | `string` | `""` | 调用方 | 取值范围、联动和非法值处理见行为规范 |

### 事件

| 名称 | 参数 | 触发语义与边界 |
|---|---|---|
| `changed` | `index: int, checked: bool` | 组件接受一个不同的新值后触发一次 |

### 公开方法

| 名称 | 参数 | 状态、事件与边界 |
|---|---|---|
| `set-checked()` | `index: int, checked: bool` | 当前实现约束：`if (enabled && index >= 0 && index < items.length && items[index].enabled && items[index].checked != checked) { root.items[index].checked = checked; root.changed(index, checked); }` |

## 视觉规范

### 组成结构

当前实现由 `VerticalLayout`、`Checkbox`、`Text` 组成。内部元素不是公开 API，调用方不得依赖其名称或坐标。

### 变体、尺寸与状态外观

视觉控制入口：由内容和全局 Theme 决定。状态组合以公开属性和行为规范为准，不为 Gallery 虚构额外状态。

### Theme Token

实际消费：`Theme.color-error`、`Theme.font-size-caption`、`Theme.space-2`。不得在调用方通过任意颜色绕过这些语义 Token。

### 内容、布局与环境

遵循全局排版、密度、长文本、100%–200% 缩放、浅色/深色/高对比度和 LTR/RTL 规则；组件特有差异见本页组成结构及行为规范。

## 行为规范

### 状态与交互

覆盖 Checked、Unchecked、Focused、Disabled、组错误和长文本；同值、禁用项和越界不回调。

### 无障碍与本地化

根使用 groupbox 并暴露错误说明和项数；条目复用 Checkbox 语义。文案和值分离，显示文字由调用方本地化。

### 验证、宿主职责与限制

Gallery 展示选中、未选、禁用和错误；Harness 覆盖正常、重复、禁用和越界。首期不提供“全选”业务策略。

全局约束：[`foundations.md`](../../foundations.md)、[`interaction.md`](../../interaction.md)、[`accessibility.md`](../../accessibility.md)、[`content-and-localization.md`](../../content-and-localization.md)。
