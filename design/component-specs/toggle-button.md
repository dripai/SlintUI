# ToggleButton

状态：已实现（P1）。源码：`ui/controls/toggle-button.slint`。

## 用途与边界
用于可保持按下状态的独立命令；表单布尔值使用 Checkbox，立即生效的设置使用 Switch。

## 公开 API
继承 Button 的 `text`、`icon`、`variant`、`size`、`enabled`、`loading`、`checked` 和 `clicked()`；`checkable` 固定为 true。

## 状态与交互
覆盖 Default、Hover、Pressed、Focused、Checked、Disabled 和 Loading；每次有效激活切换 checked，禁用和 Loading 阻止回调。

## 无障碍与本地化
使用可检查 button 语义并暴露 checked；无文字时必须提供本地化 `accessible-name`。

## Gallery、测试与限制
Gallery 展示选中和禁用；Harness 覆盖两次切换与禁用。它复用 Button，不提供三态。

遵循四份全局规范。
