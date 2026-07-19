# SegmentedControl

状态：已实现。源码：`ui/controls/segmented-control.slint`。

## 用途与边界

用于少量互斥视图或模式切换。大量选项使用 Select，独立布尔设置使用 Switch；不用于步骤流程。

## 公开 API

`options: [string] = []`、`selected-index: int = 0`、`enabled: bool = true`、`size: ControlSize = medium`、`accessible-name: string = ""`、`selected(index, value)`、`select(index)`。

## 状态与交互

容器支持 Enabled/Disabled，子项复用 Button 的 Default、Hover、Pressed、Focused 和 Checked。点击或 `select(index)` 只在启用、索引有效且实际变化时更新并回调；重复和越界选择无操作。

## 无障碍与本地化

容器角色为 tab-list，暴露名称、enabled 和项目数；Slint 1.17.1 要求角色是编译期常量，复用 Button 子项仍报告 button。选项由调用方本地化，RTL 下业务顺序不自动反转。

## Gallery、测试与限制

Gallery 顶栏和“输入/图标”页覆盖主题、密度、缩放和目录切换；自动测试覆盖有效选择、回调、重复及越界。方向键 roving focus 待专用子项接口，不增加备用实现。遵循四份全局规范。
