# Toast

成熟度：`Alpha`。源码：`crates/slint-ui/ui/feedback/toast.slint`。公开名称：`Toast`。

展示短暂、非阻塞且可忽略的结果；安全、权限、数据丢失或必须操作的信息使用 Alert/Dialog。

## 公开 API

### 数据类型与枚举

#### `ToastMessage`

| 字段或值 | 类型/语义 |
|---|---|
| `id` | `int` |
| `text` | `string` |
| `detail` | `string` |
| `tone` | `ToastTone` |
| `duration` | `duration` |
| `dismissible` | `bool` |

### 属性

| 名称 | 方向 | 类型 | 默认值 | 所有者 | 说明与边界 |
|---|---|---|---|---|---|
| `messages` | `in` | `[ToastMessage]` | `类型默认值` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `accessible-name` | `in` | `string` | `"Notifications"` | 调用方 | 无可见文字的交互组件必须提供本地化名称 |

### 事件

| 名称 | 参数 | 触发语义与边界 |
|---|---|---|
| `dismiss-requested` | `id: int` | 请求宿主执行操作，不表示操作已经成功 |

### 公开方法

| 名称 | 参数 | 状态、事件与边界 |
|---|---|---|
| `request-dismiss()` | `index: int` | 当前实现约束：`if (index >= 0 && index < messages.length && messages[index].dismissible) { root.dismiss-requested(messages[index].id); }` |

## 视觉规范

### 组成结构

当前实现由 `VerticalLayout`、`ToastItem` 组成。内部元素不是公开 API，调用方不得依赖其名称或坐标。

### 变体、尺寸与状态外观

视觉控制入口：由内容和全局 Theme 决定。状态组合以公开属性和行为规范为准，不为 Gallery 虚构额外状态。

### Theme Token

实际消费：`Theme.space-2`。不得在调用方通过任意颜色绕过这些语义 Token。

### 内容、布局与环境

遵循全局排版、密度、长文本、100%–200% 缩放、浅色/深色/高对比度和 LTR/RTL 规则；组件特有差异见本页组成结构及行为规范。

## 行为规范

### 状态与交互

按模型顺序排队，duration 大于零时自动请求关闭，可显式关闭；模型删除由宿主原子完成，组件不静默修改业务队列。

### 无障碍与本地化

普通消息 polite，错误 assertive；文本与详情由宿主本地化，关闭按钮提供可访问名称。

### 验证、宿主职责与限制

Gallery 固定展示 success Toast，截图覆盖。Hover/焦点暂停计时受 Slint Timer 公开能力限制，重要内容禁止放入自动关闭 Toast。

全局约束：[`foundations.md`](../../foundations.md)、[`interaction.md`](../../interaction.md)、[`accessibility.md`](../../accessibility.md)、[`content-and-localization.md`](../../content-and-localization.md)。
