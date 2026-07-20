# TextField

状态：已实现。源码：`ui/controls/text-field.slint`。

## 用途与边界

用于单行文本、密码和只读输入。多行使用后续 TextArea，格式化数字使用后续 NumberInput；Label 和错误关系由 `FormRow` 提供。

## 公开 API

- `text: string`、`placeholder-text: string = ""`、`prefix-icon/suffix-icon: image`、`clearable: bool = false`、`clear-accessible-name: string = ""`。
- `enabled: bool = true`、`read-only: bool = false`、`input-type: InputType = text`、`size: ControlSize = medium`、`validation: ValidationState = normal`、只读 `has-focus`。
- `edited(text)`、`accepted(text)`、`cleared()`、`clear()`；validation 为 normal/success/warning/error。

## 状态与交互

支持 Default、Focused、Disabled、ReadOnly、四种校验状态和有值清除入口；不适用 Selected/Loading。输入、IME、选择、剪贴板、撤销、密码和 Enter 接受由原生 TextInput 处理。`clear()` 与清除按钮走同一路径，禁用、只读、不可清除或空值时不触发。

## 无障碍与本地化

角色为 text-input，暴露名称、enabled、read-only、placeholder 和 value。Placeholder 不替代 Label；清除入口出现时必须提供本地化名称。组件不内置业务错误文案。

## Gallery、测试与限制

Gallery“数据录入 / TextField”页覆盖普通、错误和禁用状态；RTL 和高对比度由全局环境验证。自动测试覆盖清除、edited/cleared 单次回调、重复清除和禁用不变。清除图标使用 `close.svg`。遵循四份全局规范。
