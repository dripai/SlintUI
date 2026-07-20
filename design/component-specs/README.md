# 已实现组件规格说明

本目录中的组件均已实现并由 `@slint-ui/index.slint` 导出。每份规格只记录组件特有 API、行为和限制；颜色、尺寸、状态优先级、键盘、焦点、无障碍、本地化、RTL 和文本缩放统一遵循：

- [`../foundations.md`](../foundations.md)
- [`../interaction.md`](../interaction.md)
- [`../accessibility.md`](../accessibility.md)
- [`../content-and-localization.md`](../content-and-localization.md)

目录包含 26 个 P0、40 个 P1 和 20 个 P2 组件规格，共 86 份。规格记录当前公开 API、行为、原生复用和已知限制；原 58 个组件的逐项质量证据见 [`../component-quality-audit.md`](../component-quality-audit.md)，颜色配对见 [`../contrast-audit.md`](../contrast-audit.md)。

Gallery 的全局环境覆盖浅色、深色、高对比度、三种密度、100%–200% 预览缩放、中文、英文和阿拉伯语 RTL。自动交互测试覆盖全部有状态核心协议的正常、禁用、重复和边界路径；非交互原语由编译、冒烟和代表性 PNG 基线验收。截图位于 `tests/screenshots/`，三平台真机和业务接入仍以 [`../../TODO.md`](../../TODO.md) 为准。

