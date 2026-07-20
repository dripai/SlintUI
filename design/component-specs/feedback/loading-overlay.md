# LoadingOverlay

成熟度：`Alpha`。源码：`crates/slint-ui/ui/feedback/loading-overlay.slint`。公开名称：`LoadingOverlay`。

用于明确阻塞一个区域的加载任务；非阻塞加载使用 Spinner/Skeleton，整窗模态任务由 Dialog 承担。

## 公开 API

### 属性

| 名称 | 方向 | 类型 | 默认值 | 所有者 | 说明与边界 |
|---|---|---|---|---|---|
| `active` | `in` | `bool` | `false` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `text` | `in` | `string` | `""` | 调用方 | 程序赋值不伪造用户编辑事件 |
| `cancelable` | `in` | `bool` | `false` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `cancel-text` | `in` | `string` | `""` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `accessible-name` | `in` | `string` | `text` | 调用方 | 无可见文字的交互组件必须提供本地化名称 |

### 事件

| 名称 | 参数 | 触发语义与边界 |
|---|---|---|
| `cancel-requested` | `无` | 请求宿主执行操作，不表示操作已经成功 |

### 公开方法

| 名称 | 参数 | 状态、事件与边界 |
|---|---|---|
| `cancel()` | `无` | 当前实现约束：`if (active && cancelable) { root.cancel-requested(); }` |

## 视觉规范

### 组成结构

当前实现由 `TouchArea`、`Rectangle`、`VerticalLayout`、`Spinner`、`Text`、`Button` 组成。内部元素不是公开 API，调用方不得依赖其名称或坐标。

### 变体、尺寸与状态外观

视觉控制入口：`active`。状态组合以公开属性和行为规范为准，不为 Gallery 虚构额外状态。

### Theme Token

实际消费：`Theme.bg-elevated`、`Theme.border-default`、`Theme.border-width`、`Theme.font-size-body`、`Theme.overlay-content-min-width`、`Theme.overlay-mask`、`Theme.radius-large`、`Theme.space-3`、`Theme.space-4`、`Theme.space-6`、`Theme.space-8`、`Theme.text-primary`。不得在调用方通过任意颜色绕过这些语义 Token。

### 内容、布局与环境

遵循全局排版、密度、长文本、100%–200% 缩放、浅色/深色/高对比度和 LTR/RTL 规则；组件特有差异见本页组成结构及行为规范。

## 行为规范

### 状态与交互

覆盖 Inactive、Active、Cancelable、ReducedMotion、长文本和禁用式遮罩；仅 active 且 cancelable 时回调。

### 无障碍与本地化

使用 region 和 polite live region，遮罩拦截区域输入；加载说明和取消文字必须本地化。

### 验证、宿主职责与限制

Gallery 展示区域遮罩、Spinner 和取消；Harness 覆盖 active 与 inactive 边界。取消只发请求，宿主负责真正终止任务和关闭遮罩。

全局约束：[`foundations.md`](../../foundations.md)、[`interaction.md`](../../interaction.md)、[`accessibility.md`](../../accessibility.md)、[`content-and-localization.md`](../../content-and-localization.md)。
