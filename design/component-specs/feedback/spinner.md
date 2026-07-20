# Spinner

成熟度：`Alpha`。源码：`crates/slint-ui/ui/feedback/spinner.slint`。公开名称：`Spinner`。

用于局部短时加载。可量化任务使用 Progress，阻塞整个区域使用后续 LoadingOverlay；Spinner 不管理任务、超时或取消。

## 公开 API

### 数据类型与枚举

#### `SpinnerSize`

| 字段或值 | 类型/语义 |
|---|---|
| `small` | `枚举值` |
| `medium` | `枚举值` |
| `large` | `枚举值` |

### 属性

| 名称 | 方向 | 类型 | 默认值 | 所有者 | 说明与边界 |
|---|---|---|---|---|---|
| `size` | `in` | `SpinnerSize` | `SpinnerSize.medium` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `accessible-name` | `in` | `string` | `""` | 调用方 | 无可见文字的交互组件必须提供本地化名称 |
| `indicator-color` | `in` | `color` | `Theme.color-primary` | 调用方 | 取值范围、联动和非法值处理见行为规范 |

## 视觉规范

### 组成结构

当前实现由 `Image` 组成。内部元素不是公开 API，调用方不得依赖其名称或坐标。

### 变体、尺寸与状态外观

视觉控制入口：`size`。状态组合以公开属性和行为规范为准，不为 Gallery 虚构额外状态。

### Theme Token

实际消费：`Theme.color-primary`、`Theme.icon-size-large`、`Theme.icon-size-small`、`Theme.icon-size-x-large`、`Theme.reduced-motion`、`Theme.spinner-cycle`。不得在调用方通过任意颜色绕过这些语义 Token。

### 内容、布局与环境

遵循全局排版、密度、长文本、100%–200% 缩放、浅色/深色/高对比度和 LTR/RTL 规则；组件特有差异见本页组成结构及行为规范。

## 行为规范

### 状态与交互

只有 loading 展示状态，不接收指针、键盘或焦点。SVG 按 Token 着色并使用统一动效时钟；reduced-motion 时停止旋转但保留静态图形和语义。

### 无障碍与本地化

角色为 progress-indicator；必须提供说明当前等待对象的本地化名称。装饰性内部图像不重复进入可访问树。

### 验证、宿主职责与限制

Gallery“反馈 / Spinner”页和“通用 / Button”的 Loading 示例覆盖主题与减少动效；通过编译与截图验收。软件渲染器可能不展示旋转变换，但静态回退保持可见，不增加第二套组件。

全局约束：[`foundations.md`](../../foundations.md)、[`interaction.md`](../../interaction.md)、[`accessibility.md`](../../accessibility.md)、[`content-and-localization.md`](../../content-and-localization.md)。
