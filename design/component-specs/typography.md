# Typography

状态：已实现。源码：`ui/primitives/typography.slint`。

- 用途：统一 caption、body、label 和三级标题字阶。
- API：继承 `Text`；增加 `style: TypographyStyle`。
- 行为：字号、字重、最小行高和 RTL 对齐全部来自 `Theme`；支持换行和文本缩放。

