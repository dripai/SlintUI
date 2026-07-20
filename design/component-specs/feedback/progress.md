# Progress

成熟度：`Alpha`。源码：`crates/slint-ui/ui/feedback/progress.slint`。公开名称：`Progress`。

用于确定或不确定的线性进度。紧凑圆形进度使用后续 ProgressRing，短时未知等待可使用 Spinner；Progress 不负责异步任务生命周期。

## 公开 API

### 属性

| 名称 | 方向 | 类型 | 默认值 | 所有者 | 说明与边界 |
|---|---|---|---|---|---|
| `value` | `in` | `float` | `0.0` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `indeterminate` | `in` | `bool` | `false` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `accessible-name` | `in` | `string` | `""` | 调用方 | 无可见文字的交互组件必须提供本地化名称 |

## 视觉规范

### 组成结构

当前实现由 `Rectangle` 组成。内部元素不是公开 API，调用方不得依赖其名称或坐标。

### 变体、尺寸与状态外观

视觉控制入口：由内容和全局 Theme 决定。状态组合以公开属性和行为规范为准，不为 Gallery 虚构额外状态。

### Theme Token

实际消费：`Theme.color-primary`、`Theme.fill-pressed`、`Theme.progress-cycle`、`Theme.progress-indicator-min-width`、`Theme.progress-min-width`、`Theme.progress-track-height`、`Theme.reduced-motion`。不得在调用方通过任意颜色绕过这些语义 Token。

### 内容、布局与环境

遵循全局排版、密度、长文本、100%–200% 缩放、浅色/深色/高对比度和 LTR/RTL 规则；组件特有差异见本页组成结构及行为规范。

## 行为规范

### 状态与交互

有 determined/indeterminate 两种显示状态，不接收指针、键盘或焦点。动效使用统一时钟；reduced-motion 下不确定状态变为居中静态指示块。

### 无障碍与本地化

角色为 progress-indicator，暴露 0–100 范围和确定进度百分比；不确定时不伪造数值。调用方提供本地化名称和相邻说明，不能只靠颜色表达状态。

### 验证、宿主职责与限制

Gallery“反馈 / Progress”页覆盖确定和不确定进度；150% 缩放和主题由全局环境切换验证。数值归一化、减少动效与语义通过编译和截图验证。由编译和截图验收。

全局约束：[`foundations.md`](../../foundations.md)、[`interaction.md`](../../interaction.md)、[`accessibility.md`](../../accessibility.md)、[`content-and-localization.md`](../../content-and-localization.md)。
