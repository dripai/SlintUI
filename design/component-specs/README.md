# 已实现组件规格说明

本目录中的组件均已实现并由 `@slint-ui/index.slint` 导出。每份规格只记录组件特有 API、行为和限制；颜色、尺寸、状态优先级、键盘、焦点、无障碍、本地化、RTL 和文本缩放统一遵循：

- [`../foundations.md`](../foundations.md)
- [`../interaction.md`](../interaction.md)
- [`../accessibility.md`](../accessibility.md)
- [`../content-and-localization.md`](../content-and-localization.md)

目录包含 26 个 P0 组件以及提前完成的 P1 `ScrollArea`。每份规格明确用途与替代组件、API 类型和默认值、适用状态、输入与焦点、无障碍、本地化、Gallery/测试证据、原生复用和限制；不适用项显式说明，不以省略代替验收。

Gallery 的全局环境覆盖浅色、深色、高对比度、三种密度、100%–200% 预览缩放、中文、英文和阿拉伯语 RTL。自动交互测试覆盖 Button、IconButton、ToolButton、Checkbox、Switch、SegmentedControl、TextField 清除和 Select 程序化选择；非交互原语由编译、冒烟和代表性 PNG 基线验收。截图位于 `tests/screenshots/`。

