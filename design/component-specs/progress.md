# Progress

状态：已实现。源码：`ui/feedback/progress.slint`。

- 用途：确定或不确定的线性进度。
- API：`value`（0–1）、`indeterminate`、`accessible-name`。
- 行为：确定值会限制到 0–1；不确定动画跟随统一动效时钟，并暴露 progress-indicator 语义；减少动效时改为居中的静态不确定指示块。
