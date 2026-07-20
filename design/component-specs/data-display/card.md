# Card

成熟度：`Alpha`。源码：`crates/slint-ui/ui/layout/card.slint`。公开名称：`Card`。

用于组织标题、说明和内容区域。无结构的背景使用 `Surface`；可折叠内容使用后续 `Collapse`，不要让 Card 承担交互容器职责。

## 公开 API

### 属性

| 名称 | 方向 | 类型 | 默认值 | 所有者 | 说明与边界 |
|---|---|---|---|---|---|
| `title` | `in` | `string` | `""` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `description` | `in` | `string` | `""` | 调用方 | 取值范围、联动和非法值处理见行为规范 |

### 内容入口

`@children` 插入到组件声明的内容区域；尺寸、裁剪和焦点顺序由该区域布局与行为规范约束。

## 视觉规范

### 组成结构

当前实现由 `VerticalLayout`、`Text` 组成。内部元素不是公开 API，调用方不得依赖其名称或坐标。

### 变体、尺寸与状态外观

视觉控制入口：由内容和全局 Theme 决定。状态组合以公开属性和行为规范为准，不为 Gallery 虚构额外状态。

### Theme Token

实际消费：`Theme.bg-surface`、`Theme.border-default`、`Theme.border-width`、`Theme.card-min-width`、`Theme.direction`、`Theme.font-size-body`、`Theme.font-size-title-small`、`Theme.font-weight-medium`、`Theme.line-height-body`、`Theme.line-height-title-small`、`Theme.radius-large`、`Theme.space-3`、`Theme.space-4`、`Theme.text-primary`、`Theme.text-secondary`。不得在调用方通过任意颜色绕过这些语义 Token。

### 内容、布局与环境

遵循全局排版、密度、长文本、100%–200% 缩放、浅色/深色/高对比度和 LTR/RTL 规则；组件特有差异见本页组成结构及行为规范。

## 行为规范

### 状态与交互

没有 Hover、Pressed、Focused、Selected、Disabled 或 Loading 状态，不接收输入。表面、边框、圆角、内边距和排版来自 Theme。

### 无障碍与本地化

没有 Hover、Pressed、Focused、Selected、Disabled 或 Loading 状态，不接收输入。表面、边框、圆角、内边距和排版来自 Theme。

标题和说明由调用方本地化，支持长文本、RTL 和文本缩放。Card 本身不声明可访问角色；需要区域名称时由页面语义容器提供，避免无意义嵌套分组。

### 验证、宿主职责与限制

Gallery 所有内容页使用 Card，覆盖主题、密度、缩放和 RTL；非交互组件通过编译和代表性截图验收。

全局约束：[`foundations.md`](../../foundations.md)、[`interaction.md`](../../interaction.md)、[`accessibility.md`](../../accessibility.md)、[`content-and-localization.md`](../../content-and-localization.md)。
