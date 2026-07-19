# SegmentedControl

状态：已实现。源码：`ui/controls/segmented-control.slint`。

- 用途：少量互斥视图或模式切换。
- API：`options`、`selected-index`、`enabled`、`size`、`accessible-name`、`selected(index, value)`。
- 行为：只在索引实际变化后触发回调；每项复用 Button 的焦点和激活协议。
- 限制：容器暴露 tab-list 语义，但 Slint 1.17.1 要求可访问角色是编译期常量，复用的 Button 子项仍报告 button；方向键 roving focus 待后续专用子项实现。
