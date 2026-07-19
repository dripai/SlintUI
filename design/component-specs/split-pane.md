# SplitPane

状态：已实现（P1）

## 用途与边界
提供可调整的水平或垂直双栏几何；固定布局使用 Grid/Flex，不用于任意多栏停靠系统。

## 公开 API
`orientation`、`minimum-first/second`、`ratio`、`enabled`、`accessible-name`；两侧 x/y/width/height 输出；`changed(ratio)`、`set-ratio(value)`。

## 状态与交互
支持 Hover、Pressed、Focused、Disabled；拖动、方向键、Shift 大步、Home/End 调整，范围按两侧最小尺寸夹取。

## 无障碍与本地化
使用 slider 作为 Slint 可表达的最近角色，暴露方向、值、范围和步长。无内置文案，RTL 不反转数值含义。

## Gallery、测试与限制
Gallery 展示 Tree/Table 双栏；Harness 验证比例回调。稳定 Slint 缺少两个公开命名插槽，因此子区域必须绑定公开几何属性。

遵循四份全局规范。
