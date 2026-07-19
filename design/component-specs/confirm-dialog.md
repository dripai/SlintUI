# ConfirmDialog

状态：已实现（P1）

## 用途与边界
在不可撤销或高风险操作前确认对象和影响；可撤销操作优先直接执行并提供撤销。

## 公开 API
继承 ModalDialog 全部 API，新增 `dangerous: bool = false` 并映射危险主按钮。

## 状态与交互
普通与危险确认、busy、Esc 和取消行为继承 ModalDialog；业务必须使用明确的动词和对象。

## 无障碍与本地化
继承 Dialog 名称、描述和焦点要求；危险状态不能只靠红色，标题和正文必须说明后果。

## Gallery、测试与限制
与 ModalDialog 共用顶层编译和宿主说明。原生 owner/modal、默认安全按钮和关闭确认流程由宿主集成层完成。

遵循四份全局规范。
