# Alert

成熟度：`Alpha`。源码：`crates/slint-ui/ui/feedback/alert.slint`。公开名称：`Alert`。

用于页面内必须可见的 info、success、warning、error 消息；短暂反馈使用 Toast，持久浮层使用 Notification。

## 公开 API

### 数据类型与枚举

#### `AlertTone`

| 字段或值 | 类型/语义 |
|---|---|
| `info` | `枚举值` |
| `success` | `枚举值` |
| `warning` | `枚举值` |
| `error` | `枚举值` |

### 属性

| 名称 | 方向 | 类型 | 默认值 | 所有者 | 说明与边界 |
|---|---|---|---|---|---|
| `title` | `in` | `string` | `""` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `message` | `in` | `string` | `""` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `tone` | `in` | `AlertTone` | `AlertTone.info` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `dismissible` | `in` | `bool` | `false` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `close-accessible-name` | `in` | `string` | `""` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `action-text` | `in` | `string` | `""` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `enabled` | `in` | `bool` | `true` | 调用方 | false 时阻止用户操作；程序设置不触发用户事件 |
| `accessible-name` | `in` | `string` | `title` | 调用方 | 无可见文字的交互组件必须提供本地化名称 |

### 事件

| 名称 | 参数 | 触发语义与边界 |
|---|---|---|
| `action-requested` | `无` | 请求宿主执行操作，不表示操作已经成功 |
| `dismiss-requested` | `无` | 请求宿主执行操作，不表示操作已经成功 |

### 公开方法

| 名称 | 参数 | 状态、事件与边界 |
|---|---|---|
| `activate()` | `无` | 当前实现约束：`if (enabled && action-text != "") { root.action-requested(); }` |
| `dismiss()` | `无` | 当前实现约束：`if (enabled && dismissible) { root.dismiss-requested(); }` |

## 视觉规范

### 组成结构

当前实现由 `HorizontalLayout`、`Image`、`VerticalLayout`、`Text`、`Button`、`Rectangle`、`TouchArea` 组成。内部元素不是公开 API，调用方不得依赖其名称或坐标。

### 变体、尺寸与状态外观

视觉控制入口：`tone`。状态组合以公开属性和行为规范为准，不为 Gallery 虚构额外状态。

### Theme Token

实际消费：`Theme.border-width`、`Theme.card-min-width`、`Theme.color-error`、`Theme.color-error-bg`、`Theme.color-error-border`、`Theme.color-primary`、`Theme.color-primary-bg`、`Theme.color-primary-border`、`Theme.color-success`、`Theme.color-success-bg`、`Theme.color-success-border`、`Theme.color-warning`、`Theme.color-warning-bg`、`Theme.color-warning-border`、`Theme.control-height-small`、`Theme.fill-hover`、`Theme.font-size-body`、`Theme.font-weight-medium`、`Theme.icon-default`、`Theme.icon-size-medium`、`Theme.icon-size-x-small`、`Theme.radius-medium`、`Theme.radius-small`、`Theme.space-1`、`Theme.space-3`、`Theme.space-4`、`Theme.text-disabled`、`Theme.text-primary`、`Theme.text-secondary`。不得在调用方通过任意颜色绕过这些语义 Token。

### 内容、布局与环境

遵循全局排版、密度、长文本、100%–200% 缩放、浅色/深色/高对比度和 LTR/RTL 规则；组件特有差异见本页组成结构及行为规范。

## 行为规范

### 状态与交互

覆盖四种 tone、带/不带标题、操作、可关闭、Disabled 和长文本；禁用阻止操作与关闭。

### 无障碍与本地化

使用 region 和 polite/assertive live region；图标不重复朗读，标题、消息、操作和关闭名称必须本地化。

### 验证、宿主职责与限制

Gallery 展示 info 与 error；Harness 覆盖操作、关闭和禁用。关闭仅发请求，宿主决定移除和持久化。

全局约束：[`foundations.md`](../../foundations.md)、[`interaction.md`](../../interaction.md)、[`accessibility.md`](../../accessibility.md)、[`content-and-localization.md`](../../content-and-localization.md)。
