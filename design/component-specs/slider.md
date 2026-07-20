# Slider

状态：已实现（P1）。源码：`ui/controls/slider.slint`。

## 用途与边界
用于连续或步进范围的单值调整；精确整数录入使用 NumberInput，多值范围等待真实产品协议。

## 公开 API
继承原生 Slider 的 `value`、`minimum`、`maximum`、`step`、`orientation`、`enabled`、`changed/released`；新增 `accessible-name`、`set-value()`。

## 状态与交互
覆盖 Default、Hover、Pressed、Focused、Disabled、最小/最大值、步长和水平/垂直；程序化值夹取到范围且同值不重复回调。

## 无障碍与本地化
复用原生 slider 角色、范围、步长与增减动作；值格式和单位由外部 Label 或宿主格式化。

## Gallery、测试与限制
Gallery 展示正常和禁用；Harness 覆盖重复、越界和禁用。视觉使用 Slint 原生控件，颜色受当前原生样式能力约束。

遵循四份全局规范。
