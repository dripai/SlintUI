# EmptyState

成熟度：`Alpha`。源码：`crates/slint-ui/ui/feedback/empty-state.slint`。公开名称：`EmptyState`。

用于无数据、无结果、无权限三类空状态。加载中使用 Progress/Spinner，成功或失败结果使用后续 ResultState；组件不推断业务原因。

## 公开 API

### 数据类型与枚举

#### `EmptyStateKind`

| 字段或值 | 类型/语义 |
|---|---|
| `no-data` | `枚举值` |
| `no-results` | `枚举值` |
| `no-permission` | `枚举值` |

### 属性

| 名称 | 方向 | 类型 | 默认值 | 所有者 | 说明与边界 |
|---|---|---|---|---|---|
| `kind` | `in` | `EmptyStateKind` | `EmptyStateKind.no-data` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `icon` | `in` | `image` | `类型默认值` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `title` | `in` | `string` | `""` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `description` | `in` | `string` | `""` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `accessible-name` | `in` | `string` | `title` | 调用方 | 无可见文字的交互组件必须提供本地化名称 |

### 内容入口

`@children` 插入到组件声明的内容区域；尺寸、裁剪和焦点顺序由该区域布局与行为规范约束。

## 视觉规范

### 组成结构

当前实现由 `VerticalLayout`、`Image`、`Text` 组成。内部元素不是公开 API，调用方不得依赖其名称或坐标。

### 变体、尺寸与状态外观

视觉控制入口：`kind`。状态组合以公开属性和行为规范为准，不为 Gallery 虚构额外状态。

### Theme Token

实际消费：`Theme.color-warning`、`Theme.empty-state-min-height`、`Theme.font-size-body`、`Theme.font-size-title-small`、`Theme.font-weight-medium`、`Theme.icon-default`、`Theme.icon-size-empty-state`、`Theme.line-height-body`、`Theme.line-height-title-small`、`Theme.space-2`、`Theme.space-6`、`Theme.text-primary`、`Theme.text-secondary`。不得在调用方通过任意颜色绕过这些语义 Token。

### 内容、布局与环境

遵循全局排版、密度、长文本、100%–200% 缩放、浅色/深色/高对比度和 LTR/RTL 规则；组件特有差异见本页组成结构及行为规范。

## 行为规范

### 状态与交互

EmptyState 本身不接收输入；操作按钮作为子项保持各自焦点和激活行为。三种 kind 只表达语义与图标色，不内置业务文案或静默替代。

### 无障碍与本地化

角色为 region，标题作为名称，说明作为描述，图标为装饰。调用方必须提供本地化标题、说明和操作；支持长文本、RTL 与文本缩放。

### 验证、宿主职责与限制

Gallery“数据展示 / EmptyState”页覆盖 no-data、图标、说明和主要操作；主题、Locale 和缩放由全局切换验证。非交互容器通过编译和截图验收。

全局约束：[`foundations.md`](../../foundations.md)、[`interaction.md`](../../interaction.md)、[`accessibility.md`](../../accessibility.md)、[`content-and-localization.md`](../../content-and-localization.md)。
