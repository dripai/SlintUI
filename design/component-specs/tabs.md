# Tabs

状态：已实现（P1）

## 用途与边界
切换同一工作区中的并列视图；少量模式切换使用 SegmentedControl，不用于页面层级导航。

## 公开 API
`tabs: [TabItem]`、`current-index: int`、`accessible-name: string`；`selected(index)`、`close-requested(index)`；`select(index)`、`select-next(step)`。

## 状态与交互
覆盖默认、Hover、Pressed、Selected、Focused、Disabled、Closable。左右键按 RTL 视觉方向切换，Home/End 到首尾；同值与禁用项不回调。

## 无障碍与本地化
根和条目使用 tab 角色、名称和可用状态。Slint 1.17.1 未公开 tab selected 语义属性，视觉选中和 current-index 仍可读取。

## Gallery、测试与限制
Gallery“导航 / Tabs”页覆盖可关闭、选中和禁用项；Harness 验证状态边界。当前不自动跳过相邻禁用项，也不实现超宽标签溢出菜单。

遵循四份全局规范。
