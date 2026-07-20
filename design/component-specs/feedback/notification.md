# Notification

成熟度：`Alpha`。源码：`crates/slint-ui/ui/feedback/notification.slint`。公开名称：`Notification`。

展示带标题、详情和操作的持久通知；短暂反馈使用 Toast，系统通知属于平台增强。

## 公开 API

### 数据类型与枚举

#### `NotificationTone`

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
| `tone` | `in` | `NotificationTone` | `NotificationTone.info` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `action-text` | `in` | `string` | `""` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `close-accessible-name` | `in` | `string` | `""` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `dismissible` | `in` | `bool` | `true` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `busy` | `in` | `bool` | `false` | 调用方 | true 时阻止提交、关闭或重复操作，具体见行为规范 |
| `accessible-name` | `in` | `string` | `title` | 调用方 | 无可见文字的交互组件必须提供本地化名称 |

### 事件

| 名称 | 参数 | 触发语义与边界 |
|---|---|---|
| `action-requested` | `无` | 请求宿主执行操作，不表示操作已经成功 |
| `dismiss-requested` | `无` | 请求宿主执行操作，不表示操作已经成功 |

### 公开方法

| 名称 | 参数 | 状态、事件与边界 |
|---|---|---|
| `activate()` | `无` | 当前实现约束：`if (action-text != "" && !busy) { root.action-requested(); }` |
| `dismiss()` | `无` | 当前实现约束：`if (dismissible) { root.dismiss-requested(); }` |

## 视觉规范

### 组成结构

当前实现由 `HorizontalLayout`、`Image`、`VerticalLayout`、`Text`、`Button`、`Rectangle`、`TouchArea` 组成。内部元素不是公开 API，调用方不得依赖其名称或坐标。

### 变体、尺寸与状态外观

视觉控制入口：`tone`、`busy`。状态组合以公开属性和行为规范为准，不为 Gallery 虚构额外状态。

### Theme Token

实际消费：`Theme.border-width`、`Theme.color-error`、`Theme.color-error-bg`、`Theme.color-error-border`、`Theme.color-primary`、`Theme.color-primary-bg`、`Theme.color-primary-border`、`Theme.color-success`、`Theme.color-success-bg`、`Theme.color-success-border`、`Theme.color-warning`、`Theme.color-warning-bg`、`Theme.color-warning-border`、`Theme.control-height-small`、`Theme.fill-hover`、`Theme.font-size-body`、`Theme.font-weight-medium`、`Theme.icon-default`、`Theme.icon-size-medium`、`Theme.icon-size-x-small`、`Theme.radius-large`、`Theme.radius-small`、`Theme.space-2`、`Theme.space-3`、`Theme.space-4`、`Theme.text-primary`、`Theme.text-secondary`、`Theme.toast-width`。不得在调用方通过任意颜色绕过这些语义 Token。

### 内容、布局与环境

遵循全局排版、密度、长文本、100%–200% 缩放、浅色/深色/高对比度和 LTR/RTL 规则；组件特有差异见本页组成结构及行为规范。

## 行为规范

### 状态与交互

覆盖 info、success、warning、error、操作 Loading 和可关闭状态；不会自动消失。

### 无障碍与本地化

根为 live region，错误为 assertive，其余为 polite；语义 SVG 与文字共同表达状态。

### 验证、宿主职责与限制

Gallery“反馈 / Notification”页展示带图标、标题、说明和操作的通知；编译和截图验收。队列、位置、持久化和跨窗口调度由宿主负责，不调用操作系统通知 API。

全局约束：[`foundations.md`](../../foundations.md)、[`interaction.md`](../../interaction.md)、[`accessibility.md`](../../accessibility.md)、[`content-and-localization.md`](../../content-and-localization.md)。
