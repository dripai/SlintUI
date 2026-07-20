# Radio

成熟度：`Alpha`。源码：`crates/slint-ui/ui/controls/radio.slint`。公开名称：`Radio`。

评审状态：`2026-07-21 已完成契约评审`。本页 API 表记录当前请求式实现；目标事件命名和键盘规则尚待落实。

用于互斥选项中的单个可选项。多个 Radio 必须由 RadioGroup 或宿主共享一个选择状态；独立布尔值应使用 Checkbox。

## 公开 API

### 属性

| 名称 | 方向 | 类型 | 默认值 | 所有者 | 说明与边界 |
|---|---|---|---|---|---|
| `text` | `in` | `string` | `""` | 调用方 | 程序赋值不伪造用户编辑事件 |
| `value` | `in` | `string` | `""` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `checked` | `in` | `bool` | `false` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `enabled` | `in` | `bool` | `true` | 调用方 | false 时阻止用户操作；程序设置不触发用户事件 |
| `accessible-name` | `in` | `string` | `text` | 调用方 | 无可见文字的交互组件必须提供本地化名称 |
| `has-focus` | `out` | `bool` | `绑定：focus.has-focus` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |

### 事件

| 名称 | 参数 | 触发语义与边界 |
|---|---|---|
| `selected` | `value: string` | 用户选择有效且可用项目后触发一次 |

### 公开方法

| 名称 | 参数 | 状态、事件与边界 |
|---|---|---|
| `select()` | `无` | 当前实现约束：`if (enabled && !checked) { root.selected(value); }` |

### 评审结论（待实现）

- 单个 `Radio` 保持请求式状态：`checked` 是只读输入，由 `RadioGroup` 或宿主管理；组件不得自行改写。
- 将 `selected(value)` 重命名为 `selection-requested(value)`，准确表示组件只提出选择请求，尚未代表宿主已经更新组状态。
- `value` 是稳定业务值，`text` 是本地化显示文本；同一组内 `value` 必须唯一，空值和重复值由 RadioGroup 的模型校验拒绝。
- `select()` 在 `enabled && !checked` 时发出一次请求；已选中项重复激活不发请求。
- 新增 `focus()`、`clear-focus()`。单个 Radio 只响应指针和 Space；Enter 不选择。方向键、Home、End 和 roving focus 由 RadioGroup 负责。

## 视觉规范

### 组成结构

当前实现由 `HorizontalLayout`、`Rectangle`、`FocusRing`、`Text`、`TouchArea`、`FocusScope` 组成。内部元素不是公开 API，调用方不得依赖其名称或坐标。

### 变体、尺寸与状态外观

视觉控制入口：`checked`。状态组合以公开属性和行为规范为准，不为 Gallery 虚构额外状态。

### Theme Token

实际消费：`Theme.bg-control`、`Theme.bg-disabled`、`Theme.border-default`、`Theme.border-width`、`Theme.color-primary`、`Theme.control-height-small`、`Theme.font-size-body`、`Theme.icon-size-small`、`Theme.line-height-body`、`Theme.space-1`、`Theme.space-2`、`Theme.text-disabled`、`Theme.text-primary`。不得在调用方通过任意颜色绕过这些语义 Token。

### 内容、布局与环境

遵循全局排版、密度、长文本、100%–200% 缩放、浅色/深色/高对比度和 LTR/RTL 规则；组件特有差异见本页组成结构及行为规范。

## 行为规范

### 状态与交互

Unchecked、Checked、Hover、Focused 和 Disabled 由 checked、enabled 与焦点状态共同决定。select() 只在可用且尚未选中时发出 selected(value)，组件自身不修改 checked。

### 已评审事件时序

| 来源 | 前置条件 | 状态变化 | 事件 |
|---|---|---|---|
| 指针、Space、无障碍默认动作、`select()` | `enabled && !checked` | 无；等待宿主更新 `checked` | `selection-requested(value)` 一次 |
| 已选中项重复激活 | `checked == true` | 无 | 无 |
| 宿主设置 `checked` | 任意 | 同步视觉与语义状态 | 无事件 |
| Disabled 下激活 | `enabled == false` | 无 | 无 |

RadioGroup 接受请求后才更新组的 `current-index`，随后所有 Radio 根据组状态重绘；该同步不应再次发出请求。

### 已评审方法语义

| 方法 | 前置条件 | 结果与事件 | 幂等与边界 |
|---|---|---|---|
| `select()` | `enabled && !checked` | 发出 `selection-requested(value)`，不修改属性 | 已选中、禁用时无操作 |
| `focus()` | `enabled` | 获得焦点 | 已聚焦时幂等；组内通常由 RadioGroup 调用 |
| `clear-focus()` | 当前持有焦点 | 清除焦点 | 未聚焦时幂等 |

当前实现差异：事件仍名为 `selected`，Space 和 Enter 都会激活，且没有公开焦点方法。Slint 1.17.1 已提供原生 RadioGroup 的单一 `current-index` 与组内选择模型，可参考其状态所有权，但 SlintUI 仍需保留稳定业务 `value` 和禁用项规则。

### 无障碍与本地化

使用 radio-button 角色，暴露名称、可用和选中状态；可见文字与稳定 value 分离，二者均由宿主本地化和维护。

### 验证、宿主职责与限制

方向键组内移动由 RadioGroup 的选择协议承担；单个 Radio 只处理指针、Space 和 Enter 激活。

全局约束：[`foundations.md`](../../foundations.md)、[`interaction.md`](../../interaction.md)、[`accessibility.md`](../../accessibility.md)、[`content-and-localization.md`](../../content-and-localization.md)。
