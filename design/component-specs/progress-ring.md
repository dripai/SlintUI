# ProgressRing

状态：已实现（P1）。源码：`ui/feedback/progress-ring.slint`。

## 用途与边界
用于紧凑区域的确定或不确定进度；横向空间充足时使用 Progress，极短等待使用 Spinner。

## 公开 API
`value`、`indeterminate`、`show-value`、`accessible-name`。

## 状态与交互
覆盖 0%、分段弧线、100%、Indeterminate、ReducedMotion 和对比主题；组件只读，输入状态不适用。

## 无障碍与本地化
使用 progress-indicator 并暴露 0–1 范围和值；可见百分比仅为视觉辅助，名称必须说明任务。

## Gallery、测试与限制
Gallery 展示 68% 和不确定进度；编译与截图验收。Slint 动态路径能力有限，当前弧线按四分位显示，精确值仍通过文字和无障碍值提供。

遵循四份全局规范。
