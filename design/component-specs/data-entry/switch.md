# Switch

成熟度：`Alpha`。源码：`crates/slint-ui/ui/controls/switch.slint`。公开名称：`Switch`。

评审状态：`2026-07-21 已完成契约评审`。本页 API 表仍记录当前实现；Loading、事件重命名和焦点方法尚待实现。

用于立即生效的开/关设置。需要提交后才生效或表达多选时使用 Checkbox；互斥模式使用 SegmentedControl。

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
| `text` | `in` | `string` | `""` | 调用方 | 程序赋值不伪造用户编辑事件 |
| `checked` | `in-out` | `bool` | `false` | 共享受控状态 | 取值范围、联动和非法值处理见行为规范 |
| `enabled` | `in` | `bool` | `true` | 调用方 | false 时阻止用户操作；程序设置不触发用户事件 |
| `size` | `in` | `ControlSize` | `ControlSize.medium` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `accessible-name` | `in` | `string` | `text` | 调用方 | 无可见文字的交互组件必须提供本地化名称 |
| `has-focus` | `out` | `bool` | `绑定：focus-scope.has-focus` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |

### 事件

| 名称 | 参数 | 触发语义与边界 |
|---|---|---|
| `toggled` | `checked: bool` | 由该组件定义的有效用户操作或等效公开方法触发；阻止条件见行为规范 |

### 公开方法

| 名称 | 参数 | 状态、事件与边界 |
|---|---|---|
| `toggle()` | `无` | 当前实现约束：`if (enabled) { root.checked = !root.checked; root.toggled(root.checked); }` |

### 评审结论（待实现）

- `checked: in-out bool` 是唯一业务状态；组件和宿主共享所有权。`has-focus` 为派生状态，其他属性由调用方输入。
- 将 `toggled(checked)` 重命名为 `changed(checked)`，统一表示值已经更新；宿主直接设置 `checked` 不触发。
- 新增 `loading: in bool = false`。立即生效的设置经常需要等待宿主确认，Loading 保持当前值和尺寸、保留焦点并阻止重复切换。
- 保留 `toggle()`，新增 `focus()`、`clear-focus()`；指针、Space、无障碍默认动作与方法使用同一路径。
- Enter 不切换 Switch。文字必须描述开启后的状态，不能只写“启用/禁用”而缺少对象。

## 视觉规范

### 组成结构

当前实现由 `HorizontalLayout`、`Rectangle`、`FocusRing`、`Text`、`TouchArea`、`FocusScope` 组成。内部元素不是公开 API，调用方不得依赖其名称或坐标。

### 变体、尺寸与状态外观

视觉控制入口：`checked`、`size`。状态组合以公开属性和行为规范为准，不为 Gallery 虚构额外状态。

### Theme Token

实际消费：`Theme.bg-disabled`、`Theme.bg-surface`、`Theme.border-default`、`Theme.color-primary-fill`、`Theme.color-primary-fill-active`、`Theme.color-primary-fill-hover`、`Theme.control-height-small`、`Theme.fill-hover`、`Theme.fill-pressed`、`Theme.font-size-body`、`Theme.line-height-body`、`Theme.motion-fast`、`Theme.space-2`、`Theme.switch-height-large`、`Theme.switch-height-medium`、`Theme.switch-height-small`、`Theme.switch-thumb-inset`、`Theme.switch-width-large`、`Theme.switch-width-medium`、`Theme.switch-width-small`、`Theme.text-disabled`、`Theme.text-primary`。不得在调用方通过任意颜色绕过这些语义 Token。

### 内容、布局与环境

遵循全局排版、密度、长文本、100%–200% 缩放、浅色/深色/高对比度和 LTR/RTL 规则；组件特有差异见本页组成结构及行为规范。

## 行为规范

### 状态与交互

支持 Default、Hover、Pressed、Focused、Disabled、Checked。点击或 Space 切换，在状态更新后触发一次回调；禁用不变化。滑块动画使用 Theme 动效并服从 reduced-motion。

### 已评审事件时序

| 来源 | 前置条件 | 状态变化 | 事件 |
|---|---|---|---|
| 指针、Space、无障碍默认动作、`toggle()` | `enabled && !loading` | `checked = !checked` | 更新后 `changed(checked)` 一次 |
| 宿主设置 `checked` | 任意 | 同步新值 | 无事件 |
| Disabled 或 Loading 下激活 | 条件不满足 | 无 | 无 |

宿主收到 `changed` 后如需异步保存，应立即设置 `loading = true`；失败回滚通过宿主写回 `checked`，不会再次触发 `changed`，错误反馈由 FormRow、Alert 或 Notification 承担。

### 已评审方法语义

| 方法 | 前置条件 | 结果与事件 | 幂等与边界 |
|---|---|---|---|
| `toggle()` | `enabled && !loading` | 切换并触发 `changed` | 每次有效调用切换一次；被阻止时无操作 |
| `focus()` | `enabled` | 获得焦点 | 已聚焦时幂等；Loading 仍可聚焦 |
| `clear-focus()` | 当前持有焦点 | 清除焦点 | 未聚焦时幂等 |

当前实现差异：回调仍名为 `toggled`，没有 Loading 与焦点方法；动效始终声明 `Theme.motion-fast`，实现阶段必须验证 reduced-motion 下该 Token 实际归零或禁用位置动画。

### 无障碍与本地化

角色为 switch，暴露 enabled、checkable、checked 和默认动作。文字由调用方本地化，支持长文本、RTL 和文本缩放；不能只靠轨道颜色表达值。

### 验证、宿主职责与限制

Gallery“数据录入 / Switch”页展示开关状态；自动测试覆盖正常切换和禁用不变。

全局约束：[`foundations.md`](../../foundations.md)、[`interaction.md`](../../interaction.md)、[`accessibility.md`](../../accessibility.md)、[`content-and-localization.md`](../../content-and-localization.md)。
