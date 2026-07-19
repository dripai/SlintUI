# Space

状态：已实现。源码：`ui/layout/space.slint`。

## 用途与边界

用于按统一间距水平排列少量内容。垂直排列使用 `Stack`；需要自动换行时由调用方依据窗口尺寸等级切换结构。

## 公开 API

继承 `HorizontalLayout`；`gap: length = Theme.space-2` 和 `@children`，交叉轴默认 center。

## 状态、交互与无障碍

没有组件状态、输入或焦点行为；子项保持各自语义和 Tab 顺序。Space 不自动改变 RTL 下的业务顺序。

## Gallery、测试与限制

Gallery 顶栏、基础、布局、操作和反馈页均使用 Space；通过主题、密度、Locale 和截图验证。稳定 `HorizontalLayout` 不支持自动换行。遵循 [`../foundations.md`](../foundations.md)、[`../interaction.md`](../interaction.md) 和本地化规范。
