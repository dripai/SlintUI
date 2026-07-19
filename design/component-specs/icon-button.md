# IconButton

状态：已实现。源码：`ui/controls/icon-button.slint`。

- 用途：仅图标的紧凑操作。
- API：继承 `Button`；增加 `tooltip`，宽高保持方形。
- 无障碍：`accessible-name` 必填且不依赖 Tooltip；Tooltip 用于可见说明。
- 资源：按钮只接收调用方按需导入的 `image`，不隐式绑定图标名称，也不加载完整图标目录。
