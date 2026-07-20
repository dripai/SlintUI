# Steps

状态：已实现（P2）

## 用途与边界
展示有顺序的多步任务进度，不用于自由导航或进度百分比。

## 公开 API
`steps: [StepItem]`、`current-index`、`accessible-name`；`selected(index)`；`select(index)`；`StepState`。

## 状态与交互
覆盖 waiting、current、complete、error、Hover、Disabled；完成态使用 SVG check。

## 无障碍与本地化
根为 list，步骤暴露标题、说明和可用状态；文案由调用方本地化。

## Gallery、测试与限制
Gallery 展示完成、当前、禁用；Harness 验证禁用和同值边界。当前为横向布局，不自动切换纵向或滚动到当前步骤。

遵循四份全局规范。
