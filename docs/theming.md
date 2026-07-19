# 主题与全局环境

`Theme` 是每个顶层窗口中的全局环境，公开：

```text
mode: light | dark | high-contrast
density: compact | regular | comfortable
direction: ltr | rtl
locale: BCP 47 string
text-scale: float
preview-scale: float
reduced-motion: bool
```

组件只读取语义 Token，不根据主题名称分支。宿主应在创建每个顶层窗口后一次性同步这些属性；Slint global 不会跨窗口共享实例。

高对比度当前使用组件库固定语义色。跟随 Windows、macOS、Linux 系统高对比度、文本缩放与减少动效需要后续平台增强层。

