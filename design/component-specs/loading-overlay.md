# LoadingOverlay

状态：已实现（P1）。源码：`ui/feedback/loading-overlay.slint`。

## 用途与边界
用于明确阻塞一个区域的加载任务；非阻塞加载使用 Spinner/Skeleton，整窗模态任务由 Dialog 承担。

## 公开 API
`active`、`text`、`cancelable`、`cancel-text`、`accessible-name`；`cancel()`、`cancel-requested()`。

## 状态与交互
覆盖 Inactive、Active、Cancelable、ReducedMotion、长文本和禁用式遮罩；仅 active 且 cancelable 时回调。

## 无障碍与本地化
使用 region 和 polite live region，遮罩拦截区域输入；加载说明和取消文字必须本地化。

## Gallery、测试与限制
Gallery 展示区域遮罩、Spinner 和取消；Harness 覆盖 active 与 inactive 边界。取消只发请求，宿主负责真正终止任务和关闭遮罩。

遵循四份全局规范。
