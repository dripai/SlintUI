# Checkbox

状态：已实现。源码：`ui/controls/checkbox.slint`。

## 用途与边界

用于独立或列表中的选择状态。立即生效的开关使用 `Switch`，互斥选择使用 `SegmentedControl` 或后续 RadioGroup。

## 公开 API

`text: string = ""`、`state: CheckState = unchecked`、`enabled: bool = true`、`accessible-name: string = text`、`toggled(state)`、`toggle()`；状态为 unchecked/checked/indeterminate。

## 状态与交互

支持 Hover、Focused、Disabled 和三态视觉；不适用 Loading/错误。点击或 Space 调用 `toggle()`，用户切换从 indeterminate/unchecked 进入 checked、从 checked 进入 unchecked，并在更新后触发一次回调；禁用不变化。

## 无障碍与本地化

角色为 checkbox，暴露 enabled、checkable、checked 和默认动作。checked/indeterminate 使用 `check.svg`/`minus.svg`，不依赖字体。Slint 1.17.1 只能公开布尔 checked，mixed 读屏状态仍受上游限制。

## Gallery 与测试

Gallery“数据录入 / Checkbox”页覆盖 checked 和多语言；indeterminate、disabled 由交互 Harness 与组件规格验证。完整规则见四份全局规范。
