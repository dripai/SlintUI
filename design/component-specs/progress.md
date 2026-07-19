# Progress

状态：已实现。源码：`ui/feedback/progress.slint`。

## 用途与边界

用于确定或不确定的线性进度。紧凑圆形进度使用后续 ProgressRing，短时未知等待可使用 Spinner；Progress 不负责异步任务生命周期。

## 公开 API

`value: float = 0.0`、`indeterminate: bool = false`、`accessible-name: string = ""`。确定值显示时限制到 0–1，输入值本身不被静默改写。

## 状态与交互

有 determined/indeterminate 两种显示状态，不接收指针、键盘或焦点。动效使用统一时钟；reduced-motion 下不确定状态变为居中静态指示块。

## 无障碍与本地化

角色为 progress-indicator，暴露 0–100 范围和确定进度百分比；不确定时不伪造数值。调用方提供本地化名称和相邻说明，不能只靠颜色表达状态。

## Gallery 与测试

Gallery“反馈”页覆盖确定、不确定、150% 缩放和主题环境；数值归一化、减少动效与语义通过编译和截图验证。非交互组件无需回调测试。遵循四份全局规范。
