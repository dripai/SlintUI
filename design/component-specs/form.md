# Form

状态：已实现（P1）

## 用途与边界
聚合字段错误并统一提交门禁；单个字段布局使用 FormRow，业务校验仍由数据层负责。

## 公开 API
`errors: [FormError]`、`enabled`、`submitting`、`error-summary-title`、`valid`；`submitted`、`invalid-submit`、`focus-error-requested`；`submit-form()`。

## 状态与交互
无错误时提交一次；错误时显示摘要并请求聚焦首项；disabled/submitting 阻止重复提交，不清空输入。

## 无障碍与本地化
根为 groupbox，错误摘要为 assertive live region。字段名和修复文案由本地化层提供，不在组件内拼接业务句子。

## Gallery、测试与限制
Gallery 展示字段错误和摘要；Harness 验证无效提交。Slint 无子组件反射，宿主必须把 error index 映射到具体字段焦点。

遵循四份全局规范。
