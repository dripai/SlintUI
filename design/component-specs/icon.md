# Icon

状态：已实现。源码：`ui/primitives/icon.slint`。

- 用途：统一 12/16/20/24px 图标尺寸、颜色和替代名称。
- API：继承 `Image`；`size`、`color`、`accessible-name`。
- 限制：Slint 要求 `accessible-role` 为常量，因此 `Icon` 固定为信息图像；纯装饰图像使用原生 `Image { accessible-role: none; }`。

