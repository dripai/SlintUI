# ScrollArea

状态：已实现（按本次任务提前完成的 P1 基础能力）。源码：`ui/primitives/scroll-area.slint`。

- 用途：统一普通内容滚动区域。
- API：`viewport-width`、`viewport-height`、`enabled`、`mouse-drag-pan-enabled`、`has-focus` 和子内容。
- 原生复用：直接包装 Slint `ScrollView`，保留原生滚动条与滚轮行为。
- 限制：会实例化全部子元素；长列表应使用后续 `List`，不能用本组件替代虚拟化。

