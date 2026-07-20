# Toolbar

成熟度：`Alpha`。源码：`crates/slint-ui/ui/layout/toolbar.slint`。公开名称：`Toolbar`。

用于组合 `ToolButton`、`IconButton` 和垂直 `Divider`。复杂溢出菜单和任意子项方向键导航不属于当前 P0。

## 公开 API

### 数据类型与枚举

#### `SurfaceVariant`

| 字段或值 | 类型/语义 |
|---|---|
| `layout` | `枚举值` |
| `surface` | `枚举值` |
| `elevated` | `枚举值` |
| `control` | `枚举值` |

### 属性

| 名称 | 方向 | 类型 | 默认值 | 所有者 | 说明与边界 |
|---|---|---|---|---|---|
| `accessible-name` | `in` | `string` | `"Toolbar"` | 调用方 | 无可见文字的交互组件必须提供本地化名称 |
| `variant` | `in` | `SurfaceVariant` | `SurfaceVariant.surface` | 调用方 | 取值范围、联动和非法值处理见行为规范；继承自 Surface |
| `bordered` | `in` | `bool` | `false` | 调用方 | 取值范围、联动和非法值处理见行为规范；继承自 Surface |
| `radius` | `in` | `length` | `Theme.radius-medium` | 调用方 | 取值范围、联动和非法值处理见行为规范；继承自 Surface |
| `content-padding` | `in` | `length` | `0px` | 调用方 | 取值范围、联动和非法值处理见行为规范；继承自 Surface |

### 内容入口

`@children` 插入到组件声明的内容区域；尺寸、裁剪和焦点顺序由该区域布局与行为规范约束。

## 视觉规范

### 组成结构

当前实现由 `HorizontalLayout`、`Rectangle` 组成。内部元素不是公开 API，调用方不得依赖其名称或坐标。

### 变体、尺寸与状态外观

视觉控制入口：`variant`、`bordered`。状态组合以公开属性和行为规范为准，不为 Gallery 虚构额外状态。

### Theme Token

实际消费：`Theme.bg-control`、`Theme.bg-elevated`、`Theme.bg-layout`、`Theme.bg-surface`、`Theme.border-default`、`Theme.border-width`、`Theme.radius-medium`、`Theme.row-height`、`Theme.space-1`、`Theme.space-2`。不得在调用方通过任意颜色绕过这些语义 Token。

### 内容、布局与环境

遵循全局排版、密度、长文本、100%–200% 缩放、浅色/深色/高对比度和 LTR/RTL 规则；组件特有差异见本页组成结构及行为规范。

## 行为规范

### 状态与交互

Toolbar 不截获子控件事件。当前焦点顺序是声明顺序的 Tab/Shift+Tab；按钮自身处理 Enter/Space。分组和分隔由子项显式表达。

### 无障碍与本地化

角色为 `groupbox`，调用方必须提供已本地化的可访问名称。无文字按钮仍需名称和 Tooltip；RTL 下业务顺序由调用方明确编排。

### 验证、宿主职责与限制

Gallery“布局 / Toolbar”页展示 IconButton、ToolButton 和 Divider 组合；自动测试覆盖按钮激活与 checked 转换。Slint 稳定 API 无法枚举任意 `@children` 实现 roving focus 和溢出，相关能力保持待实现。

全局约束：[`foundations.md`](../../foundations.md)、[`interaction.md`](../../interaction.md)、[`accessibility.md`](../../accessibility.md)、[`content-and-localization.md`](../../content-and-localization.md)。
