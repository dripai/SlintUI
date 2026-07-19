# Stack

状态：已实现。源码：`ui/layout/stack.slint`。

## 用途与边界

用于按统一间距垂直排列少量内容。水平排列使用 `Space`；需要虚拟化或可滚动数据时使用对应容器，不把 Stack 当列表控件。

## 公开 API

继承 `VerticalLayout`；`gap: length = Theme.space-2` 和 `@children`。对齐默认 start，交叉轴默认 stretch。

## 状态、交互与无障碍

没有组件状态、指针、键盘或焦点行为；子项保留各自交互和阅读顺序。Stack 不创建额外可访问分组。

## 本地化、Gallery、测试与限制

布局不内置文案，RTL 文本由子项处理。Gallery 各页面均使用 Stack，并覆盖密度和缩放。Slint 1.17.1 稳定布局不能运行时切换 `@children` 方向，未采用实验性 Flexbox。遵循四份全局规范。
