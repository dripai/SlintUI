# DescriptionList

成熟度：`Alpha`。源码：`crates/slint-ui/ui/data-display/description-list.slint`。公开名称：`DescriptionList`。

用于名称/值形式的对象详情；可编辑字段使用 FormRow，列式大数据使用 Table。

## 公开 API

### 数据类型与枚举

#### `DescriptionItem`

| 字段或值 | 类型/语义 |
|---|---|
| `label` | `string` |
| `value` | `string` |

### 属性

| 名称 | 方向 | 类型 | 默认值 | 所有者 | 说明与边界 |
|---|---|---|---|---|---|
| `items` | `in` | `[DescriptionItem]` | `类型默认值` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `label-width` | `in` | `length` | `Theme.control-min-width` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `bordered` | `in` | `bool` | `false` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `accessible-name` | `in` | `string` | `""` | 调用方 | 无可见文字的交互组件必须提供本地化名称 |

## 视觉规范

### 组成结构

当前实现由 `VerticalLayout`、`Rectangle`、`Text` 组成。内部元素不是公开 API，调用方不得依赖其名称或坐标。

### 变体、尺寸与状态外观

视觉控制入口：`bordered`。状态组合以公开属性和行为规范为准，不为 Gallery 虚构额外状态。

### Theme Token

实际消费：`Theme.bg-surface`、`Theme.border-default`、`Theme.border-secondary`、`Theme.border-width`、`Theme.card-min-width`、`Theme.control-min-width`、`Theme.direction`、`Theme.font-size-body`、`Theme.font-weight-medium`、`Theme.radius-medium`、`Theme.row-height`、`Theme.space-2`、`Theme.space-3`、`Theme.text-primary`、`Theme.text-secondary`。不得在调用方通过任意颜色绕过这些语义 Token。

### 内容、布局与环境

遵循全局排版、密度、长文本、100%–200% 缩放、浅色/深色/高对比度和 LTR/RTL 规则；组件特有差异见本页组成结构及行为规范。

## 行为规范

### 状态与交互

覆盖有/无边框、空列表、长 label/value、RTL 和缩放。

### 无障碍与本地化

根使用 groupbox 并暴露项数；标签和值按源码相邻呈现。值格式化和本地化由数据层完成。

### 验证、宿主职责与限制

Gallery 展示三行带边框详情；编译与截图验收。当前使用单列名称/值布局，不自动转为多列响应式网格。

全局约束：[`foundations.md`](../../foundations.md)、[`interaction.md`](../../interaction.md)、[`accessibility.md`](../../accessibility.md)、[`content-and-localization.md`](../../content-and-localization.md)。
