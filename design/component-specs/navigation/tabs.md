# Tabs

成熟度：`Alpha`。源码：`crates/slint-ui/ui/navigation/tabs.slint`。公开名称：`Tabs`。

切换同一工作区中的并列视图；少量模式切换使用 SegmentedControl，不用于页面层级导航。

## 公开 API

### 数据类型与枚举

#### `TabItem`

| 字段或值 | 类型/语义 |
|---|---|
| `text` | `string` |
| `accessible-name` | `string` |
| `enabled` | `bool` |
| `closable` | `bool` |

### 属性

| 名称 | 方向 | 类型 | 默认值 | 所有者 | 说明与边界 |
|---|---|---|---|---|---|
| `tabs` | `in` | `[TabItem]` | `类型默认值` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `current-index` | `in-out` | `int` | `tabs.length > 0 ? 0 : -1` | 共享受控状态 | -1 表示无选择；越界值的处理见行为规范 |
| `accessible-name` | `in` | `string` | `""` | 调用方 | 无可见文字的交互组件必须提供本地化名称 |
| `has-focus` | `out` | `bool` | `绑定：focus.has-focus` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |

### 内容入口

`@children` 插入到组件声明的内容区域；尺寸、裁剪和焦点顺序由该区域布局与行为规范约束。

### 事件

| 名称 | 参数 | 触发语义与边界 |
|---|---|---|
| `selected` | `index: int` | 用户选择有效且可用项目后触发一次 |
| `close-requested` | `index: int` | 请求宿主执行操作，不表示操作已经成功 |

### 公开方法

| 名称 | 参数 | 状态、事件与边界 |
|---|---|---|
| `select()` | `index: int` | 当前实现约束：`if (index >= 0 && index < tabs.length && tabs[index].enabled && current-index != index) { root.current-index = index; root.selected(index); }` |
| `select-next()` | `step: int` | 当前实现约束：`if (tabs.length <= 0) { return; } root.select(Math.mod(current-index + step + tabs.length, tabs.length));` |

## 视觉规范

### 组成结构

当前实现由 `VerticalLayout`、`Rectangle`、`HorizontalLayout`、`TouchArea`、`Text`、`Image`、`FocusScope`、`FocusRing` 组成。内部元素不是公开 API，调用方不得依赖其名称或坐标。

### 变体、尺寸与状态外观

视觉控制入口：由内容和全局 Theme 决定。状态组合以公开属性和行为规范为准，不为 Gallery 虚构额外状态。

### Theme Token

实际消费：`Theme.border-secondary`、`Theme.border-width`、`Theme.color-primary`、`Theme.control-height-medium`、`Theme.control-padding-horizontal`、`Theme.direction`、`Theme.fill-hover`、`Theme.fill-pressed`、`Theme.font-size-body`、`Theme.font-weight-medium`、`Theme.font-weight-regular`、`Theme.icon-default`、`Theme.icon-size-medium`、`Theme.icon-size-x-small`、`Theme.radius-medium`、`Theme.radius-small`、`Theme.selection-bg`、`Theme.space-1`、`Theme.tab-height`、`Theme.text-disabled`、`Theme.text-primary`。不得在调用方通过任意颜色绕过这些语义 Token。

### 内容、布局与环境

遵循全局排版、密度、长文本、100%–200% 缩放、浅色/深色/高对比度和 LTR/RTL 规则；组件特有差异见本页组成结构及行为规范。

## 行为规范

### 状态与交互

覆盖默认、Hover、Pressed、Selected、Focused、Disabled、Closable。左右键按 RTL 视觉方向切换，Home/End 到首尾；同值与禁用项不回调。

### 无障碍与本地化

根和条目使用 tab 角色、名称和可用状态。Slint 1.17.1 未公开 tab selected 语义属性，视觉选中和 current-index 仍可读取。

### 验证、宿主职责与限制

Gallery“导航 / Tabs”页覆盖可关闭、选中和禁用项；Harness 验证状态边界。当前不自动跳过相邻禁用项，也不实现超宽标签溢出菜单。

全局约束：[`foundations.md`](../../foundations.md)、[`interaction.md`](../../interaction.md)、[`accessibility.md`](../../accessibility.md)、[`content-and-localization.md`](../../content-and-localization.md)。
