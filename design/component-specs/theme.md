# Theme

状态：已实现。源码：`ui/tokens/theme.slint`。

- 用途：为每个顶层窗口提供主题、密度、Locale、Direction、文本/预览缩放和减少动效环境，以及完整语义 Token。
- API：`mode`、`density`、`direction`、`locale`、`text-scale`、`preview-scale`、`reduced-motion`；其余属性只读。
- 限制：Slint global 不跨窗口共享，宿主必须逐窗口同步；系统高对比度与减少动效自动探测仍需平台增强层。

