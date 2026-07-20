# Tag

状态：已实现（P1）。源码：`ui/data-display/tag.slint`。

## 用途与边界
用于短标签、状态、可选择筛选或可关闭条目；计数使用 Badge，复杂对象使用 Card/ListItem。

## 公开 API
`text`、`tone`、`closable`、`selectable`、`selected`、`enabled`、`accessible-name`；`toggle()/close()`、`toggled()`、`close-requested()`。

## 状态与交互
覆盖 Default、Hover、Focused、Selected、Closable、Disabled 和五种 tone；禁用阻止切换/关闭，每次有效切换显式回调。

## 无障碍与本地化
使用可检查 button 语义；关闭按钮单独可访问。标签必须简短，本地化长文本应改用其他组件。

## Gallery、测试与限制
Gallery 展示 tone、选择、关闭和禁用；Harness 覆盖重复切换、关闭与禁用。关闭只发请求，不直接修改外部模型。

遵循四份全局规范。
