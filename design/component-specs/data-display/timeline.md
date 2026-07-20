# Timeline

成熟度：`Alpha`。源码：`crates/slint-ui/ui/data-display/timeline.slint`。公开名称：`Timeline`。

按顺序展示事件节点和状态，不用于精确时间轴编辑。

## 公开 API

### 数据类型与枚举

#### `TimelineItem`

| 字段或值 | 类型/语义 |
|---|---|
| `title` | `string` |
| `detail` | `string` |
| `time` | `string` |
| `tone` | `TimelineTone` |
| `enabled` | `bool` |

### 属性

| 名称 | 方向 | 类型 | 默认值 | 所有者 | 说明与边界 |
|---|---|---|---|---|---|
| `items` | `in` | `[TimelineItem]` | `类型默认值` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `accessible-name` | `in` | `string` | `""` | 调用方 | 无可见文字的交互组件必须提供本地化名称 |

### 事件

| 名称 | 参数 | 触发语义与边界 |
|---|---|---|
| `selected` | `index: int` | 用户选择有效且可用项目后触发一次 |

### 公开方法

| 名称 | 参数 | 状态、事件与边界 |
|---|---|---|
| `select()` | `index: int` | 当前实现约束：`if (index >= 0 && index < items.length && items[index].enabled) { root.selected(index); }` |

## 视觉规范

### 组成结构

当前实现由 `VerticalLayout`、`Rectangle`、`HorizontalLayout`、`Text`、`TouchArea` 组成。内部元素不是公开 API，调用方不得依赖其名称或坐标。

### 变体、尺寸与状态外观

视觉控制入口：由内容和全局 Theme 决定。状态组合以公开属性和行为规范为准，不为 Gallery 虚构额外状态。

### Theme Token

实际消费：`Theme.bg-surface`、`Theme.border-default`、`Theme.border-width`、`Theme.card-min-width`、`Theme.color-error`、`Theme.color-primary`、`Theme.color-success`、`Theme.color-warning`、`Theme.font-size-body`、`Theme.font-size-caption`、`Theme.font-weight-medium`、`Theme.icon-size-small`、`Theme.space-1`、`Theme.space-2`、`Theme.space-3`、`Theme.space-4`、`Theme.text-disabled`、`Theme.text-primary`、`Theme.text-secondary`、`Theme.text-tertiary`。不得在调用方通过任意颜色绕过这些语义 Token。

### 内容、布局与环境

遵循全局排版、密度、长文本、100%–200% 缩放、浅色/深色/高对比度和 LTR/RTL 规则；组件特有差异见本页组成结构及行为规范。

## 行为规范

### 状态与交互

覆盖五种语义色、Hover 和 Disabled；条目由宿主按时间顺序提供。

### 无障碍与本地化

根为 list，条目朗读标题、详情和时间；日期时间字符串由宿主本地化。

### 验证、宿主职责与限制

Gallery 展示两种状态；公开选择方法经编译覆盖。当前不内建折叠、异步加载和左右交错布局。

全局约束：[`foundations.md`](../../foundations.md)、[`interaction.md`](../../interaction.md)、[`accessibility.md`](../../accessibility.md)、[`content-and-localization.md`](../../content-and-localization.md)。
