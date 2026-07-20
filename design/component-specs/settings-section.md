# SettingsSection

状态：已实现。源码：`ui/layout/settings-section.slint`。

## 用途与边界

用于设置页标题、说明和 FormRow 组合。普通卡片内容使用 `Card`；跨分组提交逻辑由页面或后续 `Form` 管理。

## 公开 API

`title: string = ""`、`description: string = ""`、`accessible-name: string = title` 和 `@children`。

## 状态与交互

没有独立交互状态或焦点；子控件按声明顺序参与 Tab。间距、排版和颜色来自 Theme。

## 无障碍与本地化

角色为 `groupbox`，标题作为名称，说明作为描述。调用方负责本地化；标题和说明支持长文本、RTL 和文本缩放。

## Gallery、测试与限制

Gallery“布局 / SettingsSection”页提供单一设置分组示例；错误、选择、开关与多语言分别在对应组件页和全局环境中验证。容器本身通过编译和截图验收。遵循四份全局规范。
