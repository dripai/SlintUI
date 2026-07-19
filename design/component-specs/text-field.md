# TextField

状态：已实现。源码：`ui/controls/text-field.slint`。

- 用途：单行文本、密码和只读输入。
- API：`text`、`placeholder-text`、`prefix-icon`、`suffix-icon`、`clearable`、`clear-accessible-name`、`enabled`、`read-only`、`input-type`、`size`、`validation`、`edited`、`accepted`、`cleared`。
- 原生复用：使用 Slint `TextInput` 的 IME、选择、剪贴板、撤销、密码和键盘行为，自定义层只处理 Token 外观与清除入口。
- 无障碍：可见 Label 由 `FormRow` 提供；启用清除时调用方必须提供本地化的 `clear-accessible-name`。

