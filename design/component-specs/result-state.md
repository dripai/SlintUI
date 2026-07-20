# ResultState

状态：已实现（P1）。源码：`ui/feedback/result-state.slint`。

## 用途与边界
用于任务完成后的成功、信息、警告或失败总结和后续操作；页面内短消息使用 Alert，空集合使用 EmptyState。

## 公开 API
`tone`、`title`、`description`、`accessible-name` 和操作区 `@children`。

## 状态与交互
覆盖四种 tone、长标题/说明、无说明和自定义操作；自身只读，操作状态由子组件负责。

## 无障碍与本地化
使用 region，标题作为默认名称；颜色同时有图标和文字语义，全部文案由宿主本地化。

## Gallery、测试与限制
Gallery 展示成功结果和主操作；编译与截图验收。组件不内建重试、导航或业务结果码。

遵循四份全局规范。
