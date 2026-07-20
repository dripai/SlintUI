# Dropdown

状态：已实现（P1）。源码：`ui/navigation/dropdown.slint`。

## 用途与边界
用于按钮触发的一组菜单命令；单值表单选择使用 Select，可编辑建议使用 ComboBox。

## 公开 API
`text`、`entries`、`enabled`、`size`、`accessible-name`、`open`；`toggle()`、`activate()`、`selected()`、`closed()`。

## 状态与交互
覆盖 Closed、Open、Hover、Pressed、Focused、Disabled、禁用菜单项、点击外部和 Escape；越界或禁用项不回调。

## 无障碍与本地化
触发器暴露 expandable/expanded，菜单复用 PopupMenu 的项语义与键盘行为。快捷键文字由宿主本地化。

## Gallery、测试与限制
Gallery“导航 / Dropdown”展示正常、勾选和禁用项；Harness 覆盖正常、禁用和越界。弹层定位与关闭策略复用 Slint PopupWindow。

遵循四份全局规范。
