# Icon

状态：已实现。源码：`ui/primitives/icon.slint`、`ui/icons/`。

## 用途与边界

用于单色操作、状态和对象图标。图标按钮使用 `IconButton`；纯装饰图像直接使用 `Image { accessible-role: none; }`。不支持需要两种独立 Theme 颜色的 Two Tone 图标。

## 公开 API

- 继承 `Image` 的 `source`。
- `size: IconSize = medium`，映射 x-small/small/medium/large 到 12/16/20/24px。
- `color: color = Theme.icon-default`、`accessible-name: string = ""`。
- 资源包含 447 个 outlined、249 个 filled；每个图标有独立模块和聚合入口。

## 状态、交互与无障碍

Icon 不接收输入或焦点，没有控件状态。固定为 `image` 角色，信息图标必须提供已本地化名称；颜色只来自 Theme 语义 Token。

## Gallery、测试与限制

Gallery“基础”和“图标”页覆盖尺寸、主题和完整目录；自动测试核对 696 个资源与模块一一对应。方向图标不自动镜像，由具体导航组件按语义选择。详见 [`../../docs/iconography.md`](../../docs/iconography.md) 及四份全局规范。
