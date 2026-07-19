# ScrollArea

状态：已实现（提前完成的 P1 基础能力）。源码：`ui/primitives/scroll-area.slint`。

## 用途与边界

用于普通内容滚动。超长数据集合必须使用后续虚拟化 `List`/`Table`，不能用 ScrollArea 实例化全部数据后作为替代。

## 公开 API

`viewport-width: length`、`viewport-height: length`、`enabled: bool`、`mouse-drag-pan-enabled: bool`、只读 `has-focus: bool` 和 `@children`；默认值继承 Slint `ScrollView`。

## 状态与交互

滚轮、滚动条、键盘滚动、焦点和禁用行为直接继承标准 `ScrollView`。组件不添加第二套滚动状态或回退路径。

## 无障碍、本地化、Gallery 与测试

内容语义和文案由子项提供，滚动不改变阅读顺序；方向与平台滚动行为由 Slint 处理。Gallery“布局”页和主内容区覆盖滚动环境，编译与软件渲染冒烟验证创建路径。

## 原生能力与限制

会实例化全部子元素，不提供虚拟化。遵循 [`../interaction.md`](../interaction.md)、[`../accessibility.md`](../accessibility.md) 和 [`../content-and-localization.md`](../content-and-localization.md)。
