# Label

状态：已实现。源码：`ui/primitives/label.slint`。

- 用途：字段标签和普通说明标签。
- API：继承 `Text`；`required`、`muted`。
- 无障碍：可见文本作为名称；`FormRow` 为必填字段显示 `*`，业务校验负责错误关系，Placeholder 不替代 Label。Slint 1.17.1 没有公开 required 可访问状态，平台读屏需要由宿主表单补充字段约束说明。
