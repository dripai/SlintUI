# ImageView

成熟度：`Alpha`。源码：`crates/slint-ui/ui/data-display/image-view.slint`。公开名称：`ImageView`。

显示单张图片及加载、空、失败状态，不提供图片编辑。

## 公开 API

### 数据类型与枚举

#### `ImageViewState`

| 字段或值 | 类型/语义 |
|---|---|
| `loading` | `枚举值` |
| `ready` | `枚举值` |
| `empty` | `枚举值` |
| `error` | `枚举值` |

### 属性

| 名称 | 方向 | 类型 | 默认值 | 所有者 | 说明与边界 |
|---|---|---|---|---|---|
| `source` | `in` | `image` | `类型默认值` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `state` | `in` | `ImageViewState` | `ImageViewState.empty` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `fit` | `in` | `ImageFit` | `ImageFit.contain` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `accessible-name` | `in` | `string` | `""` | 调用方 | 无可见文字的交互组件必须提供本地化名称 |
| `loading-text` | `in` | `string` | `""` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `empty-text` | `in` | `string` | `""` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `error-text` | `in` | `string` | `""` | 调用方 | 取值范围、联动和非法值处理见行为规范 |

### 事件

| 名称 | 参数 | 触发语义与边界 |
|---|---|---|
| `retry-requested` | `无` | 请求宿主执行操作，不表示操作已经成功 |

### 公开方法

| 名称 | 参数 | 状态、事件与边界 |
|---|---|---|
| `retry()` | `无` | 当前实现约束：`if (state == ImageViewState.error) { root.retry-requested(); }` |

## 视觉规范

### 组成结构

当前实现由 `Image`、`VerticalLayout`、`Text`、`TouchArea` 组成。内部元素不是公开 API，调用方不得依赖其名称或坐标。

### 变体、尺寸与状态外观

视觉控制入口：`state`、`loading-text`。状态组合以公开属性和行为规范为准，不为 Gallery 虚构额外状态。

### Theme Token

实际消费：`Theme.bg-surface`、`Theme.border-default`、`Theme.border-width`、`Theme.card-min-width`、`Theme.color-error`、`Theme.empty-state-min-height`、`Theme.font-size-body`、`Theme.icon-default`、`Theme.icon-size-x-large`、`Theme.radius-medium`、`Theme.space-2`、`Theme.text-secondary`。不得在调用方通过任意颜色绕过这些语义 Token。

### 内容、布局与环境

遵循全局排版、密度、长文本、100%–200% 缩放、浅色/深色/高对比度和 LTR/RTL 规则；组件特有差异见本页组成结构及行为规范。

## 行为规范

### 状态与交互

覆盖 loading、ready、empty、error；失败态整个区域可请求重试。

### 无障碍与本地化

使用 image 角色，替代说明和状态文案由调用方提供；状态使用 SVG 图标和文本共同表达。

### 验证、宿主职责与限制

Gallery 展示空状态；编译和截图验收。Slint 不向该包装层公开通用加载错误事件，宿主必须控制 `state`，且组件不含缩放、平移和缓存。

全局约束：[`foundations.md`](../../foundations.md)、[`interaction.md`](../../interaction.md)、[`accessibility.md`](../../accessibility.md)、[`content-and-localization.md`](../../content-and-localization.md)。
