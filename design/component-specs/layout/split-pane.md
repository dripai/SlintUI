# SplitPane

成熟度：`Alpha`。源码：`crates/slint-ui/ui/layout/split-pane.slint`。公开名称：`SplitPane`。

提供可调整的水平或垂直双栏几何；固定布局使用 Grid/Flex，不用于任意多栏停靠系统。

## 公开 API

### 数据类型与枚举

#### `SplitOrientation`

| 字段或值 | 类型/语义 |
|---|---|
| `horizontal` | `枚举值` |
| `vertical` | `枚举值` |

### 属性

| 名称 | 方向 | 类型 | 默认值 | 所有者 | 说明与边界 |
|---|---|---|---|---|---|
| `orientation` | `in` | `SplitOrientation` | `SplitOrientation.horizontal` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `minimum-first` | `in` | `length` | `Theme.split-pane-min-size` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `minimum-second` | `in` | `length` | `Theme.split-pane-min-size` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `ratio` | `in-out` | `float` | `0.5` | 共享受控状态 | 取值范围、联动和非法值处理见行为规范 |
| `enabled` | `in` | `bool` | `true` | 调用方 | false 时阻止用户操作；程序设置不触发用户事件 |
| `accessible-name` | `in` | `string` | `""` | 调用方 | 无可见文字的交互组件必须提供本地化名称 |
| `first-x` | `out` | `length` | `0px` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `first-y` | `out` | `length` | `0px` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `first-width` | `out` | `length` | `orientation == SplitOrientation.horizontal ? divider-position : self.width` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `first-height` | `out` | `length` | `orientation == SplitOrientation.vertical ? divider-position : self.height` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `second-x` | `out` | `length` | `orientation == SplitOrientation.horizontal ? divider-position + Theme.split-handle-size : 0px` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `second-y` | `out` | `length` | `orientation == SplitOrientation.vertical ? divider-position + Theme.split-handle-size : 0px` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `second-width` | `out` | `length` | `orientation == SplitOrientation.horizontal ? self.width - second-x : self.width` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `second-height` | `out` | `length` | `orientation == SplitOrientation.vertical ? self.height - second-y : self.height` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `has-focus` | `out` | `bool` | `绑定：focus.has-focus` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |

### 内容入口

`@children` 插入到组件声明的内容区域；尺寸、裁剪和焦点顺序由该区域布局与行为规范约束。

### 事件

| 名称 | 参数 | 触发语义与边界 |
|---|---|---|
| `changed` | `ratio: float` | 组件接受一个不同的新值后触发一次 |

### 公开方法

| 名称 | 参数 | 状态、事件与边界 |
|---|---|---|
| `set-ratio()` | `value: float` | 当前实现约束：`if (enabled) { root.ratio = clamp(value, 0, 1); root.changed(root.ratio); }` |

## 视觉规范

### 组成结构

当前实现由 `Rectangle`、`TouchArea`、`FocusRing`、`FocusScope` 组成。内部元素不是公开 API，调用方不得依赖其名称或坐标。

### 变体、尺寸与状态外观

视觉控制入口：`orientation`。状态组合以公开属性和行为规范为准，不为 Gallery 虚构额外状态。

### Theme Token

实际消费：`Theme.border-default`、`Theme.border-width`、`Theme.color-primary-bg`、`Theme.fill-hover`、`Theme.radius-small`、`Theme.space-12`、`Theme.space-6`、`Theme.split-handle-size`、`Theme.split-pane-min-size`。不得在调用方通过任意颜色绕过这些语义 Token。

### 内容、布局与环境

遵循全局排版、密度、长文本、100%–200% 缩放、浅色/深色/高对比度和 LTR/RTL 规则；组件特有差异见本页组成结构及行为规范。

## 行为规范

### 状态与交互

支持 Hover、Pressed、Focused、Disabled；拖动、方向键、Shift 大步、Home/End 调整，范围按两侧最小尺寸夹取。

### 无障碍与本地化

使用 slider 作为 Slint 可表达的最近角色，暴露方向、值、范围和步长。无内置文案，RTL 不反转数值含义。

### 验证、宿主职责与限制

Gallery 展示 Tree/Table 双栏；Harness 验证比例回调。稳定 Slint 缺少两个公开命名插槽，因此子区域必须绑定公开几何属性。

全局约束：[`foundations.md`](../../foundations.md)、[`interaction.md`](../../interaction.md)、[`accessibility.md`](../../accessibility.md)、[`content-and-localization.md`](../../content-and-localization.md)。
