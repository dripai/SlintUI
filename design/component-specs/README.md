# P0 组件规格说明

本目录中的组件均已实现并由 `@slint-ui/index.slint` 导出。每份规格只记录组件特有 API、行为和限制；颜色、尺寸、状态优先级、键盘、焦点、无障碍、本地化、RTL 和文本缩放统一遵循：

- [`../foundations.md`](../foundations.md)
- [`../interaction.md`](../interaction.md)
- [`../accessibility.md`](../accessibility.md)
- [`../content-and-localization.md`](../content-and-localization.md)

所有组件在 Gallery 中覆盖浅色、深色、高对比度、三种密度、100%–200% 预览缩放、中文、英文和阿拉伯语 RTL 环境。自动化测试覆盖公开状态转换；PNG 基线位于 `tests/screenshots/`。

