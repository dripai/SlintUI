# FocusRing

状态：已实现。源码：`ui/primitives/focus-ring.slint`。

## 用途与边界

为自定义控件绘制统一键盘焦点环。它只负责视觉，不获取焦点；实际焦点必须由 `FocusScope` 或原生控件管理。

## 公开 API

`active: bool = false`、`radius: length = Theme.radius-medium`、`target-width: length = 0px`、`target-height: length = 0px`。

## 状态与交互

仅有 active/inactive 视觉状态，不接收指针或键盘。焦点环位于目标外侧，不改变布局尺寸；颜色、宽度和偏移来自 Theme。

## 无障碍、Gallery、测试与限制

FocusRing 不进入可访问树；可访问焦点仍由目标控件声明。Gallery“基础”页展示 active 状态，高对比度截图验证可见性；非交互原语无需单独回调测试。遵循 [`../interaction.md`](../interaction.md) 与 [`../accessibility.md`](../accessibility.md)。
