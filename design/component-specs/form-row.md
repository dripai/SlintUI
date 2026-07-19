# FormRow

状态：已实现。源码：`ui/layout/form-row.slint`。

- 用途：统一 Label、输入控件、帮助和错误信息的垂直关系。
- API：`label`、`help-text`、`error-text`、`required`、`accessible-name` 和子控件。
- 行为：错误优先于帮助文本，错误不清空输入；高度随密度、文本缩放和反馈内容增长。

