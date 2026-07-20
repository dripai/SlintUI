# Flex

状态：已实现（P1）。源码：`ui/layout/flex.slint`。

## 用途与边界
用于水平的增长、收缩、主轴与交叉轴对齐；垂直排列使用 Stack，需要显式二维单元格时使用 Grid。

## 公开 API
`gap`，并继承 HorizontalLayout 的 `alignment`、`cross-axis-alignment`、padding 与子项 stretch/min/max 约束。

## 状态与交互
布局原语不接收输入；验证默认间距、不同 stretch、长文本、密度、RTL 与缩放。Disabled、Loading、Selected 和 Error 不适用。

## 无障碍与本地化
不增加语义层，子项自行声明名称和焦点。RTL 只影响子项内容，不自动反转任意 `@children` 顺序。

## Gallery、测试与限制
Gallery“布局 / Flex”展示 1:2 增长比例；编译与截图覆盖布局。Slint 稳定 API 不支持运行时换行或方向切换。

遵循四份全局规范。
