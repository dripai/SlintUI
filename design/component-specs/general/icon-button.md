# IconButton

成熟度：`Alpha`。源码：`crates/slint-ui/ui/controls/icon-button.slint`。公开名称：`IconButton`。

评审状态：`2026-07-21 已完成契约评审`。本页 API 表仍记录当前继承实现；“评审结论”是待落实的目标契约。

用于仅图标的紧凑操作。能稳定显示文字时优先使用 `Button`；工具栏可切换项使用 `ToolButton`。

## 公开 API

### 数据类型与枚举

#### `ButtonVariant`

| 字段或值 | 类型/语义 |
|---|---|
| `default` | `枚举值` |
| `primary` | `枚举值` |
| `danger` | `枚举值` |
| `text` | `枚举值` |
| `link` | `枚举值` |

#### `ControlSize`

| 字段或值 | 类型/语义 |
|---|---|
| `small` | `枚举值` |
| `medium` | `枚举值` |
| `large` | `枚举值` |

### 属性

| 名称 | 方向 | 类型 | 默认值 | 所有者 | 说明与边界 |
|---|---|---|---|---|---|
| `tooltip` | `in` | `string` | `""` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `text` | `in` | `string` | `""` | 调用方 | 程序赋值不伪造用户编辑事件；继承自 Button |
| `icon` | `in` | `image` | `类型默认值` | 调用方 | 取值范围、联动和非法值处理见行为规范；继承自 Button |
| `variant` | `in` | `ButtonVariant` | `ButtonVariant.default` | 调用方 | 取值范围、联动和非法值处理见行为规范；继承自 Button |
| `size` | `in` | `ControlSize` | `ControlSize.medium` | 调用方 | 取值范围、联动和非法值处理见行为规范；继承自 Button |
| `enabled` | `in` | `bool` | `true` | 调用方 | false 时阻止用户操作；程序设置不触发用户事件；继承自 Button |
| `loading` | `in` | `bool` | `false` | 调用方 | true 时显示进行中状态并阻止重复操作；继承自 Button |
| `checkable` | `in` | `bool` | `false` | 调用方 | 取值范围、联动和非法值处理见行为规范；继承自 Button |
| `checked` | `in-out` | `bool` | `false` | 共享受控状态 | 取值范围、联动和非法值处理见行为规范；继承自 Button |
| `keyboard-focusable` | `in` | `bool` | `true` | 调用方 | 取值范围、联动和非法值处理见行为规范；继承自 Button |
| `radius` | `in` | `length` | `Theme.radius-medium` | 调用方 | 取值范围、联动和非法值处理见行为规范；继承自 Button |
| `accessible-name` | `in` | `string` | `text` | 调用方 | 无可见文字的交互组件必须提供本地化名称；继承自 Button |
| `has-focus` | `out` | `bool` | `绑定：focus-scope.has-focus` | 组件派生 | 取值范围、联动和非法值处理见行为规范；继承自 Button |

### 事件

| 名称 | 参数 | 触发语义与边界 |
|---|---|---|
| `clicked` | `无` | 一次被接受的用户激活完成后触发一次；继承自 Button |

### 公开方法

| 名称 | 参数 | 状态、事件与边界 |
|---|---|---|
| `activate()` | `无` | 当前实现约束：`if (interactive) { if (checkable) { root.checked = !root.checked; } root.clicked(); }`；继承自 Button |

### 评审结论（待实现）

- 改为组合 `Button`，不再继承其完整公开表面，避免向 IconButton 泄漏 `text`、`checkable`、`checked` 和 `radius`。
- 目标属性只保留 `icon`、`tooltip`、`variant`、`size`、`enabled`、`loading`、`keyboard-focusable`、`accessible-name`、`has-focus`。
- `accessible-name` 是必填契约。Slint 不能在编译期验证非空，因此由 Gallery、自动测试和宿主接入检查负责；Tooltip 不能替代可访问名称。
- `tooltip` 为空表示不显示可见说明；它不影响点击、焦点或无障碍名称。
- 保留 `clicked()`、`activate()`，新增 `focus()`、`clear-focus()`；事件语义与 Button 完全一致。

目标状态所有权：除 `has-focus` 为组件派生外，其余属性均由调用方输入；IconButton 不拥有切换状态，需要选择状态时使用 `ToolButton` 或 `ToggleButton`。

## 视觉规范

### 组成结构

当前实现由 `HelpTooltip`、`HorizontalLayout`、`Spinner`、`Image`、`Text`、`TouchArea`、`FocusScope`、`FocusRing` 组成。内部元素不是公开 API，调用方不得依赖其名称或坐标。

### 变体、尺寸与状态外观

当前视觉控制入口继承 `variant`、`size`、`loading`、`checked`。目标契约删除 Checked 外观，固定为正方形命中区域并仅显示图标或 Loading 指示。

### Theme Token

实际消费：`Theme.bg-control`、`Theme.bg-disabled`、`Theme.bg-layout`、`Theme.border-default`、`Theme.border-width`、`Theme.color-error`、`Theme.color-error-fill`、`Theme.color-error-fill-active`、`Theme.color-error-fill-hover`、`Theme.color-primary`、`Theme.color-primary-fill`、`Theme.color-primary-fill-active`、`Theme.color-primary-fill-hover`、`Theme.control-height-large`、`Theme.control-height-medium`、`Theme.control-height-small`、`Theme.control-padding-horizontal`、`Theme.fill-hover`、`Theme.fill-pressed`、`Theme.font-size-body`、`Theme.font-weight-medium`、`Theme.font-weight-regular`、`Theme.icon-default`、`Theme.icon-size-small`、`Theme.mode`、`Theme.radius-medium`、`Theme.selection-bg`、`Theme.space-2`、`Theme.text-disabled`、`Theme.text-on-accent`、`Theme.text-primary`。不得在调用方通过任意颜色绕过这些语义 Token。

### 内容、布局与环境

遵循全局排版、密度、长文本、100%–200% 缩放、浅色/深色/高对比度和 LTR/RTL 规则；组件特有差异见本页组成结构及行为规范。

## 行为规范

### 状态与交互

Default、Hover、Pressed、Focused、Disabled、Loading 和激活行为与 Button 一致；Selected 仅在调用方启用 checkable 时适用。

### 已评审事件时序

指针、Enter、Space、无障碍默认动作和 `activate()` 在 `enabled && !loading` 时各触发一次 `clicked()`；Disabled、Loading、宿主赋值、Tooltip 显示与隐藏均不触发业务事件。按下后移出命中区域再释放不触发。

### 已评审方法语义

| 方法 | 前置条件 | 结果与事件 | 幂等与边界 |
|---|---|---|---|
| `activate()` | `enabled && !loading` | 触发一次 `clicked()` | 被阻止时无操作 |
| `focus()` | `enabled && keyboard-focusable` | 获得焦点，不触发业务事件 | 已聚焦时幂等 |
| `clear-focus()` | 当前持有焦点 | 清除焦点 | 未聚焦时幂等 |

当前实现差异：继承导致公开了不适用的 Button 状态；`variant` 实际默认绑定为 `ButtonVariant.text`，与当前 API 表中的继承默认值不一致；焦点方法尚未公开。实现时必须改为组合并同步 Gallery/API 表。

### 无障碍与本地化

`accessible-name` 必填且必须本地化，不能依赖 Tooltip 代替；Tooltip 提供可见补充说明。图标资源由调用方按需导入，不隐式加载完整目录。

### 验证、宿主职责与限制

Gallery“通用 / IconButton”页展示图标按钮；Tooltip 在“数据展示 / Tooltip”独立页验证。自动测试验证 `activate()` 单次触发。Tooltip 焦点触发受 Slint 原生能力限制，重要信息不能只放 Tooltip。

全局约束：[`foundations.md`](../../foundations.md)、[`interaction.md`](../../interaction.md)、[`accessibility.md`](../../accessibility.md)、[`content-and-localization.md`](../../content-and-localization.md)。
