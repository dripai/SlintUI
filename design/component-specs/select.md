# Select

状态：已实现。源码：`ui/controls/select.slint`。

- 用途：从有限字符串模型中单选。
- API：继承 Slint `ComboBox` 的 `model`、`current-index`、`current-value`、`enabled`、`selected`；增加 `accessible-name`、`size`。
- 原生复用：保留原生 Popup、键盘、焦点和关闭策略。
- 限制：Slint 1.17.1 `ComboBox` 不支持搜索，且视觉样式在编译期选择；可搜索 Select 未伪造回退，保持待实现。

