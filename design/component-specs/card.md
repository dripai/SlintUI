# Card

状态：已实现。源码：`ui/layout/card.slint`。

## 用途与边界

用于组织标题、说明和内容区域。无结构的背景使用 `Surface`；可折叠内容使用后续 `Collapse`，不要让 Card 承担交互容器职责。

## 公开 API

`title: string = ""`、`description: string = ""` 和 `@children`；空标题或说明不占布局。

## 状态、交互与无障碍

没有 Hover、Pressed、Focused、Selected、Disabled 或 Loading 状态，不接收输入。表面、边框、圆角、内边距和排版来自 Theme。

## 本地化与无障碍

标题和说明由调用方本地化，支持长文本、RTL 和文本缩放。Card 本身不声明可访问角色；需要区域名称时由页面语义容器提供，避免无意义嵌套分组。

## Gallery、测试与限制

Gallery 所有内容页使用 Card，覆盖主题、密度、缩放和 RTL；非交互组件通过编译和代表性截图验收。遵循四份全局规范。
