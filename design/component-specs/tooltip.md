# Tooltip

状态：已实现。源码：`ui/feedback/tooltip.slint`，公开名为 `Tooltip`。

## 用途与边界

用于为非自解释控件提供简短补充说明。重要错误、操作结果或必须阅读的信息必须直接显示，不能只放 Tooltip。

## 公开 API

`content: string = ""`；作为目标控件的子元素使用。没有业务默认文案或全局队列。

## 状态与交互

悬停定位、显示、隐藏和关闭由 Slint 原生 Tooltip 处理；组件没有 Selected、Disabled、Loading 或错误状态，也不拦截目标控件输入。

## 无障碍与本地化

目标控件仍必须具有独立可访问名称，Tooltip 不能替代名称。内容由调用方本地化并保持简短；复杂 Markdown、交互内容和焦点移动不属于 Tooltip。

## Gallery、测试与限制

Gallery“数据展示 / Tooltip”页提供独立悬停目标；原生悬停在软件渲染冒烟中编译验证。Slint 1.17.1 未公开延迟/偏移，也不能让任意 FocusScope 自动触发，因此焦点触发保持明确限制。遵循四份全局规范。
