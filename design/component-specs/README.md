# 已实现组件规格说明

本目录中的组件均已实现并由 `@slint-ui/index.slint` 导出。每份规格只记录组件特有 API、行为和限制；颜色、尺寸、状态优先级、键盘、焦点、无障碍、本地化、RTL 和文本缩放统一遵循：

- [`../foundations.md`](../foundations.md)
- [`../interaction.md`](../interaction.md)
- [`../accessibility.md`](../accessibility.md)
- [`../content-and-localization.md`](../content-and-localization.md)

目录包含 26 个 P0 组件、提前完成的 `ScrollArea`、里程碑 D 已完成的 11 份 P1 规格，以及里程碑 E 已完成的 20 份 P2 规格，共 58 份。规格记录当前公开 API、行为、原生复用和已知限制；逐组件补齐全部适用状态、组件特有 Gallery API/Token 说明及测试证据的工作统一记录在 [`../../TODO.md`](../../TODO.md)，不能以“已经导出”代替完成定义。

Gallery 的全局环境覆盖浅色、深色、高对比度、三种密度、100%–200% 预览缩放、中文、英文和阿拉伯语 RTL。当前自动交互测试覆盖 P0 控件、里程碑 D 交互，以及 P2 的操作、导航、选择、日期时间边界、DataGrid 宿主窗口请求和 Calendar 选择；非交互原语由编译、冒烟和代表性 PNG 基线验收。截图位于 `tests/screenshots/`，尚未覆盖的逐组件证据以 TODO 为准。

