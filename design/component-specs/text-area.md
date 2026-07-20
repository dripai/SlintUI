# TextArea

状态：已实现（P1）。源码：`ui/controls/text-area.slint`。

## 用途与边界
用于多行自由文本；单行输入使用 TextField，搜索使用 SearchField。

## 公开 API
`text`、`placeholder-text`、`enabled`、`read-only`、`max-length`、`show-count`、`validation`、`character-count`；`replace-text()`、`edited()`。

## 状态与交互
覆盖 Default、Focused、Populated、Empty、Disabled、ReadOnly、Success、Warning、Error、滚动和长文本。`max-length` 是显式计数/校验上限，不静默截断输入。

## 无障碍与本地化
使用 text-input 角色并暴露值、只读和计数说明；换行、IME、RTL 和文本缩放由 Slint TextEdit 处理。

## Gallery、测试与限制
Gallery 展示长文本计数和禁用；Harness 覆盖正常、重复和禁用更新。调整尺寸由外部布局控制，不提供拖拽手柄。

遵循四份全局规范。
