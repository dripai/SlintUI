# Tooltip

状态：已实现。源码：`ui/feedback/tooltip.slint`，公开名为 `Tooltip`。

- 用途：为非自解释控件提供简短补充说明。
- API：`content`；作为目标控件子元素使用。
- 原生复用：包装 Slint 1.17.1 原生 Tooltip 定位、延迟和隐藏行为。
- 限制：1.17.1 未公开延迟/偏移配置，也不会因任意自定义 FocusScope 自动显示；重要信息不能只放 Tooltip。

