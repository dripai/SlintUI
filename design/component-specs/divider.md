# Divider

状态：已实现。源码：`ui/primitives/divider.slint`。

## 用途与边界

用于相邻内容的视觉分隔，不用于表达标题层级、可点击拖动条或 SplitPane。

## 公开 API

`orientation: DividerOrientation = horizontal`、`subtle: bool = true`；方向枚举为 `horizontal`、`vertical`。

## 状态、交互与无障碍

没有交互状态、指针、键盘、焦点或可访问节点；分隔不改变阅读顺序。厚度与颜色来自 Theme，高对比度下随边框 Token 增强。

## Gallery、测试与限制

Gallery“布局 / Divider”页集中展示水平和垂直组合；非交互组件无需状态转换测试，由编译和截图验收。本组件不提供文字标签，完整规则见 [`../foundations.md`](../foundations.md) 和 [`../accessibility.md`](../accessibility.md)。
