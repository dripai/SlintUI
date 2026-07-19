# Toolbar

状态：已实现。源码：`ui/layout/toolbar.slint`。

- 用途：组合 ToolButton、IconButton 和 Divider。
- API：`accessible-name` 和子内容。
- 限制：Slint 稳定 API 无法枚举任意 `@children` 并执行 roving focus，当前子控件按 Tab 顺序访问；方向键复合导航待 Slint 提供稳定子项焦点接口后实现。

