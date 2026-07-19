# Stack

状态：已实现。源码：`ui/layout/stack.slint`。

- 用途：按统一 gap 垂直排列内容。
- API：`gap` 和子内容。
- 限制：Slint 1.17.1 的稳定布局不能在运行时切换 `@children` 方向；实验性 `FlexboxLayout` 未采用。水平排列使用 `Space`。

