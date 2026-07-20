# SegmentedControl

成熟度：`Alpha`。源码：`crates/slint-ui/ui/controls/segmented-control.slint`。公开名称：`SegmentedControl`。

用于少量互斥视图或模式切换。大量选项使用 Select，独立布尔设置使用 Switch；不用于步骤流程。

## 公开 API

### 数据类型与枚举

#### `ControlSize`

| 字段或值 | 类型/语义 |
|---|---|
| `small` | `枚举值` |
| `medium` | `枚举值` |
| `large` | `枚举值` |

### 属性

| 名称 | 方向 | 类型 | 默认值 | 所有者 | 说明与边界 |
|---|---|---|---|---|---|
| `options` | `in` | `[string]` | `[]` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `selected-index` | `in-out` | `int` | `0` | 共享受控状态 | 取值范围、联动和非法值处理见行为规范 |
| `enabled` | `in` | `bool` | `true` | 调用方 | false 时阻止用户操作；程序设置不触发用户事件 |
| `size` | `in` | `ControlSize` | `ControlSize.medium` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `accessible-name` | `in` | `string` | `""` | 调用方 | 无可见文字的交互组件必须提供本地化名称 |

### 事件

| 名称 | 参数 | 触发语义与边界 |
|---|---|---|
| `selected` | `index: int, value: string` | 用户选择有效且可用项目后触发一次 |

### 公开方法

| 名称 | 参数 | 状态、事件与边界 |
|---|---|---|
| `select()` | `index: int` | 当前实现约束：`if (enabled && index >= 0 && index < options.length && selected-index != index) { root.selected-index = index; root.selected(index, root.options[index]); }` |

## 视觉规范

### 组成结构

当前实现由 `HorizontalLayout`、`Button` 组成。内部元素不是公开 API，调用方不得依赖其名称或坐标。

### 变体、尺寸与状态外观

视觉控制入口：`selected-index`、`size`。状态组合以公开属性和行为规范为准，不为 Gallery 虚构额外状态。

### Theme Token

实际消费：`Theme.border-width`、`Theme.control-height-large`、`Theme.control-height-medium`、`Theme.control-height-small`、`Theme.fill-hover`、`Theme.radius-medium`。不得在调用方通过任意颜色绕过这些语义 Token。

### 内容、布局与环境

遵循全局排版、密度、长文本、100%–200% 缩放、浅色/深色/高对比度和 LTR/RTL 规则；组件特有差异见本页组成结构及行为规范。

## 行为规范

### 状态与交互

容器支持 Enabled/Disabled，子项复用 Button 的 Default、Hover、Pressed、Focused 和 Checked。点击或 `select(index)` 只在启用、索引有效且实际变化时更新并回调；重复和越界选择无操作。

### 无障碍与本地化

容器角色为 tab-list，暴露名称、enabled 和项目数；Slint 1.17.1 要求角色是编译期常量，复用 Button 子项仍报告 button。选项由调用方本地化，RTL 下业务顺序不自动反转。

### 验证、宿主职责与限制

Gallery 顶栏和“输入/图标”页覆盖主题、密度、缩放和目录切换；自动测试覆盖有效选择、回调、重复及越界。方向键 roving focus 待专用子项接口，不增加备用实现。

全局约束：[`foundations.md`](../../foundations.md)、[`interaction.md`](../../interaction.md)、[`accessibility.md`](../../accessibility.md)、[`content-and-localization.md`](../../content-and-localization.md)。
