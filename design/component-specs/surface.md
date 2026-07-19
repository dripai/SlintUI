# Surface

状态：已实现。源码：`ui/primitives/surface.slint`。

- 用途：提供 layout、surface、elevated、control 四类语义表面。
- API：`variant`、`bordered`、`radius`、`content-padding` 和子内容。
- 限制：Slint 核心没有通用模糊阴影；层级由背景与边框表达，原生窗口阴影属于平台增强层。

