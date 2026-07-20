# Select

成熟度：`Alpha`。源码：`crates/slint-ui/ui/controls/select.slint`。公开名称：`Select`。

评审状态：`2026-07-21 已完成契约评审`。当前继承 `StandardComboBox` 的 API 仅视为临时 Alpha 实现；结构化模型和完整浮层契约尚待实现。

用于从有限结构化模型中选择一个业务值。可编辑候选使用 `ComboBox`，多选使用 `MultiSelect`，带搜索建议的自由输入使用 `AutoComplete`。

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
| `accessible-name` | `in` | `string` | `""` | 调用方 | 无可见文字的交互组件必须提供本地化名称 |
| `size` | `in` | `ControlSize` | `ControlSize.medium` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `model` | `in` | `[string]` | `[]` | 调用方 | 取值范围、联动和非法值处理见行为规范；继承自 Slint StandardComboBox |
| `enabled` | `in` | `bool` | `true` | 调用方 | false 时阻止用户操作；程序设置不触发用户事件；继承自 Slint StandardComboBox |
| `current-index` | `in-out` | `int` | `-1` | 共享受控状态 | -1 表示无选择；越界值的处理见行为规范；继承自 Slint StandardComboBox |
| `current-value` | `out` | `string` | `""` | 组件派生 | 取值范围、联动和非法值处理见行为规范；继承自 Slint StandardComboBox |
| `has-focus` | `out` | `bool` | `false` | 组件派生 | 取值范围、联动和非法值处理见行为规范；继承自 Slint StandardComboBox |

### 事件

| 名称 | 参数 | 触发语义与边界 |
|---|---|---|
| `selected` | `value: string` | 用户选择有效且可用项目后触发一次；继承自 Slint StandardComboBox |

### 公开方法

| 名称 | 参数 | 状态、事件与边界 |
|---|---|---|
| `select-index()` | `index: int` | 当前实现约束：`if (self.enabled && index >= 0 && index < self.model.length && self.current-index != index) { root.current-index = index; root.selected(root.model[index]); }` |

### 评审结论（待实现）

当前 `[string]` 模型无法区分稳定标识、业务值、显示文本和禁用项，评审不通过。目标模型固定为：

```slint
export struct SelectOption {
    id: string,
    value: string,
    label: string,
    description: string,
    icon: image,
    enabled: bool,
}

export enum SelectCloseReason {
    selection,
    escape,
    outside,
    method,
    disabled,
    owner-invalidated,
}
```

目标状态所有权：

| 名称 | 方向 | 所有者 | 目标语义 |
|---|---|---|---|
| `model` | `in [SelectOption]` | 调用方 | 只读模型；`id` 必须非空且唯一 |
| `current-index` | `in-out int = -1` | 共享受控状态 | `-1` 表示无选择；有效值必须指向 enabled 项 |
| `current-id` | `out string` | 组件派生 | 当前项稳定 ID，无选择时为空 |
| `current-value` | `out string` | 组件派生 | 当前业务值，无选择时为空 |
| `display-text` | `out string` | 组件派生 | 当前 label，无选择时为空 |
| `open` | `out bool` | 组件派生 | 浮层实际打开状态，不允许宿主直接伪造 |
| `model-valid` | `out bool` | 组件派生 | ID 非空且唯一时为 true |
| `has-focus` | `out bool` | 组件派生 | 触发器或所属 Popup 持有焦点时为 true |

模型更新时按原选中项 `id` 尝试保持选择并更新 `current-index`；原 ID 不存在、变为禁用、模型为空或模型无效时统一变为 `-1`，不静默选择第一项，也不触发用户事件。重复 ID 或空 ID 使 `model-valid == false`，组件禁止打开和选择。

目标事件：`selected(index, value)`、`cleared()`、`opened()`、`closed(reason)`。目标方法：`open()`、`close()`、`toggle()`、`select(index)`、`clear()`、`focus()`、`clear-focus()`。

## 视觉规范

### 组成结构

当前视觉由编译期选择的 Slint StandardComboBox 样式承担。目标实现改为 Theme 驱动的组合组件，包含触发器、当前值、方向图标、Popup、选项行、选中标记和滚动区；内部元素不公开。

### 变体、尺寸与状态外观

目标视觉控制入口为 `size`、`enabled`、`open`、`current-index` 和模型项状态，覆盖 Empty、No selection、Selected、Hover、Pressed、Focused、Disabled、Open、长文本和禁用项。

### Theme Token

实际消费：`Theme.control-height-large`、`Theme.control-height-medium`、`Theme.control-height-small`、`Theme.control-min-width`。不得在调用方通过任意颜色绕过这些语义 Token。

### 内容、布局与环境

遵循全局排版、密度、长文本、100%–200% 缩放、浅色/深色/高对比度和 LTR/RTL 规则；组件特有差异见本页组成结构及行为规范。

## 行为规范

### 状态与交互

Default、Hover、Pressed、Focused、Disabled、展开、当前选择和空模型行为由原生 ComboBox 提供。点击打开 Popup，方向键选择，Enter 打开/确认，Esc 关闭；`select-index()` 仅在启用、索引有效且发生变化时更新并回调。

### 已评审事件时序

| 来源 | 前置条件 | 状态变化 | 事件顺序 |
|---|---|---|---|
| 指针或 Enter/Space 打开 | `enabled && model-valid && model.length > 0 && !open` | Popup 实际打开并取得焦点 | `opened()` 一次 |
| 用户选择或 `select(index)` | 索引有效、项 enabled、与当前项不同 | 更新 `current-index` 和派生值，关闭 Popup | `selected(index, value)`，随后 `closed(selection)`（原先已打开时） |
| 用户选择当前项 | 与当前项相同 | 可关闭 Popup，不改变值 | 不触发 `selected`；已打开时只触发 `closed(selection)` |
| Escape | `open` | 关闭，不改变选择 | `closed(escape)` |
| 点击外部 | `open` | 关闭，不改变选择 | `closed(outside)` |
| `clear()` | 当前存在有效选择 | `current-index = -1` | `cleared()` 一次；不伪造 `selected` |
| 宿主设置 `current-index` | 任意 | 规范化后同步派生值 | 无用户事件 |
| 空、无效模型，越界或禁用项 | 条件不满足 | 不选择 | 无成功事件 |

方向键只移动 Popup 内部活动项，Enter/Space 才提交选择；Escape 恢复打开前选择。关闭后焦点返回触发器。RTL 下左右布局和方向图标按逻辑方向处理，但上下选择顺序不反转。

### 已评审方法语义

| 方法 | 前置条件 | 结果与事件 | 幂等与边界 |
|---|---|---|---|
| `open()` | `enabled && model-valid && model.length > 0` | 实际打开后触发 `opened()` | 已打开、空或无效模型时无操作 |
| `close()` | `open` | 关闭并触发 `closed(method)` | 已关闭时无操作 |
| `toggle()` | 满足对应 open/close 条件 | 打开或按 method 原因关闭 | 状态不变时无事件 |
| `select(index)` | 有效、enabled、非当前项 | 更新选择并触发 `selected` | 相同、越界、禁用或无效模型时不改变选择 |
| `clear()` | `enabled && current-index != -1` | 清除并触发 `cleared()` | 无选择或禁用时无操作 |
| `focus()` | `enabled` | 聚焦触发器 | 已聚焦时幂等 |
| `clear-focus()` | 当前持有焦点且 Popup 已关闭 | 清除焦点 | Popup 打开时先由 `close()` 明确关闭 |

当前实现差异：Slint 1.17.1 `ComboBoxBase` 默认 `current-index = 0`，模型变更会 Clamp 到第一项，公开的 `current-value` 写入不会改变选择，同一索引调用原生 `select()` 仍会发出 `selected`。这些行为与目标的 `-1`、稳定 ID、相同值无事件和禁用项规则冲突，因此不能继续通过继承补丁收敛，实施时必须采用最小组合组件，并记录 Popup 关闭原因和焦点恢复的上游边界。

### 无障碍与本地化

保留 combobox 的 enabled、expanded、value 和展开动作，补充可访问名称。选项由调用方本地化；RTL、字体、Popup 关闭与焦点恢复沿用原生实现。

### 验证、宿主职责与限制

Gallery“数据录入 / Select”页和全局 Locale 入口分别展示业务选择与语言选择；自动测试覆盖程序化选择、重复/越界和禁用不变。Slint 1.17.1 不支持搜索，且标准控件样式在编译期选择，不能完全跟随运行时 Theme。

全局约束：[`foundations.md`](../../foundations.md)、[`interaction.md`](../../interaction.md)、[`accessibility.md`](../../accessibility.md)、[`content-and-localization.md`](../../content-and-localization.md)。
