# Typography

状态：已实现。源码：`ui/primitives/typography.slint`。

## 用途与边界

用于 caption、body、label 和三级标题。字段名称使用 `Label`，可交互链接使用后续 `Link`，不要用 Typography 模拟控件。

## 公开 API

- 继承 `Text` 的 `text`、`wrap`、`overflow` 等公开属性。
- `style: TypographyStyle = body`；枚举值为 `caption`、`body`、`label`、`title-small`、`title-medium`、`title-large`。

## 状态与交互

没有 Hover、Pressed、Selected、Disabled 或 Loading 状态，不接收指针和键盘焦点。字号、字重、行高、颜色和 RTL 对齐由 Theme 决定。

## 无障碍与本地化

可见文本由原生 `Text` 暴露。支持换行、长文本、文本缩放和 RTL；业务文案由调用方本地化，组件不内置字符串。

## Gallery、测试与限制

Gallery“基础”页展示全部字阶、长中文/英文/阿拉伯语和缩放环境；非交互组件无需状态转换测试，由编译与截图基线验收。全局规则见 [`../foundations.md`](../foundations.md)、[`../accessibility.md`](../accessibility.md) 和 [`../content-and-localization.md`](../content-and-localization.md)。
