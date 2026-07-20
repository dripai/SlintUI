# FormRow

状态：已实现。源码：`ui/layout/form-row.slint`。

## 用途与边界

用于统一字段 Label、输入控件、帮助和错误信息的垂直关系。跨字段提交、校验聚合和错误焦点定位属于后续 `Form`。

## 公开 API

`label: string = ""`、`help-text: string = ""`、`error-text: string = ""`、`required: bool = false`、`accessible-name: string = label` 和 `@children`。

## 状态与交互

FormRow 本身不接收输入；错误文本优先于帮助文本，错误不会清空或修正子控件值。高度随密度、文本缩放和反馈内容增长。

## 无障碍与本地化

角色为 `groupbox`，名称来自 label，描述优先使用错误、其次帮助文本。业务必须提供已本地化文案；支持长文本和 RTL。`required` 视觉星号不替代业务校验或读屏说明。

## Gallery、测试与限制

Gallery“数据录入 / FormRow”页展示标签、帮助文本和输入控件组合；错误与长文本由组件测试和全局环境验证。遵循四份全局规范。
