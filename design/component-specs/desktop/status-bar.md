# StatusBar

成熟度：`Alpha`。源码：`crates/slint-ui/ui/feedback/status-bar.slint`。公开名称：`StatusBar`。

用于窗口级状态、连接信息和简短进度说明。需要用户操作的重要错误使用 Alert/Dialog，短暂通知使用后续 Toast；StatusBar 不保存历史。

## 公开 API

### 数据类型与枚举

#### `StatusTone`

| 字段或值 | 类型/语义 |
|---|---|
| `neutral` | `枚举值` |
| `info` | `枚举值` |
| `success` | `枚举值` |
| `warning` | `枚举值` |
| `error` | `枚举值` |

### 属性

| 名称 | 方向 | 类型 | 默认值 | 所有者 | 说明与边界 |
|---|---|---|---|---|---|
| `text` | `in` | `string` | `""` | 调用方 | 程序赋值不伪造用户编辑事件 |
| `detail` | `in` | `string` | `""` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `tone` | `in` | `StatusTone` | `StatusTone.neutral` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `busy` | `in` | `bool` | `false` | 调用方 | true 时阻止提交、关闭或重复操作，具体见行为规范 |
| `accessible-name` | `in` | `string` | `text` | 调用方 | 无可见文字的交互组件必须提供本地化名称 |

## 视觉规范

### 组成结构

当前实现由 `HorizontalLayout`、`Spinner`、`Rectangle`、`Text` 组成。内部元素不是公开 API，调用方不得依赖其名称或坐标。

### 变体、尺寸与状态外观

视觉控制入口：`tone`、`busy`。状态组合以公开属性和行为规范为准，不为 Gallery 虚构额外状态。

### Theme Token

实际消费：`Theme.bg-surface`、`Theme.border-secondary`、`Theme.border-width`、`Theme.color-error`、`Theme.color-primary`、`Theme.color-success`、`Theme.color-warning`、`Theme.font-size-caption`、`Theme.icon-default`、`Theme.row-height`、`Theme.space-2`、`Theme.space-3`、`Theme.status-indicator-size`、`Theme.text-primary`、`Theme.text-secondary`。不得在调用方通过任意颜色绕过这些语义 Token。

### 内容、布局与环境

遵循全局排版、密度、长文本、100%–200% 缩放、浅色/深色/高对比度和 LTR/RTL 规则；组件特有差异见本页组成结构及行为规范。

## 行为规范

### 状态与交互

支持五种 tone 和 busy；不接收指针、键盘或焦点。busy 显示 Spinner，否则显示状态点；文字和颜色共同表达状态，长内容省略。

### 无障碍与本地化

角色为 content-info，名称、说明和 polite live region 用于状态更新。调用方提供已本地化的短文本；状态色不能作为唯一信息，RTL 下文本顺序由内容语义决定。

### 验证、宿主职责与限制

Gallery 全局底栏及“反馈”页覆盖 info/success、Locale、方向和缩放信息；通过编译、冒烟和截图验收。没有可变回调，因而不需要交互状态转换测试。

全局约束：[`foundations.md`](../../foundations.md)、[`interaction.md`](../../interaction.md)、[`accessibility.md`](../../accessibility.md)、[`content-and-localization.md`](../../content-and-localization.md)。
