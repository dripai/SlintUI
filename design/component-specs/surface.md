# Surface

状态：已实现。源码：`ui/primitives/surface.slint`。

## 用途与边界

用于表达 layout、surface、elevated、control 四类语义表面。结构化标题内容使用 `Card`；平台原生窗口阴影不由 Surface 模拟。

## 公开 API

`variant: SurfaceVariant = surface`、`bordered: bool = false`、`radius: length = Theme.radius-medium`、`content-padding: length = 0px` 和 `@children`。

## 状态与交互

没有交互状态、键盘行为或焦点节点。背景、边框和圆角随 Theme 更新，子内容区域扣除统一内边距。

## 无障碍、本地化、Gallery 与测试

Surface 本身不提供语义或文案；需要分组语义时由 Card、Toolbar 或调用方容器声明。Gallery“基础”和“布局”页覆盖明暗、高对比度和密度；非交互组件通过编译与截图验收。

## 原生能力与限制

Slint 核心没有通用模糊阴影，层级通过背景和边框表达。遵循 [`../foundations.md`](../foundations.md) 与 [`../accessibility.md`](../accessibility.md)。
