# Icon

状态：已实现。源码：`ui/primitives/icon.slint`。

- 用途：统一 12/16/20/24px 图标尺寸、颜色和替代名称。
- API：继承 `Image`；`size`、`color`、`accessible-name`。
- 资源：447 个 outlined、249 个 filled；每个图标有独立模块和聚合入口，完整清单见 [`../../docs/iconography.md`](../../docs/iconography.md)。
- 行为：默认通过 `Theme.icon-default` 单色着色；outlined 用于普通界面，filled 只用于选中、强状态或品牌图形。
- 限制：Slint 要求 `accessible-role` 为常量，因此 `Icon` 固定为信息图像；纯装饰图像使用原生 `Image { accessible-role: none; }`。
