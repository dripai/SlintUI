# Skeleton

成熟度：`Alpha`。源码：`crates/slint-ui/ui/feedback/skeleton.slint`。公开名称：`Skeleton`。

首次加载时提供内容轮廓占位；短时局部操作使用 Spinner。

## 公开 API

### 属性

| 名称 | 方向 | 类型 | 默认值 | 所有者 | 说明与边界 |
|---|---|---|---|---|---|
| `rows` | `in` | `int` | `3` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `show-avatar` | `in` | `bool` | `false` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `active` | `in` | `bool` | `true` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `accessible-name` | `in` | `string` | `""` | 调用方 | 无可见文字的交互组件必须提供本地化名称 |
| `status-text` | `in` | `string` | `""` | 调用方 | 取值范围、联动和非法值处理见行为规范 |

## 视觉规范

### 组成结构

当前实现由 `HorizontalLayout`、`Rectangle`、`VerticalLayout`、`Timer` 组成。内部元素不是公开 API，调用方不得依赖其名称或坐标。

### 变体、尺寸与状态外观

视觉控制入口：`active`。状态组合以公开属性和行为规范为准，不为 Gallery 虚构额外状态。

### Theme Token

实际消费：`Theme.avatar-size-large`、`Theme.border-secondary`、`Theme.card-min-width`、`Theme.fill-hover`、`Theme.line-height-body`、`Theme.motion-medium`、`Theme.radius-small`、`Theme.reduced-motion`、`Theme.skeleton-cycle`、`Theme.space-2`、`Theme.space-3`。不得在调用方通过任意颜色绕过这些语义 Token。

### 内容、布局与环境

遵循全局排版、密度、长文本、100%–200% 缩放、浅色/深色/高对比度和 LTR/RTL 规则；组件特有差异见本页组成结构及行为规范。

## 行为规范

### 状态与交互

非交互组件；活动时在两个 Theme 语义填充色间脉冲，`reduced-motion` 下静止。

### 无障碍与本地化

使用 progress-indicator 角色和 loading 值；必须提供描述加载对象的名称。

### 验证、宿主职责与限制

Gallery 展示头像加三行骨架；截图强制减少动效。当前没有渐变 shimmer、逐行宽度模型和内容形状自动推断。

全局约束：[`foundations.md`](../../foundations.md)、[`interaction.md`](../../interaction.md)、[`accessibility.md`](../../accessibility.md)、[`content-and-localization.md`](../../content-and-localization.md)。
